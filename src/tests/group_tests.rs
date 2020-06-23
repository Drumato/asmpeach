#[cfg(test)]
mod group_tests {
    use crate::*;

    #[test]
    fn to_intel_string_test() {
        let group = prepare();
        let expected = "main:\n  push rbp\n  mov rax, 60\n  pop rbp\n  ret\n";

        assert_eq!(group.to_intel_string(), expected);
    }

    #[test]
    fn to_at_string_test() {
        let group = prepare();

        let expected = "main:\n  pushq %rbp\n  movq $60, %rax\n  popq %rbp\n  ret\n";

        assert_eq!(group.to_at_string(), expected);
    }

    fn prepare() -> Group {
        // push rbp; mov rax, 60; pop rbp; ret
        let mut g: Group = Default::default();

        g.label = "main".to_string();

        g.insts.push(Instruction {
            opcode: Opcode::PUSHR64 {
                r64: GeneralPurposeRegister::RBP,
            }
        });

        g.insts.push(Instruction {
            opcode: Opcode::MOVRM64IMM32 {
                rm64: Operand::GENERALREGISTER(GeneralPurposeRegister::RAX),
                imm: Immediate::I32(60),
            }
        });

        g.insts.push(Instruction {
            opcode: Opcode::POPR64 {
                r64: GeneralPurposeRegister::RBP,
            }
        });

        g.insts.push(Instruction {
            opcode: Opcode::RET,
        });

        g
    }
}