use crate::{Displacement, Encoding, GeneralPurposeRegister, Immediate, ModRM, Operand, REXPrefix, SIBByte, AddressingMode};

#[derive(Eq, Ord, PartialOrd, PartialEq, Debug, Clone)]
pub enum Opcode {
    // Add
    /// Add r/m64 to r64
    ADDRM64R64 { rm64: Operand, r64: GeneralPurposeRegister },

    /// Add r64 to r/m64
    ADDR64RM64 { r64: GeneralPurposeRegister, rm64: Operand },

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

    // (signed) Integer Divide
    /// signed divide RDX:RAX by r/m64;
    /// result stored in RAX := Quotient, RDX := Remainder.
    IDIVRM64 { rm64: Operand },

    // (signed) Integer Multiply
    /// Quadword register := Quadword register * r/m64
    IMULR64RM64 { r64: GeneralPurposeRegister, rm64: Operand },

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
    /// スタティックリンクで使用される， `lea rax, .LS1` のような命令です．
    /// このopcodeに対し `to_bytes()` メソッドを呼び出すと，
    /// `[REX-Prefix, opcode, ModRM]` が返されます．
    /// 自作アセンブラでは
    /// `let mut base_bytes = inst.to_bytes(); base_bytes.append(&mut vec![0x25, 0x00, 0x00, 0x00, 0x00])` のように，
    /// データセグメントを用いるプリフィックスと，再配置でアドレスを書き込むためのパディングを追加するといいでしょう．
    /// そして再配置シンボルを定義して，リンク時に文字列のアドレスを書き込むといいと思います．
    LEAR64FROMSTRADDR { r64: GeneralPurposeRegister, str_sym: String, addend: usize },

    // Move
    /// Move r8 to r/m8
    MOVRM8R8 { r8: GeneralPurposeRegister, rm8: Operand },

    /// Move r64 to r/m64
    MOVRM64R64 { r64: GeneralPurposeRegister, rm64: Operand },

    /// Move r/m64 to r64
    MOVR64RM64 { r64: GeneralPurposeRegister, rm64: Operand },

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
    SUBR64RM64 { r64: GeneralPurposeRegister, rm64: Operand },

    /// Subtract r64 from r/m64
    SUBRM64R64 { rm64: Operand, r64: GeneralPurposeRegister },

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
            Opcode::ADDRM64R64 { rm64: _, r64: _ } => vec![0x01],
            Opcode::ADDR64RM64 { r64: _, rm64: _ } => vec![0x03],

            // Call
            Opcode::CALLFUNC(_func) => unimplemented!(),

            // Convert Word to Doubleword/Convert Doubleword to Quadword
            Opcode::CWD => vec![0x66, 0x99],
            Opcode::CDQ
            | Opcode::CQO => vec![0x99],

            Opcode::CMPRAXIMM32 { imm: _, } => vec![0x3d],

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
            Opcode::LEAR64FROMSTRADDR { r64: _, str_sym: _, addend: _ } => vec![0x8d],

            // Move
            Opcode::MOVRM8R8 { r8: _, rm8: _ } => vec![0x88],
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
            // Add
            Opcode::ADDRM64R64 { rm64: _, r64: _ } => Encoding::MR,
            Opcode::ADDR64RM64 { r64: _, rm64: _ } => Encoding::RM,

            // Call
            Opcode::CALLFUNC(_func) => unimplemented!(),

            // Convert Word to Doubleword/Convert Doubleword to Quadword
            Opcode::CWD
            | Opcode::CDQ
            | Opcode::CQO => Encoding::ZO,

            // Compare Two Operands
            Opcode::CMPRAXIMM32 { imm: _, } => Encoding::I,

            // (signed) Integer Divide
            Opcode::IDIVRM64 { rm64: _ } => Encoding::M,

            // (signed) Integer Multiply
            Opcode::IMULR64RM64 { r64: _, rm64: _ } => Encoding::RM,

            // Increment
            Opcode::INCRM64 { rm64: _ } => Encoding::M,

            // Jump
            Opcode::JMPLABEL { label: _ } => Encoding::D,
            Opcode::JELABEL { label: _ } => Encoding::D,

            // Load Effective Address
            Opcode::LEAR64FROMSTRADDR { r64: _, str_sym: _, addend: _ } => Encoding::RM,

            // Move
            Opcode::MOVRM8R8 { r8: _, rm8: _ } => Encoding::MR,
            Opcode::MOVRM64R64 { r64: _, rm64: _ } => Encoding::MR,
            Opcode::MOVR64RM64 { r64: _, rm64: _ } => Encoding::RM,
            Opcode::MOVRM64IMM32 { rm64: _, imm: _ } => Encoding::MI,

            // Neg
            Opcode::NEGRM64 { rm64: _ } => Encoding::M,

            // Pop
            Opcode::POPR64 { r64: _ } => Encoding::O,

            // Push
            Opcode::PUSHRM64 { rm64: _ } => Encoding::M,
            Opcode::PUSHR64 { r64: _ } => Encoding::O,
            Opcode::PUSHIMM32 { imm: _ } => Encoding::I,

            // Return from procedure
            Opcode::RET => Encoding::ZO,

            // Sub
            Opcode::SUBRM64IMM32 { rm64: _, imm: _ } => Encoding::MI,
            Opcode::SUBR64RM64 { r64: _, rm64: _ } => Encoding::RM,
            Opcode::SUBRM64R64 { rm64: _, r64: _ } => Encoding::MR,

            // Fast system call
            Opcode::SYSCALL => Encoding::ZO,

            // etc
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
            Opcode::CMPRAXIMM32 { imm: _, } => Some(REXPrefix::new(true, false, false, false)),

            // (signed) Integer Multiply
            Opcode::IMULR64RM64 { r64, rm64 } => {
                Some(REXPrefix::new_from_mem_and_reg(true, r64, rm64))
            }

            // (signed) Integer Divide
            Opcode::IDIVRM64 { rm64 } => {
                Some(REXPrefix::new_from_mem(true, rm64))
            }

            // Increment
            Opcode::INCRM64 { rm64 } => {
                Some(REXPrefix::new_from_mem(true, rm64))
            }

            // Load Effective Address
            Opcode::LEAR64FROMSTRADDR { r64, str_sym: _, addend: _ } => {
                Some(REXPrefix::new(true, false, false, r64.is_expanded()))
            }


            // Move
            Opcode::MOVRM64R64 { rm64, r64 } => {
                Some(REXPrefix::new_from_mem_and_reg(true, r64, rm64))
            }
            Opcode::MOVR64RM64 { r64, rm64 } => {
                Some(REXPrefix::new_from_mem_and_reg(true, r64, rm64))
            }
            Opcode::MOVRM64IMM32 { rm64, imm: _ } => {
                Some(REXPrefix::new_from_mem(true, rm64))
            }

