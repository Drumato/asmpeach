use crate::Operand;
use crate::{REXPrefix, ModRM, SIBByte, Immediate, Displacement};


#[derive(Eq, Ord, PartialOrd, PartialEq, Debug, Clone)]
pub enum Opcode {
    // Move

    /// Move r8 to r/m8
    MOVRM8R8 {
        r8: Operand,
        rm8: Operand,
    },

    /// Move r64 to r/m64
    MOVRM64R64 {
        r64: Operand,
        rm64: Operand,
    },
}

impl Opcode {
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            // Move
            Opcode::MOVRM8R8 { r8: _, rm8: _ } => vec![0x88],
            Opcode::MOVRM64R64 { r64: _, rm64 : _} => vec![0x89],
        }
    }

    pub fn rex_prefix(&self) -> Option<REXPrefix> {
        match &self {

            // Move
            Opcode::MOVRM8R8 { rm8: _, r8: _, } => None,
            Opcode::MOVRM64R64 { rm64, r64, } => {
                Some(REXPrefix {
                    w_bit: true,
                    r_bit: rm64.is_expanded(),
                    // req_sib_byte() でindexフィールドが
                    x_bit: rm64.req_sib_byte() && rm64.index_reg_is_expanded(),
                    b_bit: r64.is_expanded(),
                })
            }
        }
    }

    #[allow(unreachable_patterns)]
    pub fn modrm(&self) -> Option<ModRM> {
        match &self {
            // Move
            Opcode::MOVRM8R8 { rm8, r8, } => {
                // MR
                Some(ModRM::new_mr(rm8.addressing_mode(), rm8, r8))
            }
            Opcode::MOVRM64R64 { rm64, r64, } => {
                // MR
                Some(ModRM::new_mr(rm64.addressing_mode(), rm64, r64))
            }

            _ => None,
        }
    }
    pub fn get_displacement(&self) -> Option<Displacement> {
        match &self {
            // Move
            Opcode::MOVRM8R8 { rm8, r8: _ } => rm8.get_displacement(),
            Opcode::MOVRM64R64 { rm64, r64: _ } => rm64.get_displacement(),
        }
    }
    pub fn get_immediate(&self) -> Option<Immediate> {
        match &self {
            // Move
            Opcode::MOVRM8R8 { rm8: _, r8: _ } => None,
            Opcode::MOVRM64R64 { rm64: _, r64: _ } => None,
        }
    }
    #[allow(unreachable_patterns)]
    pub fn sib_bite(&self) -> Option<SIBByte> {
        match &self {

            // Move
            Opcode::MOVRM8R8 { rm8, r8: _ } => rm8.sib_byte(),
            Opcode::MOVRM64R64 { rm64, r64: _ } => rm64.sib_byte(),
            _ => None,
        }
    }
}