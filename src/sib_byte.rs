
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
        assert!(byte <= 0x3);

        byte << 6
    }

    pub fn to_byte(&self) -> u8 {
        Self::base_field(self.base_reg) | Self::index_field(self.index_reg) | Self::scale_field(self.scale)
    }
}