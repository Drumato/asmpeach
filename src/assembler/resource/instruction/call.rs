use crate::assembler::resource::Operand;

use super::Instruction;

#[derive(Eq, Ord, PartialOrd, PartialEq, Debug, Clone)]
pub struct Call {
    pub name: Operand,
}

impl Call {
    pub fn new(op: Operand) -> Self {
        todo!()
    }

    pub fn assemble(&self) -> Vec<u8> {
        todo!()
    }
}

impl Instruction for Call {
    fn opcode(&self) -> Vec<u8> {
        todo!()
    }

    fn name(&self) -> super::InstName {
        todo!()
    }
}
