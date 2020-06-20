use crate::{Displacement, Encoding, GeneralPurposeRegister, Immediate, ModRM, Operand, REXPrefix, SIBByte};

#[derive(Eq, Ord, PartialOrd, PartialEq, Debug, Clone)]
pub enum Opcode {
    // Add
    /// Add r/m64 to r64
    ADDRM64R64 { rm64: Operand, r64: GeneralPurposeRegister },

    // Add r64 to r/m64
    ADDR64RM64 { r64: GeneralPurposeRegister, rm64: Operand },

    // Move
    /// Move r8 to r/m8
    MOVRM8R8 { r8: GeneralPurposeRegister, rm8: Operand },

    /// Move r64 to r/m64
    MOVRM64R64 { r64: GeneralPurposeRegister, rm64: Operand },

    /// Move imm32 to r/m64
    MOVRM64IMM32 { imm: Immediate, rm64: Operand },

    // Push
    /// Push r/m64
    PUSHRM64 { rm64: Operand },

    /// Push r64,
    PUSHR64 { r64: GeneralPurposeRegister },
    /// Push imm32
    PUSHIMM32 { imm: Immediate },

    // Sub
    /// Subtract r/m64 from r64
    SUBR64RM64 { r64: GeneralPurposeRegister, rm64: Operand },

    /// Subtract r64 from r/m64
    SUBRM64R64 { rm64: Operand, r64: GeneralPurposeRegister },

    /// Subtract imm32 from r/m64
    SUBRM64IMM32 { rm64: Operand, imm: Immediate },
}

impl Opcode {
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            // Add
            Opcode::ADDRM64R64 { rm64: _, r64: _ } => vec![0x01],
            Opcode::ADDR64RM64 { r64: _, rm64: _ } => vec![0x03],

            // Move
            Opcode::MOVRM8R8 { r8: _, rm8: _ } => vec![0x88],
            Opcode::MOVRM64R64 { r64: _, rm64: _ } => vec![0x89],
            Opcode::MOVRM64IMM32 { imm: _, rm64: _ } => vec![0xc7],

            // Push
            Opcode::PUSHRM64 { rm64: _ } => vec![0xff],
            Opcode::PUSHR64 { r64 } => vec![0x50 + r64.number()],
            Opcode::PUSHIMM32 { imm: _ } => vec![0x68],

