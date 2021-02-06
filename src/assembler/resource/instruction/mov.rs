use crate::assembler::resource::*;

#[derive(Eq, Ord, PartialOrd, PartialEq, Debug, Clone)]
pub enum Mov {
    /// Move r8 to r/m8
    R8ToRM8 {
        r8: GeneralPurposeRegister,
        rm8: Operand,
    },

    /// Move r32 to r/m32
    R32ToRM32 {
        r32: GeneralPurposeRegister,
        rm32: Operand,
    },

    /// Move r/m32 to r32
    RM32ToR32 {
        r32: GeneralPurposeRegister,
        rm32: Operand,
    },

    /// Move imm32 to r/m32
    Imm32ToRM32 { imm: Immediate, rm32: Operand },

    /// Move r64 to r/m64
    R64ToRM64 {
        r64: GeneralPurposeRegister,
        rm64: Operand,
    },

    /// Move r/m64 to r64
    RM64ToR64 {
        r64: GeneralPurposeRegister,
        rm64: Operand,
    },

    /// Move imm32 to r/m64
    Imm32ToRM64 { rm64: Immediate, dst: Operand },
}

impl Mov {
    pub fn new(size: OperandSize, src: Operand, dst: Operand) -> Self {
        todo!()
    }
}

impl Instruction for Mov {
    fn opcode(&self) -> Vec<u8> {
        todo!()
    }

    fn name(&self) -> InstName {
        todo!()
    }
}
