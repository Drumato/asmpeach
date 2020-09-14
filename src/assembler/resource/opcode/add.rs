use crate::assembler::resource::*;

impl Opcode {
    pub fn add(size: OperandSize, src: Operand, dst: Operand) -> Self {
        match size {
            OperandSize::DWORD => match src {
                Operand::GENERALREGISTER(src_gpr) => match dst {
                    // add -8[ebp], eax
                    Operand::ADDRESSING {
                        base: _,
                        index: _,
                        scale: _,
                        disp: _,
                    } => Opcode::ADDRM32R32 {
                        rm32: dst,
                        r32: src_gpr,
                    },
                    // add eax, ebx
                    Operand::GENERALREGISTER(dst_gpr) => Opcode::ADDR32RM32 {
                        r32: dst_gpr,
                        rm32: src,
                    },
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            },
            OperandSize::QWORD => match src {
                Operand::GENERALREGISTER(src_gpr) => match dst {
                    // add -8[rbp], rax
                    Operand::ADDRESSING {
                        base: _,
                        index: _,
                        scale: _,
                        disp: _,
                    } => Opcode::ADDRM64R64 {
                        rm64: dst,
                        r64: src_gpr,
                    },
                    // add rax, rbx
                    Operand::GENERALREGISTER(dst_gpr) => Opcode::ADDR64RM64 {
                        r64: dst_gpr,
                        rm64: src,
                    },
                    _ => unreachable!(),
                },
                Operand::ADDRESSING { base: _, index: _, disp: _, scale:_ } => match dst{
                    Operand::GENERALREGISTER(dst_gpr) => Opcode::ADDR64RM64 {
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
