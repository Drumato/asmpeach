use crate::assembler::resource::*;

#[allow(dead_code)]
const MOVRM8R8_CASES: [Instruction; 2] = [
    Instruction {
        opcode: Opcode::MOVRM8R8 {
            rm8: Operand::GENERALREGISTER(GeneralPurposeRegister::BH),
            r8: GeneralPurposeRegister::AH,
        },
    },
    Instruction {
        opcode: Opcode::MOVRM8R8 {
            rm8: Operand::ADDRESSING {
                base: GeneralPurposeRegister::AL,
                index: None,
                disp: None,
                scale: None,
            },
            r8: GeneralPurposeRegister::BH,
        },
    },
];

#[allow(dead_code)]
const MOVRM64R64_CASES: [Instruction; 2] = [
    Instruction {
        opcode: Opcode::MOVRM64R64 {
            rm64: Operand::GENERALREGISTER(GeneralPurposeRegister::RAX),
            r64: GeneralPurposeRegister::RCX,
        },
    },
    Instruction {
        opcode: Opcode::MOVRM64R64 {
            rm64: Operand::ADDRESSING {
                base: GeneralPurposeRegister::RAX,
                index: Some(GeneralPurposeRegister::RBX),
                disp: None,
                scale: Some(0x4),
            },
            r64: GeneralPurposeRegister::RCX,
        },
    },
];

#[allow(dead_code)]
const MOVR64RM64_CASES: [Instruction; 1] = [Instruction {
    opcode: Opcode::MOVR64RM64 {
        r64: GeneralPurposeRegister::RAX,
        rm64: Operand::ADDRESSING {
            base: GeneralPurposeRegister::RAX,
            index: None,
            disp: None,
            scale: None,
        },
    },
}];

#[allow(dead_code)]
const MOVRM64IMM32_CASES: [Instruction; 1] = [Instruction {
    opcode: Opcode::MOVRM64IMM32 {
        rm64: Operand::ADDRESSING {
            base: GeneralPurposeRegister::RAX,
            index: None,
            disp: None,
            scale: None,
        },
        imm: Immediate::I32(60),
    },
}];

#[cfg(test)]
mod to_bytes_tests {
    use super::*;

    #[test]
    fn movrm8r8_test() {
        // mov bh, ah
        let inst = &MOVRM8R8_CASES[0];

        assert_eq!(inst.to_bytes(), vec![0x88, 0xe7]);

        // mov BYTE PTR [rax], bh
        let inst = &MOVRM8R8_CASES[1];

        assert_eq!(inst.to_bytes(), vec![0x88, 0x38]);
    }

    #[test]
    fn movrm64r64_test() {
        // mov rax, rcx
        let inst = &MOVRM64R64_CASES[0];

        assert_eq!(inst.to_bytes(), vec![0x48, 0x89, 0xc8]);

        // mov [rax + rbx * 4], rcx
        let inst = &MOVRM64R64_CASES[1];

        assert_eq!(inst.to_bytes(), vec![0x48, 0x89, 0x0c, 0x98]);
    }

    #[test]
    fn movr64rm64_test() {
        // mov rax, [rax]
        let inst = &MOVR64RM64_CASES[0];

        assert_eq!(inst.to_bytes(), vec![0x48, 0x8b, 0x00]);
    }

    #[test]
    fn movrm64imm32_test() {
        // mov QWORD PTR [rax], 60
        let inst = &MOVRM64IMM32_CASES[0];
        assert_eq!(
            inst.to_bytes(),
            vec![0x48, 0xc7, 0x00, 0x3c, 0x00, 0x00, 0x00]
        )
    }
}