            // Neg
            Opcode::NEGRM64 { rm64 } => {
                Some(REXPrefix::new_from_mem(true, rm64))
            }

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
            Opcode::SUBRM64IMM32 { rm64, imm: _ } => {
                Some(REXPrefix::new_from_mem(true, rm64))
            }
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
                Some(ModRM::new_mr(rm64.addressing_mode(), rm64, &GeneralPurposeRegister::new_64bit_from_code(7)))
            }

            // (signed) Integer Multiply
            Opcode::IMULR64RM64 { r64, rm64 } => {
                // RM
                Some(ModRM::new_rm(rm64.addressing_mode(), r64, rm64))
            }


            // Increment
            Opcode::INCRM64 { rm64 } => {
                // Mだけど /0 なのでマスク
                Some(ModRM::new_mr(rm64.addressing_mode(), rm64, &GeneralPurposeRegister::new_64bit_from_code(0)))
            }

            // Load Effective Address
            Opcode::LEAR64FROMSTRADDR { r64, str_sym: _, addend: _ } => {
                Some(ModRM::new_rm(AddressingMode::REGISTER, r64, &Operand::GENERALREGISTER(GeneralPurposeRegister::new_64bit_from_code(4))))
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
                Some(ModRM::new_mr(rm64.addressing_mode(), rm64, &GeneralPurposeRegister::new_64bit_from_code(3)))
            }

            // Pop

            // Push
            Opcode::PUSHRM64 { rm64 } => {
                // Mだけど /6 でマスクするのでmr
                Some(ModRM::new_mr(rm64.addressing_mode(), rm64, &GeneralPurposeRegister::new_64bit_from_code(6)))
            }

            // Sub
            Opcode::SUBRM64IMM32 { rm64, imm: _ } => {
                // MIだけど /5 でマスクするのでmr
                Some(ModRM::new_mr(rm64.addressing_mode(), rm64, &GeneralPurposeRegister::new_64bit_from_code(5)))
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

            // (signed) Integer Divide
            Opcode::IDIVRM64 { rm64 } => rm64.get_displacement(),

            // (signed) Integer Multiply
            Opcode::IMULR64RM64 { r64: _, rm64 } => rm64.get_displacement(),

            // Increment
            Opcode::INCRM64 { rm64 } => rm64.get_displacement(),


            // Move
            Opcode::MOVRM8R8 { rm8, r8: _ } => rm8.get_displacement(),
            Opcode::MOVRM64R64 { rm64, r64: _ } => rm64.get_displacement(),

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
            Opcode::CMPRAXIMM32 { imm, } => Some(*imm),

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

            // (signed) Integer Divide
            Opcode::IDIVRM64 { rm64 } => rm64.sib_byte(),

            // (signed) Integer Multiply
            Opcode::IMULR64RM64 { r64: _, rm64 } => rm64.sib_byte(),

            // Increment
            Opcode::INCRM64 { rm64 } => rm64.sib_byte(),

            // Move
            Opcode::MOVRM8R8 { rm8, r8: _ } => rm8.sib_byte(),
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
            // none
            Opcode::CWD
            | Opcode::CDQ
            | Opcode::CQO
            | Opcode::RET
            | Opcode::SYSCALL => self.opcode_to_intel().to_string(),

            // Call
            Opcode::CALLFUNC(func) => format!("{} {}", self.opcode_to_intel(), func.to_intel_string()),


            // Compare Two Operands
            Opcode::CMPRAXIMM32 { imm, } => format!("{} rax, {}", self.opcode_to_intel(), imm.to_intel_string()),

            // Jump
            Opcode::JMPLABEL { label } => format!("{} {}", self.opcode_to_intel(), label),
            Opcode::JELABEL { label } => format!("{} {}", self.opcode_to_intel(), label),

            // Load Effective Address
            Opcode::LEAR64FROMSTRADDR { r64, str_sym, addend: _ } => {
                format!("{} {}, {}", self.opcode_to_intel(), str_sym, r64.to_intel_string())
            }

            // r64
            Opcode::POPR64 { r64 }
            | Opcode::PUSHR64 { r64 } => format!("{} {}", self.opcode_to_intel(), r64.to_intel_string()),

            // imm32
            Opcode::PUSHIMM32 { imm } => format!("{} {}", self.opcode_to_intel(), imm.to_intel_string()),

            // r/m64
            Opcode::IDIVRM64 { rm64 }
            | Opcode::INCRM64 { rm64 }
            | Opcode::PUSHRM64 { rm64 }
            | Opcode::NEGRM64 { rm64 } => {
                format!("{} {}", self.opcode_to_intel(), rm64.to_intel_string())
            }


            // r64, r/m64
            Opcode::ADDR64RM64 { r64, rm64 }
            | Opcode::IMULR64RM64 { r64, rm64 }
            | Opcode::SUBR64RM64 { r64, rm64 }
            | Opcode::MOVR64RM64 { r64, rm64 } => {
                format!("{} {}, {}", self.opcode_to_intel(), r64.to_intel_string(), rm64.to_intel_string())
            }

            // r/m8, r8
            Opcode::MOVRM8R8 { rm8, r8 } => {
                format!("{} {}, {}", self.opcode_to_intel(), rm8.to_intel_string(), r8.to_intel_string())
            }

            // r/m64, r64
            Opcode::ADDRM64R64 { rm64, r64 }
            | Opcode::MOVRM64R64 { rm64, r64 }
            | Opcode::SUBRM64R64 { rm64, r64 } => {
                format!("{} {}, {}", self.opcode_to_intel(), rm64.to_intel_string(), r64.to_intel_string())
            }

            // r/m64, imm32
            Opcode::MOVRM64IMM32 { rm64, imm }
            | Opcode::SUBRM64IMM32 { rm64, imm } => {
                format!("{} {}, {}", self.opcode_to_intel(), rm64.to_intel_string(), imm.to_intel_string())
            }

            // etc
            Opcode::COMMENT(com) => format!("# {}", com),
        }
    }

    /// to AT&T syntax.
    pub fn to_at_string(&self) -> String {
        match &self {
            // none
            Opcode::CWD
            | Opcode::CDQ
            | Opcode::CQO
            | Opcode::RET
            | Opcode::SYSCALL => self.opcode_to_at().to_string(),

            // Call
            Opcode::CALLFUNC(func) => format!("{} {}", self.opcode_to_at(), func.to_at_string()),

            // Compare Two Operands
            Opcode::CMPRAXIMM32 { imm, } => format!("{} {}, %rax", self.opcode_to_at(), imm.to_at_string()),

            // Jump
            Opcode::JMPLABEL { label } => format!("{} {}", self.opcode_to_at(), label),
            Opcode::JELABEL { label } => format!("{} {}", self.opcode_to_at(), label),

            // Load Effective Address
            Opcode::LEAR64FROMSTRADDR { r64, str_sym, addend: _ } => {
                format!("{} {}, {}", self.opcode_to_at(), str_sym, r64.to_at_string())
            }

            // r64
            Opcode::POPR64 { r64 }
            | Opcode::PUSHR64 { r64 } => format!("{} {}", self.opcode_to_at(), r64.to_at_string()),

            // imm32
            Opcode::PUSHIMM32 { imm } => format!("{} {}", self.opcode_to_at(), imm.to_at_string()),

            // r/m64
            Opcode::IDIVRM64 { rm64 }
            | Opcode::INCRM64 { rm64 }
            | Opcode::PUSHRM64 { rm64 }
            | Opcode::NEGRM64 { rm64 } => {
                format!("{} {}", self.opcode_to_at(), rm64.to_at_string())
            }


            // r64, r/m64
            Opcode::ADDR64RM64 { r64, rm64 }
            | Opcode::IMULR64RM64 { r64, rm64 }
            | Opcode::SUBR64RM64 { r64, rm64 }
            | Opcode::MOVR64RM64 { r64, rm64 } => {
                format!("{} {}, {}", self.opcode_to_at(), rm64.to_at_string(), r64.to_at_string())
            }

            // r/m8, r8
            Opcode::MOVRM8R8 { rm8, r8 } => {
                format!("{} {}, {}", self.opcode_to_at(), r8.to_at_string(), rm8.to_at_string())
            }

            // r/m64, r64
            Opcode::ADDRM64R64 { rm64, r64 }
            | Opcode::MOVRM64R64 { rm64, r64 }
            | Opcode::SUBRM64R64 { rm64, r64 } => {
                format!("{} {}, {}", self.opcode_to_at(), r64.to_at_string(), rm64.to_at_string())
            }

            // r/m64, imm32
            Opcode::MOVRM64IMM32 { rm64, imm }
            | Opcode::SUBRM64IMM32 { rm64, imm } => {
                format!("{} {}, {}", self.opcode_to_at(), imm.to_at_string(), rm64.to_at_string())
            }

            // etc
            Opcode::COMMENT(com) => format!("# {}", com),
        }
    }

    fn opcode_to_intel(&self) -> &str {
        match &self {
            // Add
            Opcode::ADDRM64R64 { rm64: _, r64: _ }
            | Opcode::ADDR64RM64 { r64: _, rm64: _ } => "add",


            // Call
            Opcode::CALLFUNC(_func) => "call",

            // Compare Two Operands
            Opcode::CMPRAXIMM32 { imm: _, } => "cmp",

            // Jump
            Opcode::JMPLABEL { label: _ } => "jmp",
            Opcode::JELABEL { label: _ } => "je",

            // Load Effective Address
            Opcode::LEAR64FROMSTRADDR { r64: _, str_sym: _, addend: _ } => "lea",


            // none
            Opcode::CWD => "cwd",
            Opcode::CDQ => "cdq",
            Opcode::CQO => "cqo",

            // (signed) Integer Divide
            Opcode::IDIVRM64 { rm64: _ } => "idiv",

            // (signed) Integer Multiply

            Opcode::IMULR64RM64 { r64: _, rm64: _ } => "imul",


            // Increment
            Opcode::INCRM64 { rm64: _ } => "inc",

            // Move
            Opcode::MOVRM8R8 { rm8: _, r8: _ }
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
            Opcode::ADDRM64R64 { rm64: _, r64: _ }
            | Opcode::ADDR64RM64 { r64: _, rm64: _ } => "addq",

            // Call
            Opcode::CALLFUNC(_func) => "call",

            // Compare Two Operands
            Opcode::CMPRAXIMM32 { imm: _, } => "cmpq",

            // Jump
            Opcode::JMPLABEL { label: _ } => "jmp",
            Opcode::JELABEL { label: _ } => "je",

            // Load Effective Address
            Opcode::LEAR64FROMSTRADDR { r64: _, str_sym: _, addend: _ } => "leaq",

            // none
            Opcode::CWD => "cwtd",
            Opcode::CDQ => "cltd",
            Opcode::CQO => "cqto",

            // (signed) Integer Divide
            Opcode::IDIVRM64 { rm64: _ } => "idivq",

            // (signed) Integer Multiply

            Opcode::IMULR64RM64 { r64: _, rm64: _ } => "imulq",

            // Increment
            Opcode::INCRM64 { rm64: _ } => "incq",

            // Move
            Opcode::MOVRM8R8 { rm8: _, r8: _ } => "movb",
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
