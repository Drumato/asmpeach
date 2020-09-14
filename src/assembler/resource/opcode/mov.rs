use crate::assembler::resource::*;
impl Opcode {
    pub fn mov(size: OperandSize, src: Operand, dst: Operand) -> Self {
        match size {
            OperandSize::DWORD => match src {
                Operand::GENERALREGISTER(src_gpr) => match dst {
                    // movl %ebx, %eax
                    Operand::GENERALREGISTER(_dst_gpr) => Opcode::MOVRM32R32 {
                        r32: src_gpr,
                        rm32: dst,
                    },
                    // movl %eax, -8(%ebp)
                    Operand::ADDRESSING {
                        base: _,
                        index: _,
                        disp: _,
                        scale: _,
                    } => Opcode::MOVRM32R32 {
                        r32: src_gpr,
                        rm32: dst,
                    },
                    _ => unreachable!(),
                },
                Operand::Immediate(imm) => match dst {
                    // movl $3, %eax
                    Operand::GENERALREGISTER(_dst_gpr) => Opcode::MOVRM32IMM32 { imm, rm32: dst },
                    // movl $3, -8(%ebp)
                    Operand::ADDRESSING {
                        base: _,
                        index: _,
                        disp: _,
                        scale: _,
                    } => Opcode::MOVRM32IMM32 { imm, rm32: dst },
                    _ => unreachable!(),
                },
                Operand::ADDRESSING {
                    base: _,
                    index: _,
                    disp: _,
                    scale: _,
                } => match dst {
                    // movq -8(%rbp), %rax
                    Operand::GENERALREGISTER(dst_gpr) => Opcode::MOVR32RM32 {
                        r32: dst_gpr,
                        rm32: src,
                    },
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            },
            OperandSize::QWORD => match src {
                Operand::GENERALREGISTER(src_gpr) => match dst {
                    // movq %rbx, %rax
                    Operand::GENERALREGISTER(_dst_gpr) => Opcode::MOVRM64R64 {
                        r64: src_gpr,
                        rm64: dst,
                    },
                    // movq %rax, -8(%rbp)
                    Operand::ADDRESSING {
                        base: _,
                        index: _,
                        disp: _,
                        scale: _,
                    } => Opcode::MOVRM64R64 {
                        r64: src_gpr,
                        rm64: dst,
                    },
                    _ => unreachable!(),
                },
                Operand::Immediate(imm) => match dst {
                    // movq $3, %rax
                    Operand::GENERALREGISTER(_dst_gpr) => Opcode::MOVRM64IMM32 { imm, rm64: dst },
                    // movq $3, -8(%rbp)
                    Operand::ADDRESSING {
                        base: _,
                        index: _,
                        disp: _,
                        scale: _,
                    } => Opcode::MOVRM64IMM32 { imm, rm64: dst },
                    _ => unreachable!(),
                },
                Operand::ADDRESSING {
                    base: _,
                    index: _,
                    disp: _,
                    scale: _,
                } => match dst {
                    // movq -8(%rbp), %rax
                    Operand::GENERALREGISTER(dst_gpr) => Opcode::MOVR64RM64 {
                        r64: dst_gpr,
                        rm64: src,
                    },
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
}
