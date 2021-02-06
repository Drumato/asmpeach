use super::Instruction;

pub struct EndBr64();

impl Instruction for EndBr64 {
    fn opcode(&self) -> Vec<u8> {
        todo!()
    }

    fn name(&self) -> super::InstName {
        todo!()
    }
}
