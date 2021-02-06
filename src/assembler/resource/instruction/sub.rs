use crate::assembler::resource::*;

#[derive(Eq, Ord, PartialOrd, PartialEq, Debug, Clone)]
pub enum Sub {
    /// Subtract r/m64 from r64
    RM64FromR64 {
        r64: GeneralPurposeRegister,
        rm64: Operand,
    },

    /// Subtract r64 from r/m64
    R64FromRM64 {
        rm64: Operand,
        r64: GeneralPurposeRegister,
    },

    /// Subtract imm32 from r/m64
    SubImm32FromRM64 { rm64: Operand, imm: Immediate },
}

impl Sub {
    pub fn new(size: OperandSize, src: Operand, dst: Operand) -> Self {
        todo!()
    }
}
impl Instruction for Sub {
    fn opcode(&self) -> Vec<u8> {
        todo!()
    }

    fn name(&self) -> InstName {
        todo!()
    }
}
