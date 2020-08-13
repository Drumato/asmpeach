//! Type definitions for displacement field in each instructions.

use fmt::Formatter;
use std::fmt;

#[derive(Eq, Ord, PartialOrd, PartialEq, Debug, Clone, Copy)]
pub enum Displacement {
    /// 8bit-displacement
    DISP8(i8),
    /// 32bit-displacement
    DISP32(i32),
}

impl Displacement {
    /// translate to LE bytes for generating machine-code
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            Displacement::DISP8(v8) => vec![*v8 as u8],
            Displacement::DISP32(v32) => (*v32 as u32).to_le_bytes().to_vec(),
        }
    }
}

impl fmt::Display for Displacement {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Displacement::DISP8(v8) => write!(f, "{}", *v8),
            Displacement::DISP32(v32) => write!(f, "{}", *v32),
        }
    }
}
