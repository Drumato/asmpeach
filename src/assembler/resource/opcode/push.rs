use crate::assembler::resource::*;

impl Opcode {
    pub fn push(size: OperandSize, operand: Operand) -> Self {
        match size {
            OperandSize::QWORD => match operand {
                Operand::GENERALREGISTER(gpr) => Opcode::PUSHR64 { r64: gpr },
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
}
