use crate::assembler::resource::*;

#[allow(dead_code)]
#[derive(Eq, Ord, PartialOrd, PartialEq, Debug, Clone)]
pub enum Add {
    /// Add r/m32 to r32
    RM32ToR32 {
        rm32: Operand,
        r32: GeneralPurposeRegister,
    },

    /// Add r32 to r/m32
    R32ToRM32 {
        r32: GeneralPurposeRegister,
        rm32: Operand,
    },

    /// Add r/m64 to r64
    RM64ToR64 {
        rm64: Operand,
        r64: GeneralPurposeRegister,
    },

    /// Add r64 to r/m64
    R64ToRM64 {
        r64: GeneralPurposeRegister,
        rm64: Operand,
    },

    /// Add imm8 to RM64
    Imm8ToRM64 { imm8: Immediate, rm64: Operand },
    /// Add imm32 to RM64
    Imm32ToRM64 { imm32: Immediate, rm64: Operand },
    /// Add imm8 to RM32
    Imm8ToRM32 { imm8: Immediate, rm32: Operand },
    /// Add imm32 to RM32
    Imm32ToRM32 { imm32: Immediate, rm32: Operand },
}
impl Add {
    pub fn new(size: OperandSize, src: Operand, dst: Operand) -> Self {
        match size {
            OperandSize::Qword => add_qword(src, dst),
            OperandSize::Dword => add_dword(src, dst),
            _ => panic!("cannot generate 'add {}, {}'", src, dst),
        }
    }
}

impl Instruction for Add {
    fn opcode(&self) -> Vec<u8> {
        match self {
            Add::R64ToRM64 { .. } => vec![0x01],
            Add::Imm8ToRM64 { .. } => vec![0x83],
            Add::Imm32ToRM64 { .. } => vec![0x81],
            _ => todo!(),
        }
    }

    fn rex_prefix(&self) -> Option<REXPrefix> {
        match self {
            Add::R64ToRM64 { r64, rm64 } => Some(REXPrefix::new(
                true,
                r64.is_expanded(),
                rm64.index_reg_is_expanded(),
                rm64.is_expanded(),
            )),
            Add::Imm8ToRM64 { imm8: _, rm64 } | Add::Imm32ToRM64 { imm32: _, rm64 } => {
                Some(REXPrefix::new(
                    true,
                    false,
                    rm64.index_reg_is_expanded(),
                    rm64.is_expanded(),
                ))
            }
            _ => todo!(),
        }
    }
    fn modrm(&self) -> Option<ModRM> {
        match self {
            Add::R64ToRM64 { r64, rm64 } => Some(ModRM::new(self.encoding(), r64, rm64)),
            Add::Imm8ToRM64 { imm8: _, rm64 } | Add::Imm32ToRM64 { imm32: _, rm64 } => {
                Some(ModRM::new_mi(rm64.addressing_mode(), rm64))
            }
            _ => todo!(),
        }
    }
    fn encoding(&self) -> Encoding {
        match self {
            Add::R64ToRM64 { .. } => Encoding::MR,
            Add::Imm8ToRM64 { .. } | Add::Imm32ToRM64 { .. } => Encoding::MI,
            _ => todo!(),
        }
    }
    fn displacement(&self) -> Option<Displacement> {
        match self {
            Add::R64ToRM64 { r64: _, rm64 } => rm64.displacement(),
            Add::Imm8ToRM64 { imm8: _, rm64 } | Add::Imm32ToRM64 { imm32: _, rm64 } => {
                rm64.displacement()
            }
            _ => todo!(),
        }
    }

    fn immediate(&self) -> Option<Immediate> {
        match self {
            Add::Imm8ToRM64 { imm8: imm, .. } | Add::Imm32ToRM64 { imm32: imm, .. } => Some(*imm),
            _ => None,
        }
    }

    fn name(&self) -> InstName {
        todo!()
    }
}

fn add_dword(src: Operand, dst: Operand) -> Add {
    if src.size() > dst.size() {
        panic!("cannot add {} to {} (oversize)", src, dst)
    }

    match src {
        Operand::GeneralReg(src_gpr) => add_dword_reg_to(src_gpr, dst),
        Operand::Immediate(imm) => match imm {
            Immediate::I8(_v) => add_imm8_to_dword(imm, dst),
            Immediate::I32(_v) => add_imm32_to_dword(imm, dst),
            _ => panic!("cannot generate 'add {}, {}'", src, dst),
        },
        _ => panic!("cannot generate 'add {}, {}'", src, dst),
    }
}

