use crate::assembler::resource::{
    AddressingMode, Displacement, GeneralPurposeRegister, Immediate, RegisterSize, SIBByte,
};

#[allow(dead_code)]
#[derive(Eq, Ord, PartialOrd, PartialEq, Debug, Clone)]
pub enum Operand {
    // register operands
    GENERALREGISTER(GeneralPurposeRegister),
    // SEGMENT,
    // FLAGS,
    // X87FPU
    // MMX
    // XMM
    // CONTROL
    /// memory addressing
    /// ex. [rax], -4[rbp]
    ADDRESSING {
        base: GeneralPurposeRegister,
        index: Option<GeneralPurposeRegister>,
        disp: Option<Displacement>,
        scale: Option<u8>,
    },

    /// label in assembly code.
    /// using label operand in jump-related instructions.
    LABEL(String),
    Immediate(Immediate),
}

#[allow(dead_code)]
impl Operand {
    /// ラベルの文字列を取得
    pub fn copy_label(&self) -> String {
        match self {
            Operand::LABEL(contents) => contents.to_string(),
            _ => unimplemented!(),
        }
    }

    /// メモリアドレッシングかチェック
    pub fn is_addressing(&self) -> bool {
        match self {
            Operand::ADDRESSING {
                base: _,
                index: _,
                disp: _,
                scale: _,
            } => true,
            _ => false,
        }
    }
    /// 使用しているレジスタがx64拡張のものかチェック
    /// REX-Prefix の計算に使用
    pub fn is_expanded(&self) -> bool {
        match self {
            Operand::ADDRESSING {
                base: base_reg,
                index: _,
                disp: _,
                scale: _,
            } => base_reg.is_expanded(),
            Operand::GENERALREGISTER(gpr) => gpr.is_expanded(),
            _ => false,
        }
    }
    /// メモリアドレッシングのindex-regがx64拡張のものかチェック
    /// REX-Prefix のx_bitの計算に使用
    pub fn index_reg_is_expanded(&self) -> bool {
        match self {
            Operand::ADDRESSING {
                base: _,
                index: index_reg,
                disp: _,
                scale: _,
            } => {
                if index_reg.is_none() {
                    return false;
                }

                index_reg.unwrap().is_expanded()
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

        let (base, index, _disp, scale) = self.get_addressing();

        if let Some(scale_byte) = scale {
            Some(SIBByte {
                base_reg: base.number(),
                index_reg: index.unwrap().number(),
                scale: scale_byte,
            })
        } else {
            Some(SIBByte {
                base_reg: base.number(),
                index_reg: index.unwrap().number(),
                scale: 0,
            })
        }
    }

    /// displacementを取得
    /// コード生成に使用
    pub fn get_displacement(&self) -> Option<Displacement> {
        if !self.is_addressing() {
            return None;
        }

        let (_base, _index, disp, _scale) = self.get_addressing();
        disp
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
            Operand::ADDRESSING {
                base: _,
                index: index_reg,
                disp: _,
                scale: _,
            } => index_reg.is_some(),

            _ => false,
        }
    }

    /// register code
    /// レジスタ番号の取得
    pub fn number(&self) -> u8 {
        match self {
            Self::GENERALREGISTER(reg) => reg.number(),
            Self::ADDRESSING {
                base: base_reg,
                index: _,
                disp: _,
                scale: _,
            } => base_reg.number(),
            _ => panic!("cannot get register-number from {:?}", self),
        }
    }

    /// get addressing mode in ModRM:mode
    pub fn addressing_mode(&self) -> AddressingMode {
        match self {
            Operand::ADDRESSING {
                base: _,
                index: _,
                disp: displacement,
                scale: _,
            } => {
                if displacement.is_none() {
                    return AddressingMode::REGISTER;
                }

                let disp = displacement.unwrap();
                match disp {
                    Displacement::DISP8(_v8) => AddressingMode::DISP8,
                    Displacement::DISP32(_v32) => AddressingMode::DISP32,
                }
            }
            Operand::GENERALREGISTER(_reg) => AddressingMode::DIRECTREG,
            _ => panic!("cannot get addressing mode from {:?}", self),
        }
    }

    /// メモリアドレッシングの情報を取得する
    /// is_addressing() を先に呼ぶ必要がある
    pub fn get_addressing(
        &self,
    ) -> (
        GeneralPurposeRegister,
        Option<GeneralPurposeRegister>,
        Option<Displacement>,
        Option<u8>,
    ) {
        match self {
            Operand::ADDRESSING {
                base: base_reg,
                index: index_reg,
                disp: displacement,
                scale,
            } => (*base_reg, *index_reg, *displacement, *scale),
            _ => panic!(
                "cannot get addressing materials. check 'is_addressing()' before calling this."
            ),
        }
    }

    pub fn to_intel_string(&self) -> String {
        match self {
            Operand::GENERALREGISTER(gpr) => gpr.to_intel_string(),
            Operand::Immediate(imm) => imm.to_intel_string(),
            Operand::LABEL(s) => s.to_string(),
            Operand::ADDRESSING {
                base: base_reg,
                index: index_reg,
                disp: displacement,
                scale,
            } => {
                let size_ptr = match base_reg.size() {
                    RegisterSize::S8 => "BYTE PTR",
                    RegisterSize::S16 => "WORD PTR",
                    RegisterSize::S32 => "DWORD PTR",
                    RegisterSize::S64 => "QWORD PTR",
                };

                let mut addressing = if displacement.is_some() {
                    format!("{}[", displacement.unwrap().to_string())
                } else {
                    "[".to_string()
                };
                addressing += &base_reg.to_64bit().to_intel_string();

                if let Some(index) = index_reg {
                    addressing += &format!(" + {}", index.to_intel_string());
                }
                if let Some(s) = scale {
                    addressing += &format!(" * {}", s);
                }
                addressing += "]";

                format!("{} {}", size_ptr, addressing)
            }
        }
    }

    pub fn to_at_string(&self) -> String {
        match self {
            Operand::GENERALREGISTER(gpr) => gpr.to_at_string(),
            Operand::Immediate(imm) => imm.to_at_string(),
            Operand::LABEL(s) => s.to_string(),
            Operand::ADDRESSING {
                base: base_reg,
                index: index_reg,
                disp: displacement,
                scale,
            } => {
                let disp_str = if displacement.is_some() {
                    displacement.unwrap().to_string()
                } else {
                    String::new()
                };

                let mut addressing = base_reg.to_64bit().to_at_string();

                if let Some(index) = index_reg {
                    addressing += &format!(", {}", index.to_at_string());
                }
                if let Some(s) = scale {
                    addressing += &format!(", {}", s);
                }

                format!("{}({})", disp_str, addressing)
            }
        }
    }

    pub fn to_8bit(&self) -> Self {
        match self {
            Operand::GENERALREGISTER(gpr) => Operand::GENERALREGISTER(gpr.to_8bit()),
            Operand::Immediate(imm) => Operand::Immediate(imm.as_8bit()),
            Operand::ADDRESSING {
                base: b,
                index: i,
                disp: d,
                scale: s,
            } => Operand::ADDRESSING {
                base: b.to_8bit(),
                index: match i {
                    Some(ireg) => Some(ireg.to_8bit()),
                    None => None,
                },
                disp: *d,
                scale: *s,
            },
            Operand::LABEL(_label) => unreachable!(),
        }
    }
    pub fn to_16bit(&self) -> Self {
        match self {
            Operand::GENERALREGISTER(gpr) => Operand::GENERALREGISTER(gpr.to_16bit()),
            Operand::Immediate(imm) => Operand::Immediate(imm.as_16bit()),
            Operand::ADDRESSING {
                base: b,
                index: i,
                disp: d,
                scale: s,
            } => Operand::ADDRESSING {
                base: b.to_16bit(),
                index: match i {
                    Some(ireg) => Some(ireg.to_16bit()),
                    None => None,
                },
                disp: *d,
                scale: *s,
            },
            Operand::LABEL(_label) => unreachable!(),
        }
    }
    pub fn to_32bit(&self) -> Self {
        match self {
            Operand::GENERALREGISTER(gpr) => Operand::GENERALREGISTER(gpr.to_32bit()),
            Operand::Immediate(imm) => Operand::Immediate(imm.as_32bit()),
            Operand::ADDRESSING {
                base: b,
                index: i,
                disp: d,
                scale: s,
            } => Operand::ADDRESSING {
                base: b.to_32bit(),
                index: match i {
                    Some(ireg) => Some(ireg.to_32bit()),
                    None => None,
                },
                disp: *d,
                scale: *s,
            },
            Operand::LABEL(_label) => unreachable!(),
        }
    }

    pub fn to_64bit(&self) -> Self {
        match self {
            Operand::GENERALREGISTER(gpr) => Operand::GENERALREGISTER(gpr.to_64bit()),
            Operand::Immediate(imm) => Operand::Immediate(imm.as_32bit()),
            Operand::ADDRESSING {
                base: b,
                index: i,
                disp: d,
                scale: s,
            } => Operand::ADDRESSING {
                base: b.to_64bit(),
                index: match i {
                    Some(ireg) => Some(ireg.to_64bit()),
                    None => None,
                },
                disp: *d,
                scale: *s,
            },
            Operand::LABEL(_label) => unreachable!(),
        }
    }

    pub fn size(&self) -> OperandSize {
        match self {
            Operand::GENERALREGISTER(gpr) => match gpr.size() {
                RegisterSize::S8 => OperandSize::BYTE,
                RegisterSize::S16 => OperandSize::WORD,
                RegisterSize::S32 => OperandSize::DWORD,
                RegisterSize::S64 => OperandSize::QWORD,
            },
            Operand::ADDRESSING {
                base: base_reg,
                index: _,
                disp: _,
                scale: _,
            } => match base_reg.size() {
                RegisterSize::S8 => OperandSize::BYTE,
                RegisterSize::S16 => OperandSize::WORD,
                RegisterSize::S32 => OperandSize::DWORD,
                RegisterSize::S64 => OperandSize::QWORD,
            },
            Operand::LABEL(_label) => unreachable!(),
            Operand::Immediate(imm) => match imm {
                Immediate::I8(_v) => OperandSize::BYTE,
                Immediate::I16(_v) => OperandSize::WORD,
                Immediate::I32(_v) => OperandSize::DWORD,
            },
        }
    }
}

#[derive(Eq, Ord, PartialOrd, PartialEq, Debug, Clone, Copy)]
pub enum OperandSize {
    BYTE,
    WORD,
    DWORD,
    QWORD,
}
