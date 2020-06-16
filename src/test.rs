#[cfg(test)]
mod mov_tests {
    use crate::*;

    #[test]
    fn movrm8r8_test() {
        // mov bh, ah
        let inst = Instruction {
            opcode: Opcode::MOVRM8R8 {
                rm8: Operand::GENERALREGISTER(GeneralPurposeRegister::BH),
                r8: Operand::GENERALREGISTER(GeneralPurposeRegister::AH),
            }
        };

        assert_eq!(inst.to_bytes(), vec![0x88, 0xe7]);

        // mov BYTE PTR [rax], bh
        let inst = Instruction {
            opcode: Opcode::MOVRM8R8 {
                rm8: Operand::ADDRESSING {
                    base_reg: GeneralPurposeRegister::AL,
                    index_reg: None,
                    displacement: None,
                    scale: None,
                },
                r8: Operand::GENERALREGISTER(GeneralPurposeRegister::BH),
            }
        };

        assert_eq!(inst.to_bytes(), vec![0x88, 0x38]);
    }

    #[test]
    fn movrm64r64_test() {
        // mov [rax + rbx * 4], rcx
        let inst = Instruction {
            opcode: Opcode::MOVRM64R64 {
                rm64: Operand::ADDRESSING {
                    base_reg: GeneralPurposeRegister::RAX,
                    index_reg: Some(GeneralPurposeRegister::RBX),
                    displacement: None,
                    scale: Some(0x4),
                },
                r64: Operand::GENERALREGISTER(GeneralPurposeRegister::RCX),
            }
        };

        assert_eq!(inst.to_bytes(), vec![0x48, 0x89, 0x0c, 0x98]);
    }
}