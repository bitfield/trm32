use std::collections::HashMap;

use thiserror::Error;

#[derive(Debug, Default)]
pub struct Machine {
    regs: Registers,
    mem: Memory,
}

impl Machine {
    #[must_use]
    pub fn new() -> Self {
        Self {
            regs: Registers::default(),
            mem: Memory::default(),
        }
    }

    #[must_use]
    pub fn state(&self) -> String {
        format!("a0:{:08x}", self.regs.a0)
    }
}

#[derive(Debug, PartialEq)]
enum RegisterID {
    X0,
    A0,
}

#[derive(Debug, Default)]
pub struct Registers {
    a0: Register,
}

type Register = u32;
type Address = u32;
type Word = u32;
type Memory = HashMap<Address, Word>;

#[derive(Debug, PartialEq)]
struct Instruction {
    operation: Operation,
    src1: RegisterID,
    src2: RegisterID,
    dest: RegisterID,
    imm: u16,
}

#[derive(Debug, PartialEq)]
enum Operation {
    LoadImmediate,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("invalid instruction")]
    InvalidInstruction,
    #[error("unknown instruction {0}")]
    UnknownInstruction(Word),
}

impl TryFrom<Word> for Instruction {
    type Error = Error;

    fn try_from(word: Word) -> Result<Self, Self::Error> {
        let operation = match word & 0b11111 {
            1 => Operation::LoadImmediate,
            _ => return Err(Error::UnknownInstruction(word)),
        };
        let dest = RegisterID::from((word >> 5) & 0b1111);
        let src1 = RegisterID::from((word >> 9) & 0b1111);
        let src2 = RegisterID::from((word >> 13) & 0b1111);
        let imm = (word >> 16)
            .try_into()
            .expect("immediate argument should be 16 bits");
        Ok(Instruction {
            operation,
            src1,
            src2,
            dest,
            imm,
        })
    }
}

impl From<u32> for RegisterID {
    fn from(value: u32) -> Self {
        match value {
            0 => RegisterID::X0,
            1 => RegisterID::A0,
            _ => panic!("invalid register ID {value:02x}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_instruction() {
        let input: Word = 0b0000_0000_0000_0010_0000_0000_0010_0001;
        let want = Instruction {
            operation: Operation::LoadImmediate,
            src1: RegisterID::X0,
            src2: RegisterID::X0,
            dest: RegisterID::A0,
            imm: 2,
        };
        if let Ok(ins) = Instruction::try_from(input) {
            assert_eq!(want, ins, "wrong decode: {input:08x}");
        } else {
            panic!("not decoded: {input:08x}");
        }
    }
}
