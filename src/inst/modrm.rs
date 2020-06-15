pub struct ModRM {
    pub mode: AddressingMode,
    pub reg: u8,
    pub rm: u8,
}

impl ModRM {
    pub fn to_byte(&self) -> u8 {
        self.mode.to_byte() | self.reg | self.rm
    }
}

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