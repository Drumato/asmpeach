use crate::assembler::resource::*;

#[allow(dead_code)]
const ADDRM64R64_CASES: [Instruction; 2] = [
    Instruction {
        opcode: Opcode::ADDRM64R64 {
            rm64: Operand::GENERALREGISTER(GeneralPurposeRegister::RAX),
            r64: GeneralPurposeRegister::RBX,
        },
    },
    Instruction {
        opcode: Opcode::ADDRM64R64 {
            rm64: Operand::ADDRESSING {
                base: GeneralPurposeRegister::RAX,
                index: None,
                scale: None,
                disp: None,
            },
            r64: GeneralPurposeRegister::RBX,
        },
    },
];

#[allow(dead_code)]
const ADDR64RM64_CASES: [Instruction; 1] = [Instruction {
    opcode: Opcode::ADDR64RM64 {
        r64: GeneralPurposeRegister::RAX,
        rm64: Operand::GENERALREGISTER(GeneralPurposeRegister::RBX),
    },
}];

#[cfg(test)]
mod to_bytes_tests {
    use super::*;

    #[test]
    fn addrm64r64_test() {
        // add rax, rbx
        let inst = &ADDRM64R64_CASES[0];

        assert_eq!(inst.to_bytes(), vec![0x48, 0x01, 0xd8]);

        // add QWORD PTR [rax], rbx
        let inst = &ADDRM64R64_CASES[1];

        assert_eq!(inst.to_bytes(), vec![0x48, 0x01, 0x18]);
    }

    #[test]
    fn addr64rm64_test() {
        // add rax, rbx
        let inst = &ADDR64RM64_CASES[0];

        assert_eq!(inst.to_bytes(), vec![0x48, 0x03, 0xc3]);
    }
}
