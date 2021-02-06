use super::Instruction;

pub struct SysCall();

impl Instruction for SysCall {
    fn opcode(&self) -> Vec<u8> {
        todo!()
    }

    fn name(&self) -> super::InstName {
        todo!()
    }
}
