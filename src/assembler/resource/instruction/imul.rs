use crate::assembler::resource::*;

#[derive(Eq, Ord, PartialOrd, PartialEq, Debug, Clone)]
pub enum IMul {
    /// Quadword register := Quadword register * r/m64
    R64WithRM64 {
        r64: GeneralPurposeRegister,
        rm64: Operand,
    },
}

impl IMul {
    pub fn new(size: OperandSize, src: Operand, dst: Operand) -> Self {
        todo!()
    }
}

impl Instruction for IMul {
    fn opcode(&self) -> Vec<u8> {
        todo!()
    }

    fn name(&self) -> InstName {
        todo!()
    }
}