fn add_imm16_to(imm: Immediate, dst: Operand) -> Add {
    match dst {
        Operand::GeneralReg(dst) => match dst.size() {
            _ => panic!("cannot generate 'add {}, {}'", imm, dst),
        },
        _ => panic!("cannot generate 'add {}, {}'", imm, dst),
    }
}
fn add_imm32_to(imm: Immediate, dst: Operand) -> Add {
    match dst {
        Operand::GeneralReg(dst) => match dst.size() {
            _ => panic!("cannot generate 'add {}, {}'", imm, dst),
        },
        _ => panic!("cannot generate 'add {}, {}'", imm, dst),
    }
}

fn add_dword_reg_to(src: GeneralPurposeRegister, dst: Operand) -> Add {
    match dst {
        // add %src, %dst
        Operand::GeneralReg(_dst_gpr) => Add::R32ToRM32 {
            r32: src,
            rm32: dst,
        },
        // add %src, (%dst)
        Operand::Memory(ref _dst_mem) => Add::R32ToRM32 {
            r32: src,
            rm32: dst,
        },

        _ => panic!("cannot generate 'add {}, {}'", src, dst),
    }
}

fn add_qword(src: Operand, dst: Operand) -> Add {
    match src {
        Operand::Immediate(imm) => match imm {
            Immediate::I8(_v) => add_imm8_to_qword(imm, dst),
            Immediate::I32(_v) => add_imm32_to_qword(imm, dst),
            _ => panic!("cannot generate 'add {}, {}'", src, dst),
        },
        _ => panic!("cannot generate 'add {}, {}'", src, dst),
    }
}
fn add_imm8_to_dword(imm: Immediate, dst: Operand) -> Add {
    match &dst {
        Operand::GeneralReg(_reg) => Add::Imm8ToRM32 {
            imm8: imm,
            rm32: dst,
        },
        Operand::Memory(_mem) => Add::Imm8ToRM32 {
            imm8: imm,
            rm32: dst,
        },
        _ => panic!("cannot generate 'add {}, {}'", imm, dst),
    }
}

fn add_imm32_to_dword(imm: Immediate, dst: Operand) -> Add {
    match &dst {
        Operand::GeneralReg(_reg) => Add::Imm32ToRM32 {
            imm32: imm,
            rm32: dst,
        },
        Operand::Memory(_mem) => Add::Imm32ToRM32 {
            imm32: imm,
            rm32: dst,
        },
        _ => panic!("cannot generate 'add {}, {}'", imm, dst),
    }
}

fn add_imm8_to_qword(imm: Immediate, dst: Operand) -> Add {
    match &dst {
        Operand::GeneralReg(_reg) => Add::Imm8ToRM64 {
            imm8: imm,
            rm64: dst,
        },
        Operand::Memory(_mem) => Add::Imm8ToRM64 {
            imm8: imm,
            rm64: dst,
        },
        _ => panic!("cannot generate 'add {}, {}'", imm, dst),
    }
}

fn add_imm32_to_qword(imm: Immediate, dst: Operand) -> Add {
    match &dst {
        Operand::GeneralReg(_reg) => Add::Imm32ToRM64 {
            imm32: imm,
            rm64: dst,
        },
        Operand::Memory(_mem) => Add::Imm32ToRM64 {
            imm32: imm,
            rm64: dst,
        },
        _ => panic!("cannot generate 'add {}, {}'", imm, dst),
    }
}
#[cfg(test)]
mod r64_to_rm64_tests {
    use super::*;

    #[test]
    fn test1() {
        // add %rbx, %rax
        let inst = reg_and_reg_setup(GeneralPurposeRegister::RBX, GeneralPurposeRegister::RAX);

        assert_eq!(vec![0x48, 0x01, 0xd8], inst.assemble());
    }

    #[test]
    fn test2() {
        // add %r8, %rax
        let inst = reg_and_reg_setup(GeneralPurposeRegister::R8, GeneralPurposeRegister::RAX);

        assert_eq!(vec![0x4c, 0x01, 0xc0], inst.assemble());
    }
    #[test]
    fn test3() {
        // add %rax, %r8
        let inst = reg_and_reg_setup(GeneralPurposeRegister::RAX, GeneralPurposeRegister::R8);

        assert_eq!(vec![0x49, 0x01, 0xc0], inst.assemble());
    }
    #[test]
    fn test4() {
        // add %r9, %r8
        let inst = reg_and_reg_setup(GeneralPurposeRegister::R9, GeneralPurposeRegister::R8);

        assert_eq!(vec![0x4d, 0x01, 0xc8], inst.assemble());
    }

