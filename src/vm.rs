use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::num::NonZeroUsize;
use std::ops::{Index, IndexMut, Range};
use crate::bytecode::chunk::{Chunk, Value};
use crate::bytecode::opcode::{NumeralType, OpCode, OpCodeError};
use crate::bytecode::values::*;
use crate::vm::VMError::UnknownOpCode;


#[derive(Clone, Debug)]
pub enum VMError {
    CompileError((u16, u16), String),
    RuntimeError(String),
    UnexpectedEoF,
    JITError((u16, u16)),
    UnknownOpCode(OpCodeError),
}

impl Display for VMError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{:#?}", self))
    }
}

impl Error for VMError {}



pub struct Stack<const SIZE: usize> {
    stack: [u8; SIZE],
    sp: usize
}

impl<const SIZE: usize> Stack<SIZE> {
    pub fn new() -> Self {
        Stack {
            stack: [0; SIZE],
            sp: 0
        }
    }

    pub fn is_empty(&self) -> bool {
        self.sp == 0
    }

    pub fn push_value<const N: usize, Val: Value<N>>(&mut self, val: Val) {
        self.push(&val.to_bits());
    }

    pub fn push(&mut self, src: &[u8]) {
        self.stack[self.sp..(self.sp + src.len())].copy_from_slice(src);
        self.sp += src.len();
    }

    pub fn pop_to(&mut self, dst: &mut [u8]) {
        dst.copy_from_slice(&self.stack[(self.sp - dst.len())..self.sp]);
        self.sp -= dst.len();
    }

    pub fn pop_value<const N: usize, Val: Value<N>>(&mut self) -> Val {
        let mut bits = [0u8; N];
        self.pop_to(&mut bits);
        Val::from_bits(bits)
    }

    pub fn pop(&mut self, size: usize) {
        self.sp -= size;
    }

    pub fn len(&self) -> usize {
        self.sp
    }

    pub fn cap(&self) -> usize {
        SIZE
    }
}

impl<const SIZE: usize> Debug for Stack<SIZE> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("[ ")?;
        for i in 0..self.sp {
            f.write_str(&format!("{:02x}", self.stack[i]))?;
            if i + 1 < self.sp {
                f.write_str(", ")?;
            }
        }
        f.write_str(" ]")
    }
}

impl<const SIZE: usize> Index<usize> for Stack<SIZE> {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.stack[index]
    }
}

impl<const SIZE: usize> IndexMut<usize> for Stack<SIZE> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.stack[index]
    }
}

impl<const SIZE: usize> Index<Range<usize>> for Stack<SIZE> {
    type Output = [u8];

    fn index(&self, index: Range<usize>) -> &Self::Output {
        &self.stack[index]
    }
}

impl<const SIZE: usize> IndexMut<Range<usize>> for Stack<SIZE> {
    fn index_mut(&mut self, index: Range<usize>) -> &mut Self::Output {
        &mut self.stack[index]
    }
}



macro_rules! impl_binop(
    ($self:expr, $T:ty, $op:expr) => {
        {
            let b: $T = $self.stack.pop_value();
            let a: $T = $self.stack.pop_value();
            $self.stack.push_value($op(a, b));
        }
    };
);


pub struct VM {
    /// stack
    stack: Stack<256>, // limit stack for now
    /// instruction pointer
    ip: usize,
    /// frame pointer
    fp: usize,

    pub exit_code: i32,
    pub is_active: bool,

    /// main chunk of program memory
    chunk: Chunk,

    use_jit: bool,
}

impl VM {
    pub fn new(chunk: Chunk) -> Self {
        VM {
            stack: Stack::new(),
            ip: 0,
            fp: 0,

            exit_code: 0,
            is_active: true,
            chunk,

            use_jit: false,
        }
    }

    /// Returns the next byte in program memory and increments the instruction pointer.
    fn fetch(&mut self) -> Result<OpCode, VMError> {
        let out = OpCode::try_from((self.ip, &self.chunk))
            .map_err(|e| UnknownOpCode(e))?;
        self.ip += out.size();
        Ok(out)
    }

    /// Returns the next byte in program memory without incrementing the instruction counter.
    fn peak(&mut self) -> u8 {
        self.chunk.code()[self.ip]
    }

    /// Executes a single CPU cycle
    pub fn cycle(&mut self) -> Result<(), VMError> {
        match self.fetch()? {
            OpCode::Const(i, s) => {
                self.stack.push(&self.chunk.vals()[(i as usize)..(i as usize + s as usize)])
            },
            OpCode::Ret => {
                // TODO temporary
                if self.stack.len() <= 8 {
                    let test: f64 = self.stack.pop_value();
                    println!("stack as f64: {test}");

                    self.exit_code = self.stack[0] as i32;
                    self.is_active = false;
                    return Ok(())
                }
            },


            OpCode::Neg(NumeralType::I8) => {
                let val: i8 = self.stack.pop_value();
                self.stack.push_value(-val);
            },
            OpCode::Neg(NumeralType::F64) => {
                let val: f64 = self.stack.pop_value();
                self.stack.push_value(-val);
            }

            OpCode::Add(NumeralType::F64) => impl_binop!(self, f64, |a, b| a + b),
            OpCode::Sub(NumeralType::F64) => impl_binop!(self, f64, |a, b| a - b),
            OpCode::Mul(NumeralType::F64) => impl_binop!(self, f64, |a, b| a * b),
            OpCode::Div(NumeralType::F64) => impl_binop!(self, f64, |a, b| a / b),

            _ => {}
        }

        Ok(())
    }
}

impl Debug for VM {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.chunk.disassemble(self.ip, 1, f)?;
        f.write_str(&format!("      {:?}", self.stack))
    }
}
