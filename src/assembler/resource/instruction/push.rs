use std::cell::RefMut;

use crate::assembler::resource::*;

#[derive(Eq, Ord, PartialOrd, PartialEq, Debug, Clone)]
pub enum Push {
    /// Push r/m64
    RM64 { rm64: Operand },

    /// Push r64,
    R64 { r64: GeneralPurposeRegister },
    /// Push imm8
    Imm8 { imm8: Immediate },
    /// Push imm16
    Imm16 { imm16: Immediate },
    /// Push imm32
    Imm32 { imm32: Immediate },
}

impl Push {
    pub fn new(size: OperandSize, op: Operand) -> Self {
        match size {
            OperandSize::Qword => push_qword(op),
            _ => todo!(),
        }
    }
}

impl Instruction for Push {
    fn opcode(&self) -> Vec<u8> {
        match self {
            Push::RM64 { .. } => vec![0xff],
            Push::R64 { r64 } => vec![0x50 + r64.number()],
            Push::Imm8 { .. } => vec![0x6a],
            Push::Imm16 { .. } => vec![0x68],
            Push::Imm32 { .. } => vec![0x68],
        }
    }

    fn name(&self) -> InstName {
        InstName::Push
    }
    fn rex_prefix(&self) -> Option<REXPrefix> {
        match self {
            Push::R64 { r64 } => {
                // bビットにr64.is_expanded()を渡してはいけない．
                // 拡張レジスタ以外ではREX-prefixがつかない．
                if r64.is_expanded() {
                    Some(REXPrefix::new(false, false, false, true))
                } else {
                    None
                }
            }
            _ => None,
        }
    }
    fn immediate(&self) -> Option<Immediate> {
        match self {
            Push::Imm8 { imm8: imm } | Push::Imm16 { imm16: imm } | Push::Imm32 { imm32: imm } => {
                Some(*imm)
            }
            _ => None,
        }
    }
}

fn push_qword(op: Operand) -> Push {
    match op {
        Operand::GeneralReg(reg) => Push::R64 { r64: reg },
        Operand::Immediate(imm) => match imm {
            Immediate::I8(_v) => Push::Imm8 { imm8: imm },
            Immediate::I16(_v) => Push::Imm16 { imm16: imm },
            Immediate::I32(_v) => Push::Imm32 { imm32: imm },
            _ => panic_gen_unop_inst("push", imm),
        },
        _ => panic_gen_unop_inst("push", op),
    }
}

#[cfg(test)]
mod r64_tests {
    use super::*;

    #[test]
    fn test1() {
        let inst = Push::R64 {
            r64: GeneralPurposeRegister::RAX,
        };

        assert_eq!(vec![0x50], inst.assemble());
    }
    #[test]
    fn test2() {
        let inst = Push::R64 {
            r64: GeneralPurposeRegister::R15,
        };

        assert_eq!(vec![0x41, 0x57], inst.assemble());
    }
}

#[cfg(test)]
mod imm_tests {
    use super::*;

    #[test]
    fn test1() {
        let inst = Push::Imm8 {
            imm8: Immediate::I8(30),
        };
        assert_eq!(vec![0x6a, 0x1e], inst.assemble());
    }
    #[test]
    fn test2() {
        let inst = Push::Imm16 {
            imm16: Immediate::I16(30),
        };
        assert_eq!(vec![0x68, 0x1e, 0x00], inst.assemble());
    }
    #[test]
    fn test3() {
        let inst = Push::Imm32 {
            imm32: Immediate::I32(30),
        };
        assert_eq!(vec![0x68, 0x1e, 0x00, 0x00, 0x00], inst.assemble());
    }
}
