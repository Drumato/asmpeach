use crate::assembler::resource::*;

#[allow(dead_code)]
const IDIVRM64: [Instruction; 1] = [
    Instruction {
        opcode: Opcode::IDIVRM64 {
            rm64: Operand::ADDRESSING {
                base_reg: GeneralPurposeRegister::RAX,
                index_reg: None,
                displacement: None,
                scale: None,
            }
        }
    }
];

#[cfg(test)]
mod to_intel_tests {
    use super::*;

    #[test]
    fn idivrm64_test() {
        let inst = &IDIVRM64[0];
        assert_eq!(inst.to_intel_string(), "idiv QWORD PTR [rax]");
    }
}

#[cfg(test)]
mod to_at_tests {
    use super::*;

    #[test]
    fn idivrm64_test() {
        let inst = &IDIVRM64[0];
        assert_eq!(inst.to_at_string(), "idivq (%rax)");
    }
}

#[cfg(test)]
mod to_bytes_tests {
    use super::*;

    #[test]
    fn idivrm64_test() {
        let inst = &IDIVRM64[0];
        assert_eq!(inst.to_bytes(), vec![0x48, 0xf7, 0x38]);
    }
}