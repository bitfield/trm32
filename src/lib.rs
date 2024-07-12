use std::collections::HashMap;

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
        format!(
            "a0:{:08x} sp:{:08x} ra:{:08x}",
            self.regs.a0, self.regs.sp, self.regs.ra
        )
    }
}

#[derive(Debug, PartialEq)]
enum RegisterID {
    X0,
    A0,
    RA,
    SP,
}

#[derive(Debug, Default)]
pub struct Registers {
    pc: Register,
    ra: Register,
    sp: Register,
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

fn decode(word: Word) -> Option<Instruction> {
    let operation = match word & 0b11111 {
        1 => Operation::LoadImmediate,
        _ => return None,
    };
    let dest = reg_id((word >> 5) & 0b1111);
    let src1 = reg_id((word >> 9) & 0b1111);
    let src2 = reg_id((word >> 13) & 0b1111);
    let imm = (word >> 16)
        .try_into()
        .expect("immediate argument should be 16 bits");
    Some(Instruction {
        operation,
        src1,
        src2,
        dest,
        imm,
    })
}

fn reg_id(id: u32) -> RegisterID {
    match id {
        0 => RegisterID::X0,
        1 => RegisterID::A0,
        _ => panic!("invalid register ID {id:02x}"),
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
        match decode(input) {
            Some(ins) => {
                assert_eq!(want, ins, "wrong decode: {input:08x}");
            }
            _ => panic!("not decoded: {input:08x}"),
        }
    }
}
