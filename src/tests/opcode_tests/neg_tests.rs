use crate::resources::*;

#[allow(dead_code)]
const NEGRM64_CASES: [Instruction; 1] = [
    Instruction {
        opcode: Opcode::NEGRM64 {
            rm64: Operand::GENERALREGISTER(GeneralPurposeRegister::RAX),
        }
    }
];

#[cfg(test)]
mod to_intel_tests {
    use super::*;

    #[test]
    fn negrm64_test() {
        let inst = &NEGRM64_CASES[0];

        assert_eq!(inst.to_intel_string(), "neg rax");
    }
}

#[cfg(test)]
mod to_at_tests {
    use super::*;

    #[test]
    fn negrm64_test() {
        let inst = &NEGRM64_CASES[0];

        assert_eq!(inst.to_at_string(), "negq %rax");
    }
}

#[cfg(test)]
mod to_bytes_tests {
    use super::*;

    #[test]
    fn negrm64_test() {
        let inst = &NEGRM64_CASES[0];

        assert_eq!(inst.to_bytes(), vec![0x48, 0xf7, 0xd8]);
    }
}
