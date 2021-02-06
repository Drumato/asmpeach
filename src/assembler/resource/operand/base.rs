use crate::assembler::resource::{
    AddressingMode, Displacement, GeneralPurposeRegister, Immediate, SIBByte,
};

#[allow(dead_code)]
#[derive(Eq, Ord, PartialOrd, PartialEq, Debug, Clone)]
pub enum Operand {
    // register operands
    GeneralReg(GeneralPurposeRegister),
    // SEGMENT,
    // FLAGS,
    // X87FPU
    // MMX
    // XMM
    // CONTROL
    /// memory addressing
    /// ex. [rax], -4[rbp]
    Memory(OpMemory),
    /// label in assembly code.
    /// using label operand in jump-related instructions.
    Label(String),
    Immediate(Immediate),
}
#[derive(Eq, Ord, PartialOrd, PartialEq, Debug, Clone)]
pub struct OpMemory {
    pub base: GeneralPurposeRegister,
    pub index: Option<GeneralPurposeRegister>,
    pub disp: Option<Displacement>,
    pub scale: Option<u8>,
}
#[derive(Eq, Ord, PartialOrd, PartialEq, Debug, Clone, Copy)]
pub enum OperandSize {
    Byte,
    Word,
    Dword,
    Qword,
}

impl std::fmt::Display for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operand::GeneralReg(gpr) => write!(f, "{}", gpr),
            Operand::Memory(mem) => write!(f, "{:?}", mem),
            Operand::Label(l) => write!(f, "{}", l),
            Operand::Immediate(v) => write!(f, "{}", v),
        }
    }
}

#[allow(dead_code)]
impl Operand {
    /// ラベルの文字列を取得
    pub fn copy_label(&self) -> String {
        match self {
            Operand::Label(contents) => contents.to_string(),
            _ => unimplemented!(),
        }
    }

    /// メモリアドレッシングかチェック
    pub fn is_addressing(&self) -> bool {
        match self {
            Operand::Memory(_) => true,
            _ => false,
        }
    }
    /// 使用しているレジスタがx64拡張のものかチェック
    /// REX-Prefix の計算に使用
    pub fn is_expanded(&self) -> bool {
        match self {
            Operand::Memory(mem) => mem.base.is_expanded(),
            Operand::GeneralReg(gpr) => gpr.is_expanded(),
            _ => false,
        }
    }
    /// メモリアドレッシングのindex-regがx64拡張のものかチェック
    /// REX-Prefix のx_bitの計算に使用
    pub fn index_reg_is_expanded(&self) -> bool {
        match self {
            Operand::Memory(mem) => {
                if mem.index.is_none() {
                    return false;
                }

                mem.index.unwrap().is_expanded()
            }

            _ => false,
        }
    }

    /// SIB-Byteが必要でなければNone,そうでなければSIBByteが返る
    /// コード生成に使用
    pub fn sib_byte(&self) -> Option<SIBByte> {
        if !self.req_sib_byte() {
            return None;
        }

        if let Operand::Memory(mem) = self {
            let scale = mem.scale.unwrap_or(0);
            return Some(SIBByte {
                base_reg: mem.base.number(),
                index_reg: mem.index.unwrap().number(),
                scale,
            });
        }

        unreachable!()
    }

    /// displacementを取得
    /// コード生成に使用
    pub fn displacement(&self) -> Option<Displacement> {
        if !self.is_addressing() {
            return None;
        }

        if let Operand::Memory(mem) = self {
            return mem.disp;
        }

        unreachable!()
    }

    /// immediateを取得
    /// コード生成に使用
    pub fn get_immediate(&self) -> Option<Immediate> {
        match self {
            Operand::Immediate(imm) => Some(*imm),
            _ => None,
        }
    }

    /// SIB-Byteを必要とするかチェック
    pub fn req_sib_byte(&self) -> bool {
        match self {
            Operand::Memory(mem) => mem.index.is_some(),
            _ => false,
        }
    }

    /// register code
    /// レジスタ番号の取得
    pub fn number(&self) -> u8 {
        match self {
            Self::GeneralReg(reg) => reg.number(),
            Self::Memory(mem) => mem.base.number(),
            _ => panic!("cannot get register-number from {}", self),
        }
    }

    /// get addressing mode in ModRM:mode
    pub fn addressing_mode(&self) -> AddressingMode {
        match self {
            Operand::Memory(mem) => {
                if mem.disp.is_none() {
                    return AddressingMode::REGISTER;
                }

                let disp = mem.disp.unwrap();
                match disp {
                    Displacement::Disp8(_v8) => AddressingMode::DISP8,
                    Displacement::Disp32(_v32) => AddressingMode::DISP32,
                }
            }
            Operand::GeneralReg(_reg) => AddressingMode::DIRECTREG,
            _ => panic!("cannot get addressing mode from {:?}", self),
        }
    }

