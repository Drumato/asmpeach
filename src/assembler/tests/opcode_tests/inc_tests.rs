use crate::assembler::resource::*;

#[allow(dead_code)]
const INCRM64_CASES: [Instruction; 1] = [Instruction {
    opcode: Opcode::INCRM64 {
        rm64: Operand::GENERALREGISTER(GeneralPurposeRegister::RAX),
    },
}];

#[cfg(test)]
mod to_bytes_tests {
    use super::*;

    #[test]
    fn incrm64_test() {
        let inst = &INCRM64_CASES[0];

        assert_eq!(inst.to_bytes(), vec![0x48, 0xff, 0xc0]);
    }
}
