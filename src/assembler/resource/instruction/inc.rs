use crate::assembler::resource::Operand;

#[derive(Eq, Ord, PartialOrd, PartialEq, Debug, Clone)]
pub enum Inc {
    RM64 { rm64: Operand },
}