    #[test]
    fn test5() {
        let inst = reg_and_mem_setup(
            GeneralPurposeRegister::RAX,
            OpMemory {
                base: GeneralPurposeRegister::RBP,
                index: None,
                scale: None,
                disp: Some(Displacement::Disp8(-8)),
            },
        );

        assert_eq!(vec![0x48, 0x01, 0x45, 0xf8], inst.assemble());
    }
    #[test]
    fn test6() {
        let inst = reg_and_mem_setup(
            GeneralPurposeRegister::R8,
            OpMemory {
                base: GeneralPurposeRegister::RBP,
                index: None,
                scale: None,
                disp: Some(Displacement::Disp8(-8)),
            },
        );

        assert_eq!(vec![0x4c, 0x01, 0x45, 0xf8], inst.assemble());
    }
    #[test]
    fn test7() {
        let inst = reg_and_mem_setup(
            GeneralPurposeRegister::RAX,
            OpMemory {
                base: GeneralPurposeRegister::R15,
                index: None,
                scale: None,
                disp: Some(Displacement::Disp8(-8)),
            },
        );

        assert_eq!(vec![0x49, 0x01, 0x47, 0xf8], inst.assemble());
    }
    #[test]
    fn test8() {
        let inst = reg_and_mem_setup(
            GeneralPurposeRegister::R8,
            OpMemory {
                base: GeneralPurposeRegister::R15,
                index: None,
                scale: None,
                disp: Some(Displacement::Disp8(-8)),
            },
        );

        assert_eq!(vec![0x4d, 0x01, 0x47, 0xf8], inst.assemble());
    }

    fn reg_and_reg_setup(r64: GeneralPurposeRegister, rm64: GeneralPurposeRegister) -> Add {
        Add::R64ToRM64 {
            r64,
            rm64: Operand::GeneralReg(rm64),
        }
    }
    fn reg_and_mem_setup(r64: GeneralPurposeRegister, rm64: OpMemory) -> Add {
        Add::R64ToRM64 {
            r64,
            rm64: Operand::Memory(rm64),
        }
    }
}

#[cfg(test)]
mod imm_to_rm64_tests {
    use super::*;

    #[test]
    fn imm8_test1() {
        let inst = imm_and_reg_setup(
            OperandSize::Byte,
            Immediate::I8(3),
            Operand::GeneralReg(GeneralPurposeRegister::RAX),
        );

        assert_eq!(vec![0x48, 0x83, 0xc0, 0x03], inst.assemble());
    }

    #[test]
    fn imm8_test2() {
        let inst = imm_and_reg_setup(
            OperandSize::Byte,
            Immediate::I8(3),
            Operand::Memory(OpMemory {
                base: GeneralPurposeRegister::RBP,
                index: None,
                disp: Some(Displacement::Disp8(-8)),
                scale: None,
            }),
        );

        assert_eq!(vec![0x48, 0x83, 0x45, 0xf8, 0x03], inst.assemble());
    }

    #[test]
    fn imm32_test1() {
        let inst = imm_and_reg_setup(
            OperandSize::Dword,
            Immediate::I32(3),
            Operand::GeneralReg(GeneralPurposeRegister::RAX),
        );

        assert_eq!(
            vec![0x48, 0x81, 0xc0, 0x03, 0x00, 0x00, 0x00],
            inst.assemble()
        );
    }

    #[test]
    fn imm32_test2() {
        let inst = imm_and_reg_setup(
            OperandSize::Dword,
            Immediate::I32(3),
            Operand::Memory(OpMemory {
                base: GeneralPurposeRegister::RBP,
                index: None,
                disp: Some(Displacement::Disp8(-8)),
                scale: None,
            }),
        );

        assert_eq!(
            vec![0x48, 0x81, 0x45, 0xf8, 0x03, 0x00, 0x00, 0x00],
            inst.assemble()
        );
    }

    fn imm_and_reg_setup(size: OperandSize, imm: Immediate, rm64: Operand) -> Add {
        match size {
            OperandSize::Byte => Add::Imm8ToRM64 { imm8: imm, rm64 },
            OperandSize::Dword => Add::Imm32ToRM64 { imm32: imm, rm64 },
            _ => unreachable!(),
        }
    }
}