    pub fn to_intel_string(&self) -> String {
        match self {
            Operand::GeneralReg(gpr) => gpr.to_intel_string(),
            Operand::Immediate(imm) => imm.to_intel_string(),
            Operand::Label(s) => s.to_string(),
            Operand::Memory(mem) => {
                let size_ptr = match mem.base.size() {
                    OperandSize::Byte => "BYTE PTR",
                    OperandSize::Word => "WORD PTR",
                    OperandSize::Dword => "DWORD PTR",
                    OperandSize::Qword => "QWORD PTR",
                };

                let mut addressing = if mem.disp.is_some() {
                    format!("{}[", mem.disp.unwrap().to_string())
                } else {
                    "[".to_string()
                };
                addressing += &mem.base.to_64bit().to_intel_string();

                if let Some(index) = mem.index {
                    addressing += &format!(" + {}", index.to_intel_string());
                }
                if let Some(s) = mem.scale {
                    addressing += &format!(" * {}", s);
                }
                addressing += "]";

                format!("{} {}", size_ptr, addressing)
            }
        }
    }

    pub fn to_at_string(&self) -> String {
        match self {
            Operand::GeneralReg(gpr) => gpr.to_at_string(),
            Operand::Immediate(imm) => imm.to_at_string(),
            Operand::Label(s) => s.to_string(),
            Operand::Memory(mem) => {
                let disp_str = if mem.disp.is_some() {
                    mem.disp.unwrap().to_string()
                } else {
                    String::new()
                };

                let mut addressing = mem.base.to_64bit().to_at_string();

                if let Some(index) = mem.index {
                    addressing += &format!(", {}", index.to_at_string());
                }
                if let Some(s) = mem.scale {
                    addressing += &format!(", {}", s);
                }

                format!("{}({})", disp_str, addressing)
            }
        }
    }

    pub fn to_8bit(&self) -> Self {
        match self {
            Operand::GeneralReg(gpr) => Operand::GeneralReg(gpr.to_8bit()),
            Operand::Immediate(imm) => Operand::Immediate(imm.as_8bit()),
            Operand::Memory(mem) => Operand::Memory(OpMemory {
                base: mem.base.to_8bit(),
                index: match mem.index {
                    Some(ireg) => Some(ireg.to_8bit()),
                    None => None,
                },
                disp: mem.disp,
                scale: mem.scale,
            }),
            Operand::Label(_label) => unreachable!(),
        }
    }
    pub fn to_16bit(&self) -> Self {
        match self {
            Operand::GeneralReg(gpr) => Operand::GeneralReg(gpr.to_16bit()),
            Operand::Immediate(imm) => Operand::Immediate(imm.as_16bit()),
            Operand::Memory(mem) => Operand::Memory(OpMemory {
                base: mem.base.to_16bit(),
                index: match mem.index {
                    Some(ireg) => Some(ireg.to_16bit()),
                    None => None,
                },
                disp: mem.disp,
                scale: mem.scale,
            }),
            Operand::Label(_label) => unreachable!(),
        }
    }
    pub fn to_32bit(&self) -> Self {
        match self {
            Operand::GeneralReg(gpr) => Operand::GeneralReg(gpr.to_32bit()),
            Operand::Immediate(imm) => Operand::Immediate(imm.as_32bit()),
            Operand::Memory(mem) => Operand::Memory(OpMemory {
                base: mem.base.to_32bit(),
                index: match mem.index {
                    Some(ireg) => Some(ireg.to_32bit()),
                    None => None,
                },
                disp: mem.disp,
                scale: mem.scale,
            }),
            Operand::Label(_label) => unreachable!(),
        }
    }

    pub fn to_64bit(&self) -> Self {
        match self {
            Operand::GeneralReg(gpr) => Operand::GeneralReg(gpr.to_64bit()),
            Operand::Immediate(imm) => Operand::Immediate(imm.as_32bit()),
            Operand::Memory(mem) => Operand::Memory(OpMemory {
                base: mem.base.to_64bit(),
                index: match mem.index {
                    Some(ireg) => Some(ireg.to_64bit()),
                    None => None,
                },
                disp: mem.disp,
                scale: mem.scale,
            }),
            Operand::Label(_label) => unreachable!(),
        }
    }

    pub fn size(&self) -> OperandSize {
        match self {
            Operand::GeneralReg(gpr) => gpr.size(),
            Operand::Memory(mem) => mem.base.size(),
            Operand::Label(_label) => unreachable!(),
            Operand::Immediate(imm) => match imm {
                Immediate::I8(_v) => OperandSize::Byte,
                Immediate::I16(_v) => OperandSize::Word,
                Immediate::I32(_v) => OperandSize::Dword,
            },
        }
    }
}
