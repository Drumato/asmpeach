use super::{InstName, Instruction};

pub enum Ret {
    Near,
    // Far
}

impl Instruction for Ret {
    fn opcode(&self) -> Vec<u8> {
        match self {
            Ret::Near => vec![0xc3],
        }
    }

    fn name(&self) -> super::InstName {
        InstName::Ret
    }
}

#[cfg(test)]
mod near_tests {
    use super::*;

    #[test]
    fn test1() {
        let inst = Ret::Near;
        assert_eq!(vec![0xc3], inst.assemble());
    }
}
