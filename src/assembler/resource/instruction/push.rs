use crate::assembler::resource::*;

#[derive(Eq, Ord, PartialOrd, PartialEq, Debug, Clone)]
pub enum Push {
    /// Push r/m64
    RM64 { rm64: Operand },

    /// Push r64,
    R64 { r64: GeneralPurposeRegister },
    /// Push imm32
    Imm32 { imm: Immediate },
}

impl Push {
    pub fn new(size: OperandSize, op: Operand) -> Self {
        todo!()
    }
}

impl Instruction for Push {
    fn opcode(&self) -> Vec<u8> {
        todo!()
    }

    fn name(&self) -> InstName {
        todo!()
    }
}
