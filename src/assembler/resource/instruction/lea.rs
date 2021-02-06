use crate::assembler::resource::*;

#[derive(Eq, Ord, PartialOrd, PartialEq, Debug, Clone)]
pub enum Lea {
    /// Store effective address for m in register r64
    FromMemToR64 {
        r64: GeneralPurposeRegister,
        mem: Operand,
    },
}

impl Lea {
    pub fn new(size: OperandSize, src: Operand, dst: Operand) -> Self {
        todo!()
    }
}

impl Instruction for Lea {
    fn opcode(&self) -> Vec<u8> {
        todo!()
    }

    fn name(&self) -> InstName {
        todo!()
    }
}
