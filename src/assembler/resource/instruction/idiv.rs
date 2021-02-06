use crate::assembler::resource::Operand;

#[derive(Eq, Ord, PartialOrd, PartialEq, Debug, Clone)]
pub enum Idiv {
    /// signed divide RDX:RAX by r/m64;
    /// result stored in RAX := Quotient, RDX := Remainder.
    IdivRM64 { rm64: Operand },
}
