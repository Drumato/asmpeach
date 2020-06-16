#[derive(Eq, Ord, PartialOrd, PartialEq, Debug, Clone, Copy)]
pub enum Displacement {
    DISP8(i8),
    DISP32(i32),
}

impl Displacement {
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            Displacement::DISP8(v8) => vec![*v8 as u8],
            Displacement::DISP32(v32) => (*v32 as u32).to_le_bytes().to_vec(),
        }
    }
}