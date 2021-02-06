use crate::assembler::resource::*;

#[derive(Eq, Ord, PartialOrd, PartialEq, Debug, Clone)]
pub enum Pop {
    R64 { r64: GeneralPurposeRegister },
}

impl Pop {
    pub fn new(size: OperandSize, op: Operand) -> Self {
        todo!()
    }
}

impl Instruction for Pop {
    fn opcode(&self) -> Vec<u8> {
        todo!()
    }

    fn name(&self) -> InstName {
        todo!()
    }
}
