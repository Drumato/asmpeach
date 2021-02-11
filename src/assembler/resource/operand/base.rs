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
            Operand::Memory(mem) => write!(
                f,
                "Memory{{ base: {}, index: {:?}, disp: {:?}, scale: {:?} }}",
                mem.base, mem.index, mem.disp, mem.scale
            ),
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
    /// メモリオペランドの場合はbase-registerが拡張レジスタかどうかのチェックに用いる
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

                let mut addressing = mem.base.to_at_string();

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

    pub fn size(&self) -> OperandSize {
        match self {
            Operand::GeneralReg(gpr) => gpr.size(),
            Operand::Memory(mem) => mem.base.size(),
            Operand::Label(_label) => unreachable!(),
            Operand::Immediate(imm) => match imm {
                Immediate::I8(_v) => OperandSize::Byte,
                Immediate::I16(_v) => OperandSize::Word,
                Immediate::I32(_v) => OperandSize::Dword,
                Immediate::I64(_v) => OperandSize::Qword,
            },
        }
    }
}
