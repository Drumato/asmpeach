use crate::assembler::resource::*;

#[allow(dead_code)]
const SUBRM64R64_CASES: [Instruction; 1] = [Instruction {
    opcode: Opcode::SUBRM64R64 {
        rm64: Operand::ADDRESSING {
            base: GeneralPurposeRegister::RAX,
            index: None,
            scale: None,
            disp: None,
        },
        r64: GeneralPurposeRegister::RBX,
    },
}];

#[allow(dead_code)]
const SUBR64RM64_CASES: [Instruction; 1] = [Instruction {
    opcode: Opcode::SUBR64RM64 {
        r64: GeneralPurposeRegister::RBX,
        rm64: Operand::ADDRESSING {
            base: GeneralPurposeRegister::RAX,
            index: None,
            scale: None,
            disp: None,
        },
    },
}];

#[allow(dead_code)]
const SUBRM64IMM32_CASES: [Instruction; 1] = [Instruction {
    opcode: Opcode::SUBRM64IMM32 {
        rm64: Operand::GENERALREGISTER(GeneralPurposeRegister::RAX),
        imm: Immediate::I32(60),
    },
}];

#[cfg(test)]
mod to_intel_tests {
    use super::*;

    #[test]
    fn subrm64r64_test() {
        // sub QWORD PTR [rax], rbx
        let inst = &SUBRM64R64_CASES[0];

        assert_eq!(inst.to_intel_string(), "sub QWORD PTR [rax], rbx");
    }

    #[test]
    fn subr64rm64_test() {
        // sub rbx, QWWORD PTR [rax]
        let inst = &SUBR64RM64_CASES[0];

        assert_eq!(inst.to_intel_string(), "sub rbx, QWORD PTR [rax]");
    }

    #[test]
    fn subrm64imm32_test() {
        // sub rax, 60
        let inst = &SUBRM64IMM32_CASES[0];

        assert_eq!(inst.to_intel_string(), "sub rax, 60");
    }
}

#[cfg(test)]
mod to_at_tests {
    use super::*;

    #[test]
    fn subrm64r64_test() {
        // sub QWORD PTR [rax], rbx
        let inst = &SUBRM64R64_CASES[0];

        assert_eq!(inst.to_at_string(), "subq %rbx, (%rax)");
    }

    #[test]
    fn subr64rm64_test() {
        // sub rbx, QWORD PTR [rax]
        let inst = &SUBR64RM64_CASES[0];

        assert_eq!(inst.to_at_string(), "subq (%rax), %rbx");
    }

    #[test]
    fn subrm64imm32_test() {
        // sub rax, 60
        let inst = &SUBRM64IMM32_CASES[0];

        assert_eq!(inst.to_at_string(), "subq $60, %rax");
    }
}

#[cfg(test)]
mod to_bytes_tests {
    use super::*;

    #[test]
    fn subrm64r64_test() {
        // sub QWORD PTR [rax], rbx
        let inst = &SUBRM64R64_CASES[0];

        assert_eq!(inst.to_bytes(), vec![0x48, 0x29, 0x18]);
    }

    #[test]
    fn subr64rm64_test() {
        // sub rbx, QWORD PTR [rax]
        let inst = &SUBR64RM64_CASES[0];

        assert_eq!(inst.to_bytes(), vec![0x48, 0x2b, 0x18]);
    }

    #[test]
    fn subrm64imm32_test() {
        // sub rax, 60
        let inst = &SUBRM64IMM32_CASES[0];

        assert_eq!(
            inst.to_bytes(),
            vec![0x48, 0x81, 0xe8, 0x3c, 0x00, 0x00, 0x00]
        );
    }
}
