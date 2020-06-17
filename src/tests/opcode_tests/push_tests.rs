#[cfg(test)]
mod to_bytes_tests {
    use crate::*;

    #[test]
    fn pushimm32_test() {
        // push 60
        let inst = Instruction {
            opcode: Opcode::PUSHIMM32 {
                imm: Immediate::I32(60),
            }
        };

        assert_eq!(inst.to_bytes(), vec![0x68, 0x3c, 0x00, 0x00, 0x00]);
    }

    #[test]
    fn pushr64_test() {
        // push rax
        let inst = Instruction {
            opcode: Opcode::PUSHR64 {
                r64: GeneralPurposeRegister::RAX,
            }
        };

        assert_eq!(inst.to_bytes(), vec![0x50]);

        // push r15
        let inst = Instruction {
            opcode: Opcode::PUSHR64 {
                r64: GeneralPurposeRegister::R15,
            }
        };

        assert_eq!(inst.to_bytes(), vec![0x41, 0x57]);
    }

    #[test]
    fn pushrm64_test() {
        // push [rax]
        let inst = Instruction {
            opcode: Opcode::PUSHRM64 {
                rm64: Operand::ADDRESSING {
                    base_reg: GeneralPurposeRegister::RAX,
                    index_reg: None,
                    displacement: None,
                    scale: None,
                },
            }
        };

        assert_eq!(inst.to_bytes(), vec![0xff, 0x30]);

        // push -4[rax + rbx * 4]
        let inst = Instruction {
            opcode: Opcode::PUSHRM64 {
                rm64: Operand::ADDRESSING {
                    base_reg: GeneralPurposeRegister::RAX,
                    index_reg: Some(GeneralPurposeRegister::RBX),
                    displacement: Some(Displacement::DISP8(-4)),
                    scale: Some(4),
                },
            }
        };

        assert_eq!(inst.to_bytes(), vec![0xff, 0x74, 0x98, 0xfc]);
    }
}