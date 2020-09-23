use crate::assembler::resource::*;

#[allow(dead_code)]
const IMULR64RM64: [Instruction; 1] = [Instruction {
    opcode: Opcode::IMULR64RM64 {
        r64: GeneralPurposeRegister::R12,
        rm64: Operand::ADDRESSING {
            base: GeneralPurposeRegister::RBP,
            index: None,
            disp: Some(Displacement::DISP8(-16)),
            scale: None,
        },
    },
}];

#[cfg(test)]
mod to_bytes_tests {
    use super::*;

    #[test]
    fn imulr64rm64_test() {
        let inst = &IMULR64RM64[0];
        assert_eq!(inst.to_bytes(), vec![0x4c, 0x0f, 0xaf, 0x65, 0xf0]);
    }
}
