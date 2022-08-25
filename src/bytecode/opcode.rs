use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Write};
use std::io::BufRead;
use crate::bytecode::chunk::Chunk;
use crate::bytecode::opcode::OpCodeError::IllegalNumeralType;

#[derive(Clone, Debug)]
pub enum OpCodeError {
    IllegalOpcode(u8),
    IllegalNumeralType(u8),
}


#[derive(Clone, Debug)]
pub enum NumeralType {
    I8,
    I16,
    I32,
    I64,
    I128,

    U8,
    U16,
    U32,
    U64,
    U128,

    F32,
    F64,
}


impl NumeralType {
    pub fn size(&self) -> usize {
        match self {
            NumeralType::I8 => 1,
            NumeralType::I16 => 2,
            NumeralType::I32 => 4,
            NumeralType::I64 => 8,
            NumeralType::I128 => 16,
            NumeralType::U8 => 1,
            NumeralType::U16 => 2,
            NumeralType::U32 => 4,
            NumeralType::U64 => 8,
            NumeralType::U128 => 16,
            NumeralType::F32 => 4,
            NumeralType::F64 => 8,
        }
    }
}

impl TryFrom<u8> for NumeralType {
    type Error = OpCodeError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(NumeralType::I8),
            1 => Ok(NumeralType::I16),
            2 => Ok(NumeralType::I32),
            3 => Ok(NumeralType::I64),
            4 => Ok(NumeralType::I128),

            5 => Ok(NumeralType::U8),
            6 => Ok(NumeralType::U16),
            7 => Ok(NumeralType::U32),
            8 => Ok(NumeralType::U64),
            9 => Ok(NumeralType::U128),

            10 => Ok(NumeralType::F32),
            11 => Ok(NumeralType::F64),
            v => Err(IllegalNumeralType(v))
        }
    }
}

impl Into<u8> for NumeralType {
    fn into(self) -> u8 {
        match self {
            NumeralType::I8 => 0,
            NumeralType::I16 => 1,
            NumeralType::I32 => 2,
            NumeralType::I64 => 3,
            NumeralType::I128 => 4,
            NumeralType::U8 => 5,
            NumeralType::U16 => 6,
            NumeralType::U32 => 7,
            NumeralType::U64 => 8,
            NumeralType::U128 => 9,
            NumeralType::F32 => 10,
            NumeralType::F64 => 11,
        }
    }
}


impl Display for OpCodeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IllegalOpcode(c) => f.write_str(&format!("There is no opcode with id {}", c)),
            Self::IllegalNumeralType(n) => f.write_str(&format!("There is no numeral type with id {}", n)),
        }
    }
}

impl Error for OpCodeError {}




pub enum OpCode {
    /// Returns the current stack frame
    Ret,
    /// Pushes a constant onto the stack
    Const(u8, u8),
    /// Negates the stop of the stack
    Neg(NumeralType),

    /// Binary operators
    Add(NumeralType),
    Sub(NumeralType),
    Mul(NumeralType),
    Div(NumeralType),
}

impl OpCode {
    pub fn disassemble(
        &self,
        f: &mut Formatter<'_>,
        offset: usize,
        lines: &[(u16, u16)],
    ) -> Result<usize, std::fmt::Error>{
        let line_format = if offset > 0 && lines[offset].0 == lines[offset - 1].0 {
            format!("   |")
        } else {
            format!("{:04}", lines[offset].0)
        };
        f.write_str(&format!("{:04}  {}-{char:03}  ", offset, line_format, char=lines[offset].1))?;

        match self {
            Self::Ret => {
                f.write_str("RET\n")?;
                Ok(offset + 1)
            },
            Self::Const(i, s) => {
                f.write_str(&format!("CONST {i:>16}{s:>16}\n"))?;
                Ok(offset + 3)
            },
            Self::Neg(n) => {
                f.write_str(&format!("NEG   {:?}\n", n))?;
                Ok(offset + 2)
            },

            OpCode::Add(n) => {
                f.write_str(&format!("ADD   {:?}\n", n))?;
                Ok(offset + 2)
            }
            OpCode::Sub(n) => {
                f.write_str(&format!("SUB   {:?}\n", n))?;
                Ok(offset + 2)
            }
            OpCode::Mul(n) => {
                f.write_str(&format!("MUL   {:?}\n", n))?;
                Ok(offset + 2)
            }
            OpCode::Div(n) => {
                f.write_str(&format!("DIV   {:?}\n", n))?;
                Ok(offset + 2)
            }
        }
    }

    pub fn write<F: FnMut(u8)>(self, mut writer: F) {
        match self {
            Self::Ret => writer(0),
            Self::Const(i, s) => {
                writer(1);
                writer(i);
                writer(s);
            },
            Self::Neg(num) => {
                writer(2);
                writer(num.into());
            },

            OpCode::Add(num) => {
                writer(3);
                writer(num.into());
            }
            OpCode::Sub(num) => {
                writer(4);
                writer(num.into());
            }
            OpCode::Mul(num) => {
                writer(5);
                writer(num.into());
            }
            OpCode::Div(num) => {
                writer(6);
                writer(num.into());
            }
        };
    }

    /// Returns the size of the opcode within program memory in bytes
    pub fn size(&self) -> usize {
        match self {
            OpCode::Ret => 1,
            OpCode::Const(_, _) => 3,
            OpCode::Neg(_) => 2,
            OpCode::Add(_) => 2,
            OpCode::Sub(_) => 2,
            OpCode::Mul(_) => 2,
            OpCode::Div(_) => 2,
        }
    }
}

impl Debug for OpCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // disassemble instruction
        todo!()
    }
}

impl TryFrom<(usize, &Chunk)> for OpCode {
    type Error = OpCodeError;

    /// Decode instruction
    fn try_from((offset, chunk): (usize, &Chunk)) -> Result<Self, Self::Error> {
        match chunk.code()[offset] {
            0 => Ok(Self::Ret),
            1 => Ok(Self::Const(chunk.code()[offset + 1], chunk.code()[offset + 2])),
            2 => Ok(Self::Neg(NumeralType::try_from(chunk.code()[offset + 1])?)),
            3 => Ok(Self::Add(NumeralType::try_from(chunk.code()[offset + 1])?)),
            4 => Ok(Self::Sub(NumeralType::try_from(chunk.code()[offset + 1])?)),
            5 => Ok(Self::Mul(NumeralType::try_from(chunk.code()[offset + 1])?)),
            6 => Ok(Self::Div(NumeralType::try_from(chunk.code()[offset + 1])?)),
            v => Err(OpCodeError::IllegalOpcode(v))
        }
    }
}
