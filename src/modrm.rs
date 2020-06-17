use crate::{Operand};

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

    /// new MR Encoding.
    pub fn new_mr(mode: AddressingMode, rm: &Operand, reg: &Operand) -> Self {
        let rm_byte = if rm.req_sib_byte() {
            0x04
        } else {
            rm.number()
        };
        Self {
            mode,
            rm: Self::rm_field(rm_byte),
            reg: Self::reg_field(reg.number()),
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