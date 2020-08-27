use crate::assembler::resource::*;
impl Opcode {
    pub fn lea(size: OperandSize, src: Operand, dst: Operand) -> Self {
        match size {
            OperandSize::QWORD => match src {
                Operand::ADDRESSING {
                    base_reg: _,
                    index_reg: _,
                    displacement: _,
                    scale: _,
                } => match dst {
                    // leaq -8(%rbp), %rax
                    Operand::GENERALREGISTER(dst_gpr) => Opcode::LEAR64M {
                        r64: dst_gpr,
                        m: src,
                    },
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
}
