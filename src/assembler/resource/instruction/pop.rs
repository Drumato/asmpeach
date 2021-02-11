use crate::assembler::resource::*;

#[derive(Eq, Ord, PartialOrd, PartialEq, Debug, Clone)]
pub enum Pop {
    R64 { r64: GeneralPurposeRegister },
    RM64 { rm64: Operand },
}

impl Pop {
    pub fn new(size: OperandSize, op: Operand) -> Self {
        match size {
            OperandSize::Qword => pop_qword(op),
            _ => panic_gen_unop_inst("pop", op),
        }
    }
}

impl Instruction for Pop {
    fn opcode(&self) -> Vec<u8> {
        match self {
            Pop::R64 { r64 } => vec![0x58 + r64.number()],
            Pop::RM64 { .. } => vec![0x8f],
        }
    }

    fn name(&self) -> InstName {
        InstName::Pop
    }
    fn rex_prefix(&self) -> Option<REXPrefix> {
        match self {
            // r64が拡張レジスタの場合のみREX-prefixが付くので注意
            Pop::R64 { r64 } => {
                if r64.is_expanded() {
                    Some(REXPrefix::new(false, false, false, r64.is_expanded()))
                } else {
                    None
                }
            }
            // rm64のベースレジスタが拡張レジスタの場合のみREX-prefixが付くので注意
            Pop::RM64 { rm64 } => {
                if rm64.is_expanded() {
                    Some(REXPrefix::new(
                        false,
                        false,
                        rm64.index_reg_is_expanded(),
                        rm64.is_expanded(),
                    ))
                } else {
                    None
                }
            }
        }
    }
    fn modrm(&self) -> Option<ModRM> {
        match self {
            Pop::RM64 { rm64 } => Some(ModRM::new_m(rm64.addressing_mode(), rm64)),
            _ => None,
        }
    }
    fn displacement(&self) -> Option<Displacement> {
        match self {
            Pop::RM64 { rm64 } => rm64.displacement(),
            _ => None,
        }
    }
}

fn pop_qword(op: Operand) -> Pop {
    match op {
        Operand::GeneralReg(reg) => Pop::R64 { r64: reg },
        _ => panic_gen_unop_inst("pop", op),
    }
}

#[cfg(test)]
mod r64_tests {
    use super::*;

    #[test]
    fn test1() {
        let inst = Pop::R64 {
            r64: GeneralPurposeRegister::RAX,
        };
        assert_eq!(vec![0x58], inst.assemble());
    }

    #[test]
    fn test2() {
        let inst = Pop::R64 {
            r64: GeneralPurposeRegister::R15,
        };
        assert_eq!(vec![0x41, 0x5f], inst.assemble());
    }
}

#[cfg(test)]
mod rm64_tests {
    use super::*;

    #[test]
    fn test1() {
        let inst = Pop::RM64 {
            rm64: Operand::Memory(OpMemory {
                base: GeneralPurposeRegister::RAX,
                disp: Some(Displacement::Disp8(-8)),
                index: None,
                scale: None,
            }),
        };

        assert_eq!(vec![0x8f, 0x40, 0xf8], inst.assemble());
    }

    #[test]
    fn test2() {
        let inst = Pop::RM64 {
            rm64: Operand::Memory(OpMemory {
                base: GeneralPurposeRegister::R15,
                disp: Some(Displacement::Disp8(-8)),
                index: None,
                scale: None,
            }),
        };

        assert_eq!(vec![0x41, 0x8f, 0x47, 0xf8], inst.assemble());
    }
}
