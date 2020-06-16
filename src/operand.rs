
use crate::{GeneralPurposeRegister, SIBByte, Immediate, AddressingMode, Displacement};

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


    ADDRESSING {
        base_reg: GeneralPurposeRegister,
        index_reg: Option<GeneralPurposeRegister>,
        displacement: Option<Displacement>,
        scale: Option<u8>,
    },
    LABEL(String),
    Immediate(Immediate),
}

impl Operand {
    /// メモリアドレッシングかチェック
    pub fn is_addressing(&self) -> bool {
        match self {
            Operand::ADDRESSING { base_reg: _, index_reg: _, displacement: _, scale: _ } => true,
            _ => false,
        }
    }
    /// 使用しているレジスタがx64拡張のものかチェック
    /// REX-Prefix の計算に使用
    pub fn is_expanded(&self) -> bool {
        match self {
            Operand::ADDRESSING { base_reg: _, index_reg, displacement: _, scale: _ } => {
                if index_reg.is_none() {
                    return false;
                }

                index_reg.unwrap().is_expanded()
            }

            _ => false,
        }
    }
    /// メモリアドレッシングのindex-regがx64拡張のものかチェック
    /// REX-Prefix のx_bitの計算に使用
    pub fn index_reg_is_expanded(&self) -> bool {
        match self {
            Operand::ADDRESSING { base_reg: _, index_reg, displacement: _, scale: _ } => {
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
            Operand::ADDRESSING { base_reg: _, index_reg, displacement: _, scale: _ } => {
                index_reg.is_some()
            }

            _ => false,
        }
    }

    /// レジスタ番号の取得
    pub fn number(&self) -> u8 {
        match self {
            Self::GENERALREGISTER(reg) => reg.number(),
            Self::ADDRESSING { base_reg, index_reg: _, displacement: _, scale: _ } => base_reg.number(),
            _ => panic!("cannot get register-number from {:?}", self),
        }
    }

    pub fn addressing_mode(&self) -> AddressingMode {
        match self {
            Operand::ADDRESSING { base_reg: _, index_reg: _, displacement, scale: _ } => {
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
    pub fn get_addressing(&self) -> (GeneralPurposeRegister, Option<GeneralPurposeRegister>, Option<Displacement>, Option<u8>) {
        match self {
            Operand::ADDRESSING { base_reg, index_reg, displacement, scale } => (*base_reg, *index_reg, *displacement, *scale),
            _ => panic!("cannot get addressing materials. check 'is_addressing()' before calling this."),
        }
    }
}