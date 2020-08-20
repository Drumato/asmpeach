use crate::assembler::resource::*;

impl Opcode {
    pub fn sub(size: OperandSize, src: Operand, dst: Operand) -> Self {
        match size {
            OperandSize::QWORD => match src {
                Operand::GENERALREGISTER(src_gpr) => match dst {
                    // subq %rax, -8(%rbp)
                    Operand::ADDRESSING {
                        base_reg: _,
                        index_reg: _,
                        scale: _,
                        displacement: _,
                    } => Opcode::SUBRM64R64 {
                        rm64: dst,
                        r64: src_gpr,
                    },
                    // subq %rax, %rbx
                    Operand::GENERALREGISTER(dst_gpr) => Opcode::SUBR64RM64 {
                        r64: dst_gpr,
                        rm64: src,
                    },
                    _ => unreachable!(),
                },
                Operand::Immediate(imm) => match dst {
                    // subq $3, %rax
                    Operand::GENERALREGISTER(_dst_gpr) => Opcode::SUBRM64IMM32 { imm, rm64: dst },
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
}