            // Sub
            Opcode::SUBRM64IMM32 { rm64: _, imm: _ } => vec![0x81],
            Opcode::SUBR64RM64 { r64: _, rm64: _ } => vec![0x2b],
            Opcode::SUBRM64R64 { rm64: _, r64: _ } => vec![0x29],
        }
    }

    pub fn encoding(&self) -> Encoding {
        match self {
            // Add
            Opcode::ADDRM64R64 { rm64: _, r64: _ } => Encoding::MR,
            Opcode::ADDR64RM64 { r64: _, rm64: _ } => Encoding::RM,

            // Move
            Opcode::MOVRM8R8 { r8: _, rm8: _ } => Encoding::MR,
            Opcode::MOVRM64R64 { r64: _, rm64: _ } => Encoding::MR,
            Opcode::MOVRM64IMM32 { rm64: _, imm: _ } => Encoding::MI,

            // Push
            Opcode::PUSHRM64 { rm64: _ } => Encoding::M,
            Opcode::PUSHR64 { r64: _ } => Encoding::O,
            Opcode::PUSHIMM32 { imm: _ } => Encoding::I,

            // Sub
            Opcode::SUBRM64IMM32 { rm64: _, imm: _ } => Encoding::MI,
            Opcode::SUBR64RM64 { r64: _, rm64: _ } => Encoding::RM,
            Opcode::SUBRM64R64 { rm64: _, r64: _ } => Encoding::MR,
        }
    }

    /// calculating REX-Prefix byte
    pub fn rex_prefix(&self) -> Option<REXPrefix> {
        match &self {
            // Add
            Opcode::ADDRM64R64 { rm64, r64 } => Some(REXPrefix {
                w_bit: true,
                r_bit: rm64.is_expanded(),
                x_bit: rm64.req_sib_byte() && rm64.index_reg_is_expanded(),
                b_bit: r64.is_expanded(),
            }),
            Opcode::ADDR64RM64 { r64, rm64 } => Some(REXPrefix {
                w_bit: true,
                r_bit: rm64.is_expanded(),
                x_bit: rm64.req_sib_byte() && rm64.index_reg_is_expanded(),
                b_bit: r64.is_expanded(),
            }),

            // Move
            Opcode::MOVRM8R8 { rm8: _, r8: _ } => None,
            Opcode::MOVRM64R64 { rm64, r64 } => {
                Some(REXPrefix {
                    w_bit: true,
                    r_bit: rm64.is_expanded(),
                    // req_sib_byte() でindexフィールドが
                    x_bit: rm64.req_sib_byte() && rm64.index_reg_is_expanded(),
                    b_bit: r64.is_expanded(),
                })
            }
            Opcode::MOVRM64IMM32 { rm64, imm: _ } => {
                Some(
                    REXPrefix {
                        w_bit: true,
                        r_bit: rm64.is_expanded(),
                        // req_sib_byte() でindexフィールドが
                        x_bit: rm64.req_sib_byte() && rm64.index_reg_is_expanded(),
                        b_bit: false,
                    }
                )
            }

            // Push
            Opcode::PUSHRM64 { rm64: _ } => None,
            Opcode::PUSHR64 { r64 } => {
                if r64.is_expanded() {
                    Some(REXPrefix {
                        w_bit: false,
                        r_bit: false,
                        x_bit: false,
                        b_bit: true,
                    })
                } else {
                    None
                }
            }
            Opcode::PUSHIMM32 { imm: _ } => None,

            // Sub
            Opcode::SUBRM64IMM32 { rm64, imm: _ } => Some(
                REXPrefix {
                    w_bit: true,
                    r_bit: rm64.is_expanded(),
                    // req_sib_byte() でindexフィールドが
                    x_bit: rm64.req_sib_byte() && rm64.index_reg_is_expanded(),
                    b_bit: false,
                }
            ),
            Opcode::SUBR64RM64 { r64, rm64 } => Some(
                REXPrefix {
                    w_bit: true,
                    r_bit: rm64.is_expanded(),
                    // req_sib_byte() でindexフィールドが
                    x_bit: rm64.req_sib_byte() && rm64.index_reg_is_expanded(),
                    b_bit: r64.is_expanded(),
                }
            ),
            Opcode::SUBRM64R64 { rm64, r64 } => Some(REXPrefix {
                w_bit: true,
                r_bit: rm64.is_expanded(),
                // req_sib_byte() でindexフィールドが
                x_bit: rm64.req_sib_byte() && rm64.index_reg_is_expanded(),
                b_bit: r64.is_expanded(),
            }),
        }
    }

    /// calculating ModR/M byte
    #[allow(unreachable_patterns)]
    pub fn modrm(&self) -> Option<ModRM> {
        match &self {
            // Add
            Opcode::ADDRM64R64 { rm64, r64 } => {
                // MR
                Some(ModRM::new_mr(rm64.addressing_mode(), rm64, r64))
            }
            Opcode::ADDR64RM64 { r64, rm64 } => {
                // RM
                Some(ModRM::new_rm(rm64.addressing_mode(), r64, rm64))
            }

            // Move
            Opcode::MOVRM8R8 { rm8, r8 } => {
                // MR
                Some(ModRM::new_mr(rm8.addressing_mode(), rm8, r8))
            }
            Opcode::MOVRM64R64 { rm64, r64 } => {
                // MR
                Some(ModRM::new_mr(rm64.addressing_mode(), rm64, r64))
            }
            Opcode::MOVRM64IMM32 { rm64, imm: _ } => {
                // MI( /0 マスクなのでそのままMIで )
                Some(ModRM::new_mi(rm64.addressing_mode(), rm64))
            }

            // Push
            Opcode::PUSHRM64 { rm64 } => {
                // Mだけど /6 でマスクするのでmr
                Some(ModRM::new_mr(rm64.addressing_mode(), rm64, &GeneralPurposeRegister::RSI))
            }

            // Sub
            Opcode::SUBRM64IMM32 { rm64, imm: _ } => {
                // MIだけど /5 でマスクするのでmr
                Some(ModRM::new_mr(rm64.addressing_mode(), rm64, &GeneralPurposeRegister::RBP))
            }
            Opcode::SUBR64RM64 { r64, rm64 } => {
                // RM
                Some(ModRM::new_rm(rm64.addressing_mode(), r64, rm64))
            }
            Opcode::SUBRM64R64 { rm64, r64 } => {
                // MR
                Some(ModRM::new_mr(rm64.addressing_mode(), rm64, r64))
            }

            _ => None,
        }
    }

    /// get displacement
    /// if memory operand hasn't found, it returns Option::None,
    pub fn get_displacement(&self) -> Option<Displacement> {
        match &self {
            // Add
            Opcode::ADDRM64R64 { rm64, r64: _ } => rm64.get_displacement(),
            Opcode::ADDR64RM64 { r64: _, rm64 } => rm64.get_displacement(),

            // Move
            Opcode::MOVRM8R8 { rm8, r8: _ } => rm8.get_displacement(),
            Opcode::MOVRM64R64 { rm64, r64: _ } => rm64.get_displacement(),

            // Push
            Opcode::PUSHRM64 { rm64 } => rm64.get_displacement(),

            // Sub
            Opcode::SUBRM64R64 { rm64, r64: _ } => rm64.get_displacement(),
            Opcode::SUBRM64IMM32 { rm64, imm: _ } => rm64.get_displacement(),
            Opcode::SUBR64RM64 { r64: _, rm64 } => rm64.get_displacement(),
            _ => None,
        }
    }

    pub fn get_immediate(&self) -> Option<Immediate> {
        match &self {
            // Move
            Opcode::MOVRM64IMM32 { rm64: _, imm } => Some(*imm),

            // Push
            Opcode::PUSHIMM32 { imm } => Some(*imm),

            // Sub
            Opcode::SUBRM64IMM32 { rm64: _, imm } => Some(*imm),
            _ => None,
        }
    }

    pub fn sib_bite(&self) -> Option<SIBByte> {
        match &self {
            // Add
            Opcode::ADDRM64R64 { rm64, r64: _ } => rm64.sib_byte(),
            Opcode::ADDR64RM64 { r64: _, rm64 } => rm64.sib_byte(),

            // Move
            Opcode::MOVRM8R8 { rm8, r8: _ } => rm8.sib_byte(),
            Opcode::MOVRM64R64 { rm64, r64: _ } => rm64.sib_byte(),

            // Push
            Opcode::PUSHRM64 { rm64 } => rm64.sib_byte(),

            // Sub
            Opcode::SUBRM64IMM32 { rm64, imm: _ } => rm64.sib_byte(),
            Opcode::SUBRM64R64 { rm64, r64: _ } => rm64.sib_byte(),
            Opcode::SUBR64RM64 { r64: _, rm64 } => rm64.sib_byte(),
            _ => None,
        }
    }

    /// to Intel syntax.
    pub fn to_intel_string(&self) -> String {
        match &self {
            // Add
            Opcode::ADDRM64R64 { rm64, r64 } => {
                format!("add {}, {}", rm64.to_intel_string(), r64.to_intel_string())
            }
            Opcode::ADDR64RM64 { r64, rm64 } => {
                format!("add {}, {}", r64.to_intel_string(), rm64.to_intel_string())
            }

            // Move
            Opcode::MOVRM8R8 { rm8, r8 } => {
                format!("mov {}, {}", rm8.to_intel_string(), r8.to_intel_string())
            }
            Opcode::MOVRM64R64 { rm64, r64 } => {
                format!("mov {}, {}", rm64.to_intel_string(), r64.to_intel_string())
            }
            Opcode::MOVRM64IMM32 { rm64, imm } => {
                format!("mov {}, {}", rm64.to_intel_string(), imm.to_intel_string())
            }

            // Push
            Opcode::PUSHRM64 { rm64 } => {
                format!("push {}", rm64.to_intel_string())
            }
            Opcode::PUSHR64 { r64 } => format!("push {}", r64.to_intel_string()),
            Opcode::PUSHIMM32 { imm } => format!("push {}", imm.to_intel_string()),

            // Sub
            Opcode::SUBRM64IMM32 { rm64, imm } => {
                format!("sub {}, {}", rm64.to_intel_string(), imm.to_intel_string())
            }
            Opcode::SUBR64RM64 { r64, rm64 } => {
                format!("sub {}, {}", r64.to_intel_string(), rm64.to_intel_string())
            }
            Opcode::SUBRM64R64 { rm64, r64 } => {
                format!("sub {}, {}", rm64.to_intel_string(), r64.to_intel_string())
            }
        }
    }

    /// to AT&T syntax.
    pub fn to_at_string(&self) -> String {
        match &self {
            // Add
            Opcode::ADDRM64R64 { rm64, r64 } => {
                format!("addq {}, {}", r64.to_at_string(), rm64.to_at_string())
            }
            Opcode::ADDR64RM64 { r64, rm64 } => {
                format!("addq {}, {}", rm64.to_at_string(), r64.to_at_string())
            }

            // Move
            Opcode::MOVRM8R8 { rm8, r8 } => {
                format!("movb {}, {}", r8.to_at_string(), rm8.to_at_string())
            }
            Opcode::MOVRM64R64 { rm64, r64 } => {
                format!("movq {}, {}", r64.to_at_string(), rm64.to_at_string())
            }
            Opcode::MOVRM64IMM32 { rm64, imm } => {
                format!("movq {}, {}", imm.to_at_string(), rm64.to_at_string())
            }

            // Push
            Opcode::PUSHRM64 { rm64 } => {
                format!("pushq {}", rm64.to_at_string())
            }
            Opcode::PUSHR64 { r64 } => format!("pushq {}", r64.to_at_string()),
            Opcode::PUSHIMM32 { imm } => format!("pushq {}", imm.to_at_string()),

            // Sub
            Opcode::SUBRM64IMM32 { rm64, imm } => {
                format!("subq {}, {}", imm.to_at_string(), rm64.to_at_string())
            }
            Opcode::SUBR64RM64 { r64, rm64 } => {
                format!("subq {}, {}", rm64.to_at_string(), r64.to_at_string())
            }
            Opcode::SUBRM64R64 { rm64, r64 } => {
                format!("subq {}, {}", r64.to_at_string(), rm64.to_at_string())
            }
        }
    }
}
