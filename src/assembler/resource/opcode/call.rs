use crate::assembler::resource::*;

impl Opcode {
    pub fn call(func: Operand) -> Self {
        Opcode::CALLFUNC(func)
    }
}
