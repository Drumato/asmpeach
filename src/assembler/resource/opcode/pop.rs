use crate::assembler::resource::*;

impl Opcode {
    pub fn pop(size: OperandSize, operand: Operand) -> Self {
        match size {
            OperandSize::QWORD => match operand {
                Operand::GENERALREGISTER(gpr) => Opcode::POPR64 { r64: gpr },
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
}
