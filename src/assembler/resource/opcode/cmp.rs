use crate::assembler::resource::*;

impl Opcode {
    pub fn cmp(size: OperandSize, src: Operand, dst: Operand) -> Self {
        match size {
            OperandSize::QWORD => match src {
                Operand::Immediate(imm) => match dst {
                    // cmpq $3, %rax
                    Operand::GENERALREGISTER(dst_gpr) => {
                        if dst_gpr == GeneralPurposeRegister::RAX {
                            Opcode::CMPRAXIMM32 { imm }
                        } else {
                            Opcode::CMPRM64IMM32 { imm, rm64: dst }
                        }
                    }
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
}
