use super::Instruction;

pub struct Ret();

impl Instruction for Ret {
    fn opcode(&self) -> Vec<u8> {
        todo!()
    }

    fn name(&self) -> super::InstName {
        todo!()
    }
}
