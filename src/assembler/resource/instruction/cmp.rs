use crate::assembler::resource::*;

#[allow(dead_code)]
#[derive(Eq, Ord, PartialOrd, PartialEq, Debug, Clone)]
pub enum Cmp {
    /// Compare imm32 with r/m64
    CmpRM64WithImm32 { imm: Immediate, rm64: Operand },
    /// Compare imm32 with RAX.
    CmpRAXWithImm32 { imm: Immediate },
}

impl Cmp {
    pub fn new(size: OperandSize, lhs: Operand, rhs: Operand) -> Self {
        todo!()
    }
}

impl Instruction for Cmp {
    fn opcode(&self) -> Vec<u8> {
        todo!()
    }

    fn name(&self) -> InstName {
        todo!()
    }
}
