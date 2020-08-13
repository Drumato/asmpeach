use fmt::Formatter;
use std::fmt;

/// for using relative-addressing
#[derive(Eq, Ord, PartialOrd, PartialEq, Clone, Copy)]
pub struct SIBByte {
    pub base_reg: u8,
    pub index_reg: u8,
    pub scale: u8,
}

impl SIBByte {
    pub fn base_field(byte: u8) -> u8 {
        byte
    }
    pub fn index_field(byte: u8) -> u8 {
        byte << 3
    }
    pub fn scale_field(byte: u8) -> u8 {
        match byte {
            0x1 => 0b00,
            0x2 => 0b01 << 6,
            0x4 => 0b10 << 6,
            0x8 => 0b11 << 6,
            _ => panic!("scale must 0x1, 0x2, 0x4 or 0x8"),
        }
    }

    pub fn to_byte(&self) -> u8 {
        Self::base_field(self.base_reg)
            | Self::index_field(self.index_reg)
            | Self::scale_field(self.scale)
    }
}

impl fmt::Display for SIBByte {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "SIB(0b{:b})", self.to_byte())
    }
}

impl fmt::Debug for SIBByte {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "SIB(base 0b{:b}: index 0b{:b}: {}x scale)",
            self.base_reg, self.index_reg, self.scale
        )
    }
}
