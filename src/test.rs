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
    }
}