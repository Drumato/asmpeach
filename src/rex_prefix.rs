use crate::{GeneralPurposeRegister};

use std::fmt;
use fmt::Formatter;

/// using for 64-bit mode.
#[derive(Eq, Ord, PartialOrd, PartialEq, Clone, Copy)]
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
    pub const W_BIT: u8 = 0x08;
    pub const R_BIT: u8 = 0x04;
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

    pub fn b_bit_from_reg(reg: &GeneralPurposeRegister) -> u8 {
        if reg.is_expanded() {
            Self::B_BIT
        } else {
            0b0
        }
    }
    pub fn r_bit_from_reg(reg: &GeneralPurposeRegister) -> u8 {
        if reg.is_expanded() {
            Self::R_BIT
        } else {
            0b0
        }
    }
}

impl fmt::Display for REXPrefix{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "REX(0b{:b})", self.to_byte())
    }
}

impl fmt::Debug for REXPrefix {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let func = |b: bool, c: char| -> char{
            if b {
                c
            } else {
                '-'
            }
        };

        let w = func(self.w_bit, 'W');
        let r = func(self.r_bit, 'R');
        let x = func(self.x_bit, 'X');
        let b = func(self.b_bit, 'B');

        write!(f, "0100{}{}{}{}", w, r, x, b)
    }
}

#[cfg(test)]
mod rex_prefix_tests {
    use super::*;

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

    #[test]
    fn display_rex_prefix_test() {
        let prefix = REXPrefix {
            w_bit: true,
            r_bit: false,
            x_bit: true,
            b_bit: false,
        };

        assert_eq!("REX(0b1001010)", format!("{}", prefix).as_str());
    }

    #[test]
    fn debug_rex_prefix_test() {
        let prefix = REXPrefix {
            w_bit: true,
            r_bit: false,
            x_bit: true,
            b_bit: false,
        };

        assert_eq!("0100W-X-", format!("{:?}", prefix).as_str());
    }
}