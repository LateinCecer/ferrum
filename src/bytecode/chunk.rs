use std::fmt::{Debug, Formatter};
use std::io::Write;
use std::ops::Index;
use crate::bytecode::opcode::OpCode;

pub trait Value<const N: usize>: Sized {
    fn to_bits(self) -> [u8; N];
    fn from_bits(bits: [u8; N]) -> Self;
}

type CodePos = (u16, u16);


pub struct CodeRef {
    pos: usize,
    size: usize,
}

pub struct Chunk {
    name: String,
    /// Stores the actual byte code instruction set
    code: Vec<u8>,
    /// Stores constant values
    vals: Vec<u8>,
    /// Lines of code for the corresponding instruction
    lines: Vec<CodePos>,
}

impl Chunk {
    pub fn new(name: String) -> Self {
        Chunk {
            name,
            code: Vec::with_capacity(512),
            vals: Vec::with_capacity(512),
            lines: Vec::with_capacity(512),
        }
    }

    /// Writes a single bytecode instruction to the chunk and returns the instruction index of the
    /// opcode.
    pub fn write(&mut self, b: OpCode, line: u16, char: u16) -> CodeRef {
        let i = self.code.len();
        b.write(|c| {
            self.code.push(c);
            self.lines.push((line, char));
        });
        let size = self.code.len() - i;
        CodeRef { pos: i, size, }
    }

    /// Clears all values in the chunk
    pub fn clear(&mut self) {
        self.code.clear();
        self.vals.clear();
    }

    /// Writes a single data-value entry to the vector of constants for this code chunk. Returns
    /// the index of the written value.
    pub fn write_value<const N: usize, Val: Value<N>>(&mut self, val: Val) -> usize {
        let i = self.vals.len();
        let bits = val.to_bits();
        self.vals.write_all(&bits).expect("Failed to write all bits to value vector");
        i
    }

    /// Disassembles the code chunk into semi-human readable instruction sets.
    pub fn disassemble(&self, mut i: usize, count: usize, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut c = 0;
        while i < self.code.len() && c < count {
            i = OpCode::try_from((i, self))
                .unwrap()
                .disassemble(f, i, &self.lines)?;

            c += 1;
        }
        Ok(())
    }

    pub fn code(&self) -> &[u8] {
        &self.code
    }

    pub fn vals(&self) -> &[u8] {
        &self.vals
    }
}

impl Debug for Chunk {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.disassemble(0, 10, f)
    }
}
