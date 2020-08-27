use crate::assembler::resource::{GeneralPurposeRegister, Operand};

#[derive(Eq, Ord, PartialOrd, PartialEq, Debug, Clone, Copy)]
pub struct ModRM {
    mode: AddressingMode,
    reg: u8,
    rm: u8,
}

impl ModRM {
    pub fn to_byte(&self) -> u8 {
        Self::mode_field(self.mode.to_byte()) | self.reg | self.rm
    }

    /// new MI Encoding.
    pub fn new_mi(mode: AddressingMode, rm: &Operand) -> Self {
        let rm_byte = if rm.req_sib_byte() { 0x04 } else { rm.number() & 0b111 };
        Self {
            mode,
            rm: Self::rm_field(rm_byte),
            reg: 0,
        }
    }
    /// new MR Encoding.
    pub fn new_mr(mode: AddressingMode, rm: &Operand, reg: &GeneralPurposeRegister) -> Self {
        let rm_byte = if rm.req_sib_byte() { 0x04 } else { rm.number() & 0b111};
        Self {
            mode,
            rm: Self::rm_field(rm_byte),
            reg: Self::reg_field(reg.number()),
        }
    }
    /// new RM Encoding.
    pub fn new_rm(mode: AddressingMode, reg: &GeneralPurposeRegister, rm: &Operand) -> Self {
        let rm_byte = if rm.req_sib_byte() { 0x04 } else { rm.number() & 0b111};
        Self {
            mode,
            rm: Self::rm_field(rm_byte),
            reg: Self::reg_field(reg.number()& 0b111),
        }
    }
    pub fn mode_field(byte: u8) -> u8 {
        byte << 6
    }
    pub fn rm_field(byte: u8) -> u8 {
        byte
    }
    pub fn reg_field(byte: u8) -> u8 {
        byte << 3
    }
}

#[derive(Eq, Ord, PartialOrd, PartialEq, Debug, Clone, Copy)]
pub enum AddressingMode {
    /// [reg + disp8]
    DISP8,
    /// [reg + disp16/32]
    DISP32,
    /// [reg + reg]
    REGISTER,
    /// reg
    DIRECTREG,
}

impl AddressingMode {
    pub fn to_byte(&self) -> u8 {
        match self {
            Self::REGISTER => 0b00,
            Self::DISP8 => 0b01,
            Self::DISP32 => 0b10,
            Self::DIRECTREG => 0b11,
        }
    }
}
