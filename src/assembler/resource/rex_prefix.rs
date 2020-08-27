use crate::assembler::resource::*;
use fmt::Formatter;
use std::fmt;

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

    pub fn new(w: bool, r: bool, x: bool, b: bool) -> Self {
        Self {
            w_bit: w,
            r_bit: r,
            x_bit: x,
            b_bit: b,
        }
    }

    pub fn new_from_mem(is_64bit: bool, rm: &Operand) -> Self {
        Self::new(is_64bit,rm.is_expanded(),rm.req_sib_byte() && rm.index_reg_is_expanded(),false)
    }

    pub fn new_from_mem_and_reg(
        is_64bit: bool,
        reg: &GeneralPurposeRegister,
        rm: &Operand,
    ) -> Self {
        Self::new(is_64bit,rm.is_expanded(),rm.req_sib_byte() && rm.index_reg_is_expanded(),reg.is_expanded())
    }

    pub fn to_byte(&self) -> u8 {
        let base = Self::BASE;
        let f = |bit: bool, byte: u8| -> u8 {
            if bit {
                byte
            } else {
                0b0
            }
        };

        base | f(self.w_bit, Self::W_BIT)
            | f(self.r_bit, Self::W_BIT)
            | f(self.x_bit, Self::X_BIT)
            | f(self.b_bit, Self::B_BIT)
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

impl fmt::Display for REXPrefix {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "REX(0b{:b})", self.to_byte())
    }
}

impl fmt::Debug for REXPrefix {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let func = |b: bool, c: char| -> char {
            if b {
                c
            } else {
                '-'
            }
        };

        let w_bit = func(self.w_bit, 'W');
        let r_bit = func(self.r_bit, 'R');
        let x_bit = func(self.x_bit, 'X');
        let b_bit = func(self.b_bit, 'B');

        write!(f, "0100{}{}{}{}", w_bit, r_bit, x_bit, b_bit)
    }
}
