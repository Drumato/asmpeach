/// using for 64-bit mode.
pub struct REXPrefix {
    /// related with operand-size.
    pub w_bit: bool,
    /// related with reg-field in ModR/M
    pub r_bit: bool,
    /// related with index-field in ModR/M
    pub x_bit: bool,
    /// related with r/m-field in ModR/M, base in SIB-byte, reg-field in opcode
    pub b_bit: bool,
}

impl REXPrefix {
    pub const BASE: u8 = 0x40;
    pub const W_BIT: u8 = 0x80;
    pub const R_BIT: u8 = 0x40;
    pub const X_BIT: u8 = 0x02;
    pub const B_BIT: u8 = 0x01;


    pub fn to_byte(&self) -> u8 {
        let base = Self::BASE;
        let f = |bit: bool, byte: u8| -> u8{
            if bit {
                byte
            } else {
                0b0
            }
        };

        base | f(self.w_bit, Self::W_BIT) | f(self.r_bit, Self::W_BIT) | f(self.x_bit, Self::X_BIT) | f(self.b_bit, Self::B_BIT)
    }
}

#[cfg(test)]
mod rex_prefix_tests {
    use crate::REXPrefix;

    #[test]
    fn to_byte_test() {
        let prefix = REXPrefix {
            w_bit: true,
            r_bit: false,
            x_bit: true,
            b_bit: false,
        };

        assert_eq!(REXPrefix::BASE | REXPrefix::W_BIT | REXPrefix::X_BIT, prefix.to_byte());
    }
}