#[macro_export]
macro_rules! new_r64rm64 {
    ($opcode:ident, $r64:expr, $rm64:expr) => {
        Opcode::$opcode { r64: $r64, rm64: $rm64 }
    };
}

#[macro_export]
macro_rules! new_rm64r64 {
    ($opcode:ident, $rm64:expr, $r64:expr) => {
        Opcode::$opcode { rm64: $rm64, r64: $r64 }
    };
}

#[macro_export]
macro_rules! new_rm64imm32 {
    ($opcode:ident, $rm64:expr, $imm32:expr) => {
        Opcode::$opcode { rm64: $rm64, imm: $imm32 }
    };
}

#[macro_export]
macro_rules! new_r64{
($opcode:ident, $r64:expr) => {
        Opcode::$opcode { r64: $r64 }
    };
}

#[macro_export]
macro_rules! new_rm64{
($opcode:ident, $rm64:expr) => {
        Opcode::$opcode { rm64: $rm64 }
    };
}

#[macro_export]
macro_rules! new_imm32{
($opcode:ident, $imm32:expr) => {
        Opcode::$opcode { imm: $imm32 }
    };
}


#[cfg(test)]
mod macros_tests {
    use crate::{
        GeneralPurposeRegister as GPR,
        Operand,
        Opcode,
        Immediate,
    };

    #[test]
    fn newr64rm64_test() {
        let inst = new_r64rm64!(MOVR64RM64, GPR::RAX, Operand::GENERALREGISTER(GPR::RBX));

        assert_eq!("mov rax, rbx", inst.to_intel_string());
    }

    #[test]
    fn newrm64r64_test() {
        let inst = new_rm64r64!(MOVRM64R64, Operand::GENERALREGISTER(GPR::RBX), GPR::RAX);

        assert_eq!("mov rbx, rax", inst.to_intel_string());
    }

    #[test]
    fn newrm64imm32_test() {
        let inst = new_rm64imm32!(MOVRM64IMM32, Operand::GENERALREGISTER(GPR::RAX), Immediate::I32(30));

        assert_eq!("mov rax, 30", inst.to_intel_string());
    }

    #[test]
    fn newr64_test() {
        let inst = new_r64!(PUSHR64, GPR::RAX);

        assert_eq!("push rax", inst.to_intel_string());
    }

    #[test]
    fn newrm64_test() {
        let inst = new_rm64!(INCRM64, Operand::GENERALREGISTER(GPR::RAX));

        assert_eq!("inc rax", inst.to_intel_string());
    }

    #[test]
    fn newimm32_test() {
        let inst = new_imm32!(PUSHIMM32, Immediate::I32(30));

        assert_eq!("push 30", inst.to_intel_string());
    }
}