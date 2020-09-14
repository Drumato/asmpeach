use crate::assembler::resource::*;

#[allow(dead_code)]
const IMULR64RM64: [Instruction; 1] = [Instruction {
    opcode: Opcode::IMULR64RM64 {
        r64: GeneralPurposeRegister::R12,
        rm64: Operand::ADDRESSING {
            base_reg: GeneralPurposeRegister::RBP,
            index_reg: None,
            displacement: Some(Displacement::DISP8(-16)),
            scale: None,
        },
    },
}];

#[cfg(test)]
mod to_intel_tests {
    use super::*;

    #[test]
    fn imulr64rm64_test() {
        let inst = &IMULR64RM64[0];
        assert_eq!(inst.to_intel_string(), "imul r12, QWORD PTR -16[rbp]");
    }
}

#[cfg(test)]
mod to_at_tests {
    use super::*;

    #[test]
    fn imulr64rm64_test() {
        let inst = &IMULR64RM64[0];
        assert_eq!(inst.to_at_string(), "imulq -16(%rbp), %r12");
    }
}

#[cfg(test)]
mod to_bytes_tests {
    use super::*;

    #[test]
    fn imulr64rm64_test() {
        let inst = &IMULR64RM64[0];
        assert_eq!(inst.to_bytes(), vec![0x4c, 0x0f, 0xaf, 0x65, 0xf0]);
    }
}
