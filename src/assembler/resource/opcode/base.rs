use crate::assembler::resource::*;

#[derive(Eq, Ord, PartialOrd, PartialEq, Debug, Clone)]
pub enum Opcode {
    // Add
        /// Add r/m32 to r32
        ADDRM32R32 {
            rm32: Operand,
            r32: GeneralPurposeRegister,
        },
    
        /// Add r32 to r/m32
        ADDR32RM32 {
            r32: GeneralPurposeRegister,
            rm32: Operand,
        },
    /// Add r/m64 to r64
    ADDRM64R64 {
        rm64: Operand,
        r64: GeneralPurposeRegister,
    },

    /// Add r64 to r/m64
    ADDR64RM64 {
        r64: GeneralPurposeRegister,
        rm64: Operand,
    },

    // Call
    /// CALL Function (abstraction)
    CALLFUNC(Operand),

    // Convert Word to Doubleword/Convert Doubleword to Quadword
    /// DX:AX := Sign-extended of AX
    CWD,
    /// EDX:EAX := Sign-extended of EAX
    CDQ,
    /// RDX:RAX := Sign-extended of RAX
    CQO,

    // Compare Two Operands
    /// Compare imm32 with RAX.
    CMPRAXIMM32 { imm: Immediate },

    /// End Branch 64bit
    ENDBR64,

    // (signed) Integer Divide
    /// signed divide RDX:RAX by r/m64;
    /// result stored in RAX := Quotient, RDX := Remainder.
    IDIVRM64 { rm64: Operand },

    // (signed) Integer Multiply
    /// Quadword register := Quadword register * r/m64
    IMULR64RM64 {
        r64: GeneralPurposeRegister,
        rm64: Operand,
    },

    // Increment
    /// increment r/m64 by one.
    INCRM64 { rm64: Operand },

    // Jump
    /// Jump Label
    /// このopcodeに対し `to_bytes()` メソッドを呼び出すと，
    /// `[REX-Prefix, opcode]` が返されます．
    /// 自作アセンブラでこのバイト列に対し `imm32(0)` を追加して，
    /// あとから相対オフセットを計算するといいと思います．
    JMPLABEL { label: String },

    /// Jump Equal Label
    /// このopcodeに対し `to_bytes()` メソッドを呼び出すと，
    /// `[REX-Prefix, opcode1, opcode2]` が返されます．
    /// 自作アセンブラでこのバイト列に対し `imm32(0)` を追加して，
    /// あとから相対オフセットを計算するといいと思います．
    JELABEL { label: String },

    // Load Effective Address
    /// Store effective address for m in register r64
    LEAR64M{r64: GeneralPurposeRegister, m: Operand},

    // Move
    /// Move r8 to r/m8
    MOVRM8R8 {
        r8: GeneralPurposeRegister,
        rm8: Operand,
    },

        /// Move r32 to r/m32
        MOVRM32R32 {
            r32: GeneralPurposeRegister,
            rm32: Operand,
        },
    
        /// Move r/m32 to r32
        MOVR32RM32 {
            r32: GeneralPurposeRegister,
            rm32: Operand,
        },
    
        /// Move imm32 to r/m32
        MOVRM32IMM32 { imm: Immediate, rm32: Operand },

    /// Move r64 to r/m64
    MOVRM64R64 {
        r64: GeneralPurposeRegister,
        rm64: Operand,
    },

    /// Move r/m64 to r64
    MOVR64RM64 {
        r64: GeneralPurposeRegister,
        rm64: Operand,
    },

    /// Move imm32 to r/m64
    MOVRM64IMM32 { imm: Immediate, rm64: Operand },

    // Neg
    /// Two's complement negate r/m64
    NEGRM64 { rm64: Operand },

    // Pop
    /// Pop top of stack into r64; increment stack pointer; Cannot encode 32-bit operand size.
    POPR64 { r64: GeneralPurposeRegister },

    // Push
    /// Push r/m64
    PUSHRM64 { rm64: Operand },

    /// Push r64,
    PUSHR64 { r64: GeneralPurposeRegister },
    /// Push imm32
    PUSHIMM32 { imm: Immediate },

    // Return from procedure
    /// Near Return
    RET,

    // Sub
    /// Subtract r/m64 from r64
    SUBR64RM64 {
        r64: GeneralPurposeRegister,
        rm64: Operand,
    },

    /// Subtract r64 from r/m64
    SUBRM64R64 {
        rm64: Operand,
        r64: GeneralPurposeRegister,
    },

    /// Subtract imm32 from r/m64
    SUBRM64IMM32 { rm64: Operand, imm: Immediate },

    /// Fast System Call
    SYSCALL,

    // etc
    /// for comments
    COMMENT(String),
}

impl Opcode {
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            // Add
            Opcode::ADDRM32R32 { rm32: _, r32: _ } => vec![0x01],
            Opcode::ADDR32RM32 { r32: _, rm32: _ } => vec![0x03],
            Opcode::ADDRM64R64 { rm64: _, r64: _ } => vec![0x01],
            Opcode::ADDR64RM64 { r64: _, rm64: _ } => vec![0x03],

            // Call
            Opcode::CALLFUNC(_func) => unimplemented!(),

            // Convert Word to Doubleword/Convert Doubleword to Quadword
            Opcode::CWD => vec![0x66, 0x99],
            Opcode::CDQ | Opcode::CQO => vec![0x99],

            Opcode::CMPRAXIMM32 { imm: _ } => vec![0x3d],

            Opcode::ENDBR64 => vec![0xf3, 0x0f, 0x1e, 0xfa],

            // (signed) Integer Divide
            Opcode::IDIVRM64 { rm64: _ } => vec![0xf7],

            // (signed) Integer Multiply
            Opcode::IMULR64RM64 { r64: _, rm64: _ } => vec![0x0f, 0xaf],

            // Increment
            Opcode::INCRM64 { rm64: _ } => vec![0xff],

            // Jump
            Opcode::JMPLABEL { label: _ } => vec![0xe9],
            Opcode::JELABEL { label: _ } => vec![0x0f, 0x84],

            // Load Effective Address
            Opcode::LEAR64M {r64: _, m: _} => vec![0x8d],

            // Move
            Opcode::MOVRM8R8 { r8: _, rm8: _ } => vec![0x88],
            Opcode::MOVRM32R32 { r32: _, rm32: _ } => vec![0x89],
            Opcode::MOVR32RM32 { r32: _, rm32: _ } => vec![0x8b],
            Opcode::MOVRM32IMM32 { imm: _, rm32: _ } => vec![0xc7],
            Opcode::MOVRM64R64 { r64: _, rm64: _ } => vec![0x89],
            Opcode::MOVR64RM64 { r64: _, rm64: _ } => vec![0x8b],
            Opcode::MOVRM64IMM32 { imm: _, rm64: _ } => vec![0xc7],

            // Neg
            Opcode::NEGRM64 { rm64: _ } => vec![0xf7],

            // Pop
            Opcode::POPR64 { r64 } => vec![0x58 + r64.number()],

            // Push
            Opcode::PUSHRM64 { rm64: _ } => vec![0xff],
            Opcode::PUSHR64 { r64 } => vec![0x50 + r64.number()],
            Opcode::PUSHIMM32 { imm: _ } => vec![0x68],

            // Return from procedure
            Opcode::RET => vec![0xc3],

            // Sub
            Opcode::SUBRM64IMM32 { rm64: _, imm: _ } => vec![0x81],
            Opcode::SUBR64RM64 { r64: _, rm64: _ } => vec![0x2b],
            Opcode::SUBRM64R64 { rm64: _, r64: _ } => vec![0x29],

            // Fast System Call
            Opcode::SYSCALL => vec![0x0f, 0x05],

            // etc
            Opcode::COMMENT(_com) => panic!("mustn't call 'to_bytes()' with COMMENT"),
        }
    }

    pub fn encoding(&self) -> Encoding {
        match self {
            Opcode::ADDRM32R32 { rm32: _, r32: _ } => Encoding::MR,
            Opcode::ADDR32RM32 { r32: _, rm32: _ } => Encoding::RM,
            Opcode::ADDRM64R64 { rm64: _, r64: _ } => Encoding::MR,
            Opcode::ADDR64RM64 { r64: _, rm64: _ } => Encoding::RM,
            Opcode::CALLFUNC(_func) => unimplemented!(),
            Opcode::CWD | Opcode::CDQ | Opcode::CQO => Encoding::ZO,
            Opcode::CMPRAXIMM32 { imm: _ } => Encoding::I,
            Opcode::ENDBR64 => Encoding::ZO,
            Opcode::IDIVRM64 { rm64: _ } => Encoding::M,
            Opcode::IMULR64RM64 { r64: _, rm64: _ } => Encoding::RM,
            Opcode::INCRM64 { rm64: _ } => Encoding::M,
            Opcode::JMPLABEL { label: _ } => Encoding::D,
            Opcode::JELABEL { label: _ } => Encoding::D,
            Opcode::LEAR64M {r64: _, m: _}=> Encoding::RM,
            Opcode::MOVRM8R8 { r8: _, rm8: _ } => Encoding::MR,
            Opcode::MOVRM32R32 { r32: _, rm32: _ } => Encoding::MR,
            Opcode::MOVR32RM32 { r32: _, rm32: _ } => Encoding::RM,
            Opcode::MOVRM32IMM32 { rm32: _, imm: _ } => Encoding::MI,
            Opcode::MOVRM64R64 { r64: _, rm64: _ } => Encoding::MR,
            Opcode::MOVR64RM64 { r64: _, rm64: _ } => Encoding::RM,
            Opcode::MOVRM64IMM32 { rm64: _, imm: _ } => Encoding::MI,
            Opcode::NEGRM64 { rm64: _ } => Encoding::M,
            Opcode::POPR64 { r64: _ } => Encoding::O,
            Opcode::PUSHRM64 { rm64: _ } => Encoding::M,
            Opcode::PUSHR64 { r64: _ } => Encoding::O,
            Opcode::PUSHIMM32 { imm: _ } => Encoding::I,
            Opcode::RET => Encoding::ZO,
            Opcode::SUBRM64IMM32 { rm64: _, imm: _ } => Encoding::MI,
            Opcode::SUBR64RM64 { r64: _, rm64: _ } => Encoding::RM,
            Opcode::SUBRM64R64 { rm64: _, r64: _ } => Encoding::MR,
            Opcode::SYSCALL => Encoding::ZO,
            Opcode::COMMENT(_com) => panic!("mustn't call 'encoding()' with COMMENT"),
        }
    }

    /// calculating REX-Prefix byte
    pub fn rex_prefix(&self) -> Option<REXPrefix> {
        match &self {
            // Add
            Opcode::ADDRM64R64 { rm64, r64 } => {
                Some(REXPrefix::new_from_mem_and_reg(true, r64, rm64))
            }
            Opcode::ADDR64RM64 { r64, rm64 } => {
                Some(REXPrefix::new_from_mem_and_reg(true, r64, rm64))
            }

            // Convert Word to Doubleword/Convert Doubleword to Quadword
            Opcode::CQO => Some(REXPrefix::new(true, false, false, false)),

            // Compare Two Operands
            Opcode::CMPRAXIMM32 { imm: _ } => Some(REXPrefix::new(true, false, false, false)),

            // (signed) Integer Multiply
            Opcode::IMULR64RM64 { r64, rm64 } => {
                Some(REXPrefix::new(true, r64.is_expanded(), rm64.index_reg_is_expanded(), rm64.is_expanded()))
            }

            // (signed) Integer Divide
            Opcode::IDIVRM64 { rm64 } => Some(REXPrefix::new_from_mem(true, rm64)),

            // Increment
            Opcode::INCRM64 { rm64 } => Some(REXPrefix::new_from_mem(true, rm64)),

            // Load Effective Address
            Opcode::LEAR64M {
                r64,
                m,
            } => Some(REXPrefix::new(true, r64.is_expanded(), m.index_reg_is_expanded(), m.is_expanded())),

            // Move
            Opcode::MOVRM64R64 { rm64, r64 } => {
                Some(REXPrefix::new(true, r64.is_expanded(), rm64.index_reg_is_expanded(), rm64.is_expanded()))
            }
            Opcode::MOVR64RM64 { r64, rm64 } => {
                Some(REXPrefix::new(true, r64.is_expanded(), rm64.index_reg_is_expanded(), rm64.is_expanded()))
            }
            Opcode::MOVRM64IMM32 { rm64, imm: _ } => {
                Some(REXPrefix::new(true, false, rm64.req_sib_byte() && rm64.index_reg_is_expanded(), rm64.is_expanded()))
            },

            // Neg
            Opcode::NEGRM64 { rm64 } => Some(REXPrefix::new_from_mem(true, rm64)),

            // Pop
            Opcode::POPR64 { r64 } => {
                if r64.is_expanded() {
                    Some(REXPrefix::new(false, false, false, true))
                } else {
                    None
                }
            }

            // Push
            Opcode::PUSHRM64 { rm64: _ } => None,
            Opcode::PUSHR64 { r64 } => {
                if r64.is_expanded() {
                    Some(REXPrefix::new(false, false, false, true))
                } else {
                    None
                }
            }
            // Sub
            Opcode::SUBRM64IMM32 { rm64, imm: _ } => Some(REXPrefix::new_from_mem(true, rm64)),
            Opcode::SUBR64RM64 { r64, rm64 } => {
                Some(REXPrefix::new_from_mem_and_reg(true, r64, rm64))
            }
            Opcode::SUBRM64R64 { rm64, r64 } => {
                Some(REXPrefix::new_from_mem_and_reg(true, r64, rm64))
            }

            _ => None,
        }
    }

    /// calculating ModR/M byte
    #[allow(unreachable_patterns)]
    pub fn modrm(&self) -> Option<ModRM> {
        match &self {
            // Add
            Opcode::ADDRM32R32 { rm32, r32 } => {
                // MR
                Some(ModRM::new_mr(rm32.addressing_mode(), rm32, r32))
            }
            Opcode::ADDR32RM32 { r32, rm32 } => {
                // RM
                Some(ModRM::new_rm(rm32.addressing_mode(), r32, rm32))
            }
            Opcode::ADDRM64R64 { rm64, r64 } => {
                // MR
                Some(ModRM::new_mr(rm64.addressing_mode(), rm64, r64))
            }
            Opcode::ADDR64RM64 { r64, rm64 } => {
                // RM
                Some(ModRM::new_rm(rm64.addressing_mode(), r64, rm64))
            }

            // (signed) Integer Divide
            Opcode::IDIVRM64 { rm64 } => {
                // Mだけど /7 でマスク
                Some(ModRM::new_mr(
                    rm64.addressing_mode(),
                    rm64,
                    &GeneralPurposeRegister::new_64bit_from_code(7),
                ))
            }

            // (signed) Integer Multiply
            Opcode::IMULR64RM64 { r64, rm64 } => {
                // RM
                Some(ModRM::new_rm(rm64.addressing_mode(), r64, rm64))
            }

            // Increment
            Opcode::INCRM64 { rm64 } => {
                // Mだけど /0 なのでマスク
                Some(ModRM::new_mr(
                    rm64.addressing_mode(),
                    rm64,
                    &GeneralPurposeRegister::new_64bit_from_code(0),
                ))
            }

            // Load Effective Address
            Opcode::LEAR64M {
                r64,
                m,
            } => Some(ModRM::new_rm(
                AddressingMode::DISP8,
                r64,
                m,
            )),

            // Move
            Opcode::MOVRM8R8 { rm8, r8 } => {
                // MR
                Some(ModRM::new_mr(rm8.addressing_mode(), rm8, r8))
            }
            Opcode::MOVRM32R32 { rm32, r32 } => {
                // MR
                Some(ModRM::new_mr(rm32.addressing_mode(), rm32, r32))
            }
            Opcode::MOVR32RM32 { r32, rm32 } => {
                // RM
                Some(ModRM::new_rm(rm32.addressing_mode(), r32, rm32))
            }
            Opcode::MOVRM32IMM32 { rm32, imm: _ } => {
                // MI( /0 マスクなのでそのままMIで )
                Some(ModRM::new_mi(rm32.addressing_mode(), rm32))
            }

            Opcode::MOVRM64R64 { rm64, r64 } => {
                // MR
                Some(ModRM::new_mr(rm64.addressing_mode(), rm64, r64))
            }
            Opcode::MOVR64RM64 { r64, rm64 } => {
                // RM
                Some(ModRM::new_rm(rm64.addressing_mode(), r64, rm64))
            }
            Opcode::MOVRM64IMM32 { rm64, imm: _ } => {
                // MI( /0 マスクなのでそのままMIで )
                Some(ModRM::new_mi(rm64.addressing_mode(), rm64))
            }

            // Neg
            Opcode::NEGRM64 { rm64 } => {
                // Mだけど /3 でマスクするのでmr
                Some(ModRM::new_mr(
                    rm64.addressing_mode(),
                    rm64,
                    &GeneralPurposeRegister::new_64bit_from_code(3),
                ))
            }

            // Pop

            // Push
            Opcode::PUSHRM64 { rm64 } => {
                // Mだけど /6 でマスクするのでmr
                Some(ModRM::new_mr(
                    rm64.addressing_mode(),
                    rm64,
                    &GeneralPurposeRegister::new_64bit_from_code(6),
                ))
            }

            // Sub
            Opcode::SUBRM64IMM32 { rm64, imm: _ } => {
                // MIだけど /5 でマスクするのでmr
                Some(ModRM::new_mr(
                    rm64.addressing_mode(),
                    rm64,
                    &GeneralPurposeRegister::new_64bit_from_code(5),
                ))
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
            Opcode::ADDRM32R32 { rm32, r32: _ } => rm32.get_displacement(),
            Opcode::ADDR32RM32 { r32: _, rm32 } => rm32.get_displacement(),
            Opcode::ADDRM64R64 { rm64, r64: _ } => rm64.get_displacement(),
            Opcode::ADDR64RM64 { r64: _, rm64 } => rm64.get_displacement(),

            // (signed) Integer Divide
            Opcode::IDIVRM64 { rm64 } => rm64.get_displacement(),

            // (signed) Integer Multiply
            Opcode::IMULR64RM64 { r64: _, rm64 } => rm64.get_displacement(),

            // Increment
            Opcode::INCRM64 { rm64 } => rm64.get_displacement(),

            // Lea
            Opcode::LEAR64M {r64: _, m } => m.get_displacement(),

            // Move
            Opcode::MOVRM8R8 { rm8, r8: _ } => rm8.get_displacement(),
            Opcode::MOVR32RM32 { rm32, r32: _ } => rm32.get_displacement(),
            Opcode::MOVRM32R32 { rm32, r32: _ } => rm32.get_displacement(),
            Opcode::MOVRM32IMM32{rm32, imm: _} => rm32.get_displacement(),
            Opcode::MOVR64RM64 { rm64, r64: _ } => rm64.get_displacement(),
            Opcode::MOVRM64R64 { rm64, r64: _ } => rm64.get_displacement(),
            Opcode::MOVRM64IMM32{rm64, imm: _} => rm64.get_displacement(),

            // Neg
            Opcode::NEGRM64 { rm64 } => rm64.get_displacement(),

            // Pop

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
            // Compare Two Operands
            Opcode::CMPRAXIMM32 { imm } => Some(*imm),

            // Move
            Opcode::MOVRM32IMM32 { rm32: _, imm } => Some(*imm),
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
            Opcode::ADDRM32R32 { rm32, r32: _ } => rm32.sib_byte(),
            Opcode::ADDR32RM32 { r32: _, rm32 } => rm32.sib_byte(),
            Opcode::ADDRM64R64 { rm64, r64: _ } => rm64.sib_byte(),
            Opcode::ADDR64RM64 { r64: _, rm64 } => rm64.sib_byte(),

            // (signed) Integer Divide
            Opcode::IDIVRM64 { rm64 } => rm64.sib_byte(),

            // (signed) Integer Multiply
            Opcode::IMULR64RM64 { r64: _, rm64 } => rm64.sib_byte(),

            // Increment
            Opcode::INCRM64 { rm64 } => rm64.sib_byte(),

            // Move
            Opcode::MOVRM8R8 { rm8, r8: _ } => rm8.sib_byte(),
            Opcode::MOVRM32R32 { rm32, r32: _ } => rm32.sib_byte(),
            Opcode::MOVRM64R64 { rm64, r64: _ } => rm64.sib_byte(),

            // Neg
            Opcode::NEGRM64 { rm64 } => rm64.sib_byte(),

            // Pop

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
            Opcode::CWD | Opcode::CDQ | Opcode::CQO | Opcode::ENDBR64 | Opcode::RET | Opcode::SYSCALL => {
                self.opcode_to_intel().to_string()
            }
            Opcode::CALLFUNC(func) => {
                format!("{} {}", self.opcode_to_intel(), func.to_intel_string())
            }
            Opcode::CMPRAXIMM32 { imm } => {
                format!("{} rax, {}", self.opcode_to_intel(), imm.to_intel_string())
            }
            Opcode::JMPLABEL { label } => format!("{} {}", self.opcode_to_intel(), label),
            Opcode::JELABEL { label } => format!("{} {}", self.opcode_to_intel(), label),
            Opcode::LEAR64M {
                r64,
                m,
            } => format!(
                "{} {}, {}",
                self.opcode_to_intel(),
                r64.to_intel_string(),
                m.to_intel_string()
            ),

            // r64
            Opcode::POPR64 { r64 } | Opcode::PUSHR64 { r64 } => {
                format!("{} {}", self.opcode_to_intel(), r64.to_intel_string())
            }

            // imm32
            Opcode::PUSHIMM32 { imm } => {
                format!("{} {}", self.opcode_to_intel(), imm.to_intel_string())
            }

            // r/m64
            Opcode::IDIVRM64 { rm64 }
            | Opcode::INCRM64 { rm64 }
            | Opcode::PUSHRM64 { rm64 }
            | Opcode::NEGRM64 { rm64 } => {
                format!("{} {}", self.opcode_to_intel(), rm64.to_intel_string())
            }

            // r32, r/m32
            Opcode::ADDR32RM32{r32, rm32}
            | Opcode::MOVR32RM32{r32, rm32} => format!(
                "{} {}, {}",
                self.opcode_to_intel(),
                r32.to_intel_string(),
                rm32.to_intel_string()
            ),

            // r64, r/m64
            Opcode::ADDR64RM64 { r64, rm64 }
            | Opcode::IMULR64RM64 { r64, rm64 }
            | Opcode::SUBR64RM64 { r64, rm64 }
            | Opcode::MOVR64RM64 { r64, rm64 } => format!(
                "{} {}, {}",
                self.opcode_to_intel(),
                r64.to_intel_string(),
                rm64.to_intel_string()
            ),

            // r/m8, r8
            Opcode::MOVRM8R8 { rm8, r8 } => format!(
                "{} {}, {}",
                self.opcode_to_intel(),
                rm8.to_intel_string(),
                r8.to_intel_string()
            ),

            // r/m32, r32
            Opcode::ADDRM32R32 { rm32, r32 }
            | Opcode::MOVRM32R32{rm32, r32} => format!(
                "{} {}, {}",
                self.opcode_to_intel(),
                rm32.to_intel_string(),
                r32.to_intel_string()
            ),

            // r/m64, r64
            Opcode::ADDRM64R64 { rm64, r64 }
            | Opcode::MOVRM64R64 { rm64, r64 }
            | Opcode::SUBRM64R64 { rm64, r64 } => format!(
                "{} {}, {}",
                self.opcode_to_intel(),
                rm64.to_intel_string(),
                r64.to_intel_string()
            ),

            // r/m32, imm32
            Opcode::MOVRM32IMM32 { rm32, imm } => format!(
                "{} {}, {}",
                self.opcode_to_intel(),
                rm32.to_intel_string(),
                imm.to_intel_string()
            ),

            // r/m64, imm32
            Opcode::MOVRM64IMM32 { rm64, imm } | Opcode::SUBRM64IMM32 { rm64, imm } => format!(
                "{} {}, {}",
                self.opcode_to_intel(),
                rm64.to_intel_string(),
                imm.to_intel_string()
            ),

            // etc
            Opcode::COMMENT(com) => format!("# {}", com),
        }
    }

    /// to AT&T syntax.
    pub fn to_at_string(&self) -> String {
        match &self {
            // none
            Opcode::CWD | Opcode::CDQ | Opcode::CQO | Opcode::RET | Opcode::ENDBR64 | Opcode::SYSCALL => {
                self.opcode_to_at().to_string()
            }
            Opcode::CALLFUNC(func) => format!("{} {}", self.opcode_to_at(), func.to_at_string()),
            Opcode::CMPRAXIMM32 { imm } => {
                format!("{} {}, %rax", self.opcode_to_at(), imm.to_at_string())
            }
            Opcode::JMPLABEL { label } => format!("{} {}", self.opcode_to_at(), label),
            Opcode::JELABEL { label } => format!("{} {}", self.opcode_to_at(), label),
            Opcode::LEAR64M {
                r64,
                m,
            } => format!(
                "{} {}, {}",
                self.opcode_to_at(),
                m.to_at_string(),
                r64.to_at_string()
            ),

            // r64
            Opcode::POPR64 { r64 } | Opcode::PUSHR64 { r64 } => {
                format!("{} {}", self.opcode_to_at(), r64.to_at_string())
            }

            // imm32
            Opcode::PUSHIMM32 { imm } => format!("{} {}", self.opcode_to_at(), imm.to_at_string()),

            // r/m64
            Opcode::IDIVRM64 { rm64 }
            | Opcode::INCRM64 { rm64 }
            | Opcode::PUSHRM64 { rm64 }
            | Opcode::NEGRM64 { rm64 } => {
                format!("{} {}", self.opcode_to_at(), rm64.to_at_string())
            }

            // r32, r/m32
            Opcode::ADDR32RM32{r32, rm32}
            | Opcode::MOVR32RM32{r32, rm32} => format!(
                "{} {}, {}",
                self.opcode_to_at(),
                rm32.to_at_string(),
                r32.to_at_string()
            ),

            // r64, r/m64
            Opcode::ADDR64RM64 { r64, rm64 }
            | Opcode::IMULR64RM64 { r64, rm64 }
            | Opcode::SUBR64RM64 { r64, rm64 }
            | Opcode::MOVR64RM64 { r64, rm64 } => format!(
                "{} {}, {}",
                self.opcode_to_at(),
                rm64.to_at_string(),
                r64.to_at_string()
            ),

            // r/m8, r8
            Opcode::MOVRM8R8 { rm8, r8 } => format!(
                "{} {}, {}",
                self.opcode_to_at(),
                r8.to_at_string(),
                rm8.to_at_string()
            ),

            // r/m32, r32
            Opcode::ADDRM32R32{rm32, r32}
            | Opcode::MOVRM32R32{rm32, r32} => format!(
                "{} {}, {}",
                self.opcode_to_at(),
                r32.to_at_string(),
                rm32.to_at_string()
            ),

            // r/m64, r64
            Opcode::ADDRM64R64 { rm64, r64 }
            | Opcode::MOVRM64R64 { rm64, r64 }
            | Opcode::SUBRM64R64 { rm64, r64 } => format!(
                "{} {}, {}",
                self.opcode_to_at(),
                r64.to_at_string(),
                rm64.to_at_string()
            ),

            // r/m32, imm32
            Opcode::MOVRM32IMM32 { rm32, imm } => format!(
                "{} {}, {}",
                self.opcode_to_at(),
                imm.to_at_string(),
                rm32.to_at_string()
            ),


            // r/m64, imm32
            Opcode::MOVRM64IMM32 { rm64, imm } | Opcode::SUBRM64IMM32 { rm64, imm } => format!(
                "{} {}, {}",
                self.opcode_to_at(),
                imm.to_at_string(),
                rm64.to_at_string()
            ),

            // etc
            Opcode::COMMENT(com) => format!("# {}", com),
        }
    }

    fn opcode_to_intel(&self) -> &str {
        match &self {
            // Add
            Opcode::ADDRM32R32{rm32: _, r32: _} 
            | Opcode::ADDR32RM32{rm32: _, r32: _} 
            | Opcode::ADDRM64R64 { rm64: _, r64: _ } 
            | Opcode::ADDR64RM64 { r64: _, rm64: _ } => {
                "add"
            }

            // Call
            Opcode::CALLFUNC(_func) => "call",

            // Compare Two Operands
            Opcode::CMPRAXIMM32 { imm: _ } => "cmp",

            // Jump
            Opcode::JMPLABEL { label: _ } => "jmp",
            Opcode::JELABEL { label: _ } => "je",

            // Load Effective Address
            Opcode::LEAR64M {
                r64: _,
                m: _,
            } => "lea",

            // none
            Opcode::CWD => "cwd",
            Opcode::CDQ => "cdq",
            Opcode::CQO => "cqo",
            Opcode::ENDBR64 => "endbr64",

            // (signed) Integer Divide
            Opcode::IDIVRM64 { rm64: _ } => "idiv",

            // (signed) Integer Multiply
            Opcode::IMULR64RM64 { r64: _, rm64: _ } => "imul",

            // Increment
            Opcode::INCRM64 { rm64: _ } => "inc",

            // Move
            Opcode::MOVRM8R8 { rm8: _, r8: _ }
            | Opcode::MOVRM32R32 { rm32: _, r32: _ }
            | Opcode::MOVR32RM32 { r32: _, rm32: _ }
            | Opcode::MOVRM32IMM32 { rm32: _, imm: _ }
            | Opcode::MOVRM64R64 { rm64: _, r64: _ }
            | Opcode::MOVR64RM64 { r64: _, rm64: _ }
            | Opcode::MOVRM64IMM32 { rm64: _, imm: _ } => "mov",

            // Neg
            Opcode::NEGRM64 { rm64: _ } => "neg",

            // Pop
            Opcode::POPR64 { r64: _ } => "pop",

            // Push
            Opcode::PUSHRM64 { rm64: _ }
            | Opcode::PUSHR64 { r64: _ }
            | Opcode::PUSHIMM32 { imm: _ } => "push",

            // Return from procedure
            Opcode::RET => "ret",

            // Sub
            Opcode::SUBRM64IMM32 { rm64: _, imm: _ }
            | Opcode::SUBR64RM64 { r64: _, rm64: _ }
            | Opcode::SUBRM64R64 { rm64: _, r64: _ } => "sub",

            // Fast System Call
            Opcode::SYSCALL => "syscall",

            // etc
            Opcode::COMMENT(_com) => "",
        }
    }

    fn opcode_to_at(&self) -> &str {
        match &self {
            // Add
            Opcode::ADDRM32R32 { rm32: _, r32: _ } 
            | Opcode::ADDR32RM32 { r32: _, rm32: _ } => {
                "addl"
            }
            Opcode::ADDRM64R64 { rm64: _, r64: _ }
             | Opcode::ADDR64RM64 { r64: _, rm64: _ } => {
                "addq"
            }

            // Call
            Opcode::CALLFUNC(_func) => "call",

            // Compare Two Operands
            Opcode::CMPRAXIMM32 { imm: _ } => "cmpq",

            // Jump
            Opcode::JMPLABEL { label: _ } => "jmp",
            Opcode::JELABEL { label: _ } => "je",

            // Load Effective Address
            Opcode::LEAR64M {
                r64: _,
                m: _,
            } => "leaq",

            // none
            Opcode::CWD => "cwtd",
            Opcode::CDQ => "cltd",
            Opcode::CQO => "cqto",
            Opcode::ENDBR64 => "endbr64",

            // (signed) Integer Divide
            Opcode::IDIVRM64 { rm64: _ } => "idivq",

            // (signed) Integer Multiply
            Opcode::IMULR64RM64 { r64: _, rm64: _ } => "imulq",

            // Increment
            Opcode::INCRM64 { rm64: _ } => "incq",

            // Move
            Opcode::MOVRM8R8 { rm8: _, r8: _ } => "movb",
            Opcode::MOVRM32R32 { rm32: _, r32: _ }
            | Opcode::MOVR32RM32 { r32: _, rm32: _ }
            | Opcode::MOVRM32IMM32 { rm32: _, imm: _ } => "movl",
            Opcode::MOVRM64R64 { rm64: _, r64: _ }
            | Opcode::MOVR64RM64 { r64: _, rm64: _ }
            | Opcode::MOVRM64IMM32 { rm64: _, imm: _ } => "movq",

            // Neg
            Opcode::NEGRM64 { rm64: _ } => "negq",

            // Pop
            Opcode::POPR64 { r64: _ } => "popq",

            // Push
            Opcode::PUSHRM64 { rm64: _ }
            | Opcode::PUSHR64 { r64: _ }
            | Opcode::PUSHIMM32 { imm: _ } => "pushq",

            // Return from procedure
            Opcode::RET => "ret",

            // Sub
            Opcode::SUBRM64IMM32 { rm64: _, imm: _ }
            | Opcode::SUBR64RM64 { r64: _, rm64: _ }
            | Opcode::SUBRM64R64 { rm64: _, r64: _ } => "subq",

            // Fast System Call
            Opcode::SYSCALL => "syscall",

            // etc
            Opcode::COMMENT(_com) => "",
        }
    }
}
