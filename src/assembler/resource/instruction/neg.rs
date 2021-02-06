use crate::assembler::resource::Operand;

#[derive(Eq, Ord, PartialOrd, PartialEq, Debug, Clone)]
pub enum Neg {
    /// Two's complement negate r/m64
    RM64 { rm64: Operand },
}
