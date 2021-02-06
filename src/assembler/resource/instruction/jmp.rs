use crate::assembler::resource::Operand;

use super::Instruction;

#[derive(Eq, Ord, PartialOrd, PartialEq, Debug, Clone)]
pub enum Jmp {
    Unconditional(Operand),
    LessThanEqual(Operand),
    Equal(Operand),
}

impl Jmp {
    pub fn label(&self) -> String {
        match self {
            Jmp::Unconditional(op) | Jmp::LessThanEqual(op) | Jmp::Equal(op) => op.copy_label(),
        }
    }

    pub fn assemble(&self) -> Vec<u8> {
        todo!()
    }
}

impl Instruction for Jmp {
    fn opcode(&self) -> Vec<u8> {
        todo!()
    }

    fn name(&self) -> super::InstName {
        todo!()
    }
}
