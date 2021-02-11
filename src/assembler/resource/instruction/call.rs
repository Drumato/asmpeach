use crate::assembler::resource::{Immediate, Operand};

use super::{InstName, Instruction};

#[derive(Eq, Ord, PartialOrd, PartialEq, Debug, Clone)]
pub struct Call {
    pub name: Operand,
}

impl Call {
    pub fn new(op: Operand) -> Self {
        Call { name: op }
    }
}

impl Instruction for Call {
    fn opcode(&self) -> Vec<u8> {
        vec![0xe8]
    }

    fn name(&self) -> InstName {
        InstName::Call
    }

    fn operand_1(&self) -> Option<Operand> {
        Some(self.name.clone())
    }

    fn immediate(&self) -> Option<Immediate> {
        // 適当に空のアドレスを生成しておく
        Some(Immediate::I32(0))
    }
}
