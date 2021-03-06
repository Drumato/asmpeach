use crate::assembler::resource::*;

#[allow(dead_code)]
const NEGRM64_CASES: [Instruction; 1] = [Instruction {
    opcode: Opcode::NEGRM64 {
        rm64: Operand::GENERALREGISTER(GeneralPurposeRegister::RAX),
    },
}];

#[cfg(test)]
mod to_bytes_tests {
    use super::*;

    #[test]
    fn negrm64_test() {
        let inst = &NEGRM64_CASES[0];

        assert_eq!(inst.to_bytes(), vec![0x48, 0xf7, 0xd8]);
    }
}
