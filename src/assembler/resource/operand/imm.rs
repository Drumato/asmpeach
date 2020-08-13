use fmt::Formatter;
use std::fmt;

#[derive(Eq, Ord, PartialOrd, PartialEq, Debug, Clone, Copy)]
pub enum Immediate {
    I8(i8),
    I32(i32),
}

impl Immediate {
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            Immediate::I8(v8) => vec![*v8 as u8],
            Immediate::I32(v32) => (*v32 as u32).to_le_bytes().to_vec(),
        }
    }
    pub fn as_32bit(&self) -> Self {
        match self {
            Immediate::I8(v8) => Self::I32(*v8 as i32),
            Immediate::I32(_v32) => *self,
        }
    }

    pub fn to_intel_string(&self) -> String {
        self.to_string()
    }

    pub fn to_at_string(&self) -> String {
        format!("${}", self.to_string())
    }
}

impl fmt::Display for Immediate {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Immediate::I8(v8) => write!(f, "{}", v8),
            Immediate::I32(v32) => write!(f, "{}", v32),
        }
    }
}
