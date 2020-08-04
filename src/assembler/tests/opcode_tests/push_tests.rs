use crate::assembler::resources::*;

#[allow(dead_code)]
const PUSHIMM32_CASES: [Instruction; 1] = [
    Instruction {
        opcode: Opcode::PUSHIMM32 {
            imm: Immediate::I32(60),
        }
    },
];

#[allow(dead_code)]
const PUSHR64_CASES: [Instruction; 2] = [
    Instruction {
        opcode: Opcode::PUSHR64 {
            r64: GeneralPurposeRegister::RAX,
        }
    },
    Instruction {
        opcode: Opcode::PUSHR64 {
            r64: GeneralPurposeRegister::R15,
        }
    },
];

#[allow(dead_code)]
const PUSHRM64_CASES: [Instruction; 2] = [
    Instruction {
        opcode: Opcode::PUSHRM64 {
            rm64: Operand::ADDRESSING {
                base_reg: GeneralPurposeRegister::RAX,
                index_reg: None,
                displacement: None,
                scale: None,
            },
        }
    },
    Instruction {
        opcode: Opcode::PUSHRM64 {
            rm64: Operand::ADDRESSING {
                base_reg: GeneralPurposeRegister::RAX,
                index_reg: Some(GeneralPurposeRegister::RBX),
                displacement: Some(Displacement::DISP8(-4)),
                scale: Some(4),
            },
        }
    }
];

#[cfg(test)]
mod to_intel_tests {
    use super::*;

    #[test]
    fn pushimm32_test() {
        // push 60
        let inst = &PUSHIMM32_CASES[0];

        assert_eq!(inst.to_intel_string(), "push 60");
    }

    #[test]
    fn pushr64_test() {
        // push rax
        let inst = &PUSHR64_CASES[0];

        assert_eq!(inst.to_intel_string(), "push rax");

        // push r15
        let inst = &PUSHR64_CASES[1];

        assert_eq!(inst.to_intel_string(), "push r15");
    }

    #[test]
    fn pushrm64_test() {
        // push [rax]
        let inst = &PUSHRM64_CASES[0];

        assert_eq!(inst.to_intel_string(), "push QWORD PTR [rax]");

        // push -4[rax + rbx * 4]
        let inst = &PUSHRM64_CASES[1];

        assert_eq!(inst.to_intel_string(), "push QWORD PTR -4[rax + rbx * 4]");
    }
}
#[cfg(test)]
mod to_at_tests {
    use super::*;

    #[test]
    fn pushimm32_test() {
        // push 60
        let inst = &PUSHIMM32_CASES[0];

        assert_eq!(inst.to_at_string(), "pushq $60");
    }

    #[test]
    fn pushr64_test() {
        // push rax
        let inst = &PUSHR64_CASES[0];

        assert_eq!(inst.to_at_string(), "pushq %rax");

        // push r15
        let inst = &PUSHR64_CASES[1];

        assert_eq!(inst.to_at_string(), "pushq %r15");
    }

    #[test]
    fn pushrm64_test() {
        // push [rax]
        let inst = &PUSHRM64_CASES[0];

        assert_eq!(inst.to_at_string(), "pushq (%rax)");

        // push -4[rax + rbx * 4]
        let inst = &PUSHRM64_CASES[1];

        assert_eq!(inst.to_at_string(), "pushq -4(%rax, %rbx, 4)");
    }
}

#[cfg(test)]
mod to_bytes_tests {
    use super::*;

    #[test]
    fn pushimm32_test() {
        // push 60
        let inst = &PUSHIMM32_CASES[0];

        assert_eq!(inst.to_bytes(), vec![0x68, 0x3c, 0x00, 0x00, 0x00]);
    }

    #[test]
    fn pushr64_test() {
        // push rax
        let inst = &PUSHR64_CASES[0];

        assert_eq!(inst.to_bytes(), vec![0x50]);

        // push r15
        let inst = &PUSHR64_CASES[1];

        assert_eq!(inst.to_bytes(), vec![0x41, 0x57]);
    }

    #[test]
    fn pushrm64_test() {
        // push [rax]
        let inst = &PUSHRM64_CASES[0];

        assert_eq!(inst.to_bytes(), vec![0xff, 0x30]);

        // push -4[rax + rbx * 4]
        let inst = &PUSHRM64_CASES[1];

        assert_eq!(inst.to_bytes(), vec![0xff, 0x74, 0x98, 0xfc]);
    }
}