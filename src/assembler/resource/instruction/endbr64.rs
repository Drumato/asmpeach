use super::{InstName, Instruction};

pub struct EndBr64();

impl Instruction for EndBr64 {
    fn opcode(&self) -> Vec<u8> {
        vec![0xf3, 0x0f, 0x1e, 0xfa]
    }

    fn name(&self) -> InstName {
        InstName::EndBr64
    }
}
