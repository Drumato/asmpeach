use crate::assembler::resource::*;

#[allow(dead_code)]
const POPR64_CASES: [Instruction; 2] = [
    Instruction {
        opcode: Opcode::POPR64 {
            r64: GeneralPurposeRegister::RAX,
        },
    },
    Instruction {
        opcode: Opcode::POPR64 {
            r64: GeneralPurposeRegister::R15,
        },
    },
];

#[cfg(test)]
mod to_intel_tests {
    use super::*;

    #[test]
    fn popr64_test() {
        let inst = &POPR64_CASES[0];

        assert_eq!(inst.to_intel_string(), "pop rax");

        let inst = &POPR64_CASES[1];

        assert_eq!(inst.to_intel_string(), "pop r15");
    }
}

#[cfg(test)]
mod to_at_tests {
    use super::*;

    #[test]
    fn popr64_test() {
        let inst = &POPR64_CASES[0];

        assert_eq!(inst.to_at_string(), "popq %rax");

        let inst = &POPR64_CASES[1];

        assert_eq!(inst.to_at_string(), "popq %r15");
    }
}

#[cfg(test)]
mod to_bytes_tests {
    use super::*;

    #[test]
    fn popr64_test() {
        let inst = &POPR64_CASES[0];

        assert_eq!(inst.to_bytes(), vec![0x58]);

        let inst = &POPR64_CASES[1];

        assert_eq!(inst.to_bytes(), vec![0x41, 0x5f]);
    }
}
