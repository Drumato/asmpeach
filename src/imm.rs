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
}