use crate::assembler::resource::*;

impl Opcode {
    pub fn imul(size: OperandSize, src: Operand, dst: Operand) -> Self {
        match size {
            OperandSize::QWORD => match src {
                Operand::GENERALREGISTER(_src_gpr) => match dst {
                    _ => unreachable!(),
                },
                Operand::ADDRESSING {
                    base_reg: _,
                    index_reg: _,
                    scale: _,
                    displacement: _,
                } => match dst {
                    // imul -8[rbp], rax
                    Operand::GENERALREGISTER(dst_gpr) => Opcode::IMULR64RM64 {
                        rm64: src,
                        r64: dst_gpr,
                    },
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
}
