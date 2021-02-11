use crate::assembler::resource::*;

#[derive(Eq, Ord, PartialOrd, PartialEq, Debug, Clone)]
pub enum Mov {
    /// Move r8 to r/m8
    R8ToRM8 {
        r8: GeneralPurposeRegister,
        rm8: Operand,
    },

    /// Move r32 to r/m32
    R32ToRM32 {
        r32: GeneralPurposeRegister,
        rm32: Operand,
    },

    /// Move r/m32 to r32
    RM32ToR32 {
        r32: GeneralPurposeRegister,
        rm32: Operand,
    },

    /// Move r64 to r/m64
    R64ToRM64 {
        r64: GeneralPurposeRegister,
        rm64: Operand,
    },

    /// Move r/m64 to r64
    RM64ToR64 {
        r64: GeneralPurposeRegister,
        rm64: Operand,
    },
    /// Move imm32 to r32
    Imm32ToR32 {
        imm32: Immediate,
        r32: GeneralPurposeRegister,
    },

    /// Move imm32 to r/m64
    Imm32ToRM64 { imm32: Immediate, rm64: Operand },
}

impl Mov {
    pub fn new(size: OperandSize, src: Operand, dst: Operand) -> Self {
        match size {
            OperandSize::Qword => mov_qword(src, dst),
            OperandSize::Dword => mov_dword(src, dst),
            _ => panic_gen_binop_inst("mov", src, dst),
        }
    }
}

impl Instruction for Mov {
    fn opcode(&self) -> Vec<u8> {
        match self {
            Mov::R8ToRM8 { .. } => vec![0x88],
            Mov::R32ToRM32 { .. } => vec![0x89],
            Mov::RM32ToR32 { .. } => vec![0x8b],
            Mov::Imm32ToR32 { r32, .. } => vec![0xb8 + r32.number()],
            Mov::R64ToRM64 { .. } => vec![0x89],
            Mov::RM64ToR64 { .. } => vec![0x8b],
            Mov::Imm32ToRM64 { .. } => vec![0xc7],
        }
    }

    fn name(&self) -> InstName {
        InstName::Mov
    }

    fn rex_prefix(&self) -> Option<REXPrefix> {
        match self {
            Mov::Imm32ToRM64 { rm64, .. } => Some(REXPrefix::new(
                true,
                false,
                rm64.index_reg_is_expanded(),
                rm64.is_expanded(),
            )),
            Mov::R64ToRM64 { r64, rm64 } => Some(REXPrefix::new(
                true,
                r64.is_expanded(),
                rm64.index_reg_is_expanded(),
                rm64.is_expanded(),
            )),
            Mov::Imm32ToR32 { r32, .. } => {
                if r32.is_expanded() {
                    Some(REXPrefix::new(false, false, false, r32.is_expanded()))
                } else {
                    None
                }
            }
            _ => None,
        }
    }
    fn modrm(&self) -> Option<ModRM> {
        match self {
            Mov::Imm32ToRM64 { rm64, .. } => Some(ModRM::new_mi(rm64.addressing_mode(), rm64)),
            Mov::R32ToRM32 { rm32, r32 } => Some(ModRM::new(self.encoding(), r32, rm32)),
            Mov::R64ToRM64 { rm64, r64 } => Some(ModRM::new(self.encoding(), r64, rm64)),
            _ => None,
        }
    }
    fn immediate(&self) -> Option<Immediate> {
        match self {
            Mov::Imm32ToRM64 { imm32, .. } | Mov::Imm32ToR32 { imm32, .. } => Some(*imm32),

            _ => None,
        }
    }
    fn displacement(&self) -> Option<Displacement> {
        match self {
            Mov::Imm32ToRM64 { rm64: rm, .. }
            | Mov::R32ToRM32 { rm32: rm, .. }
            | Mov::R64ToRM64 { rm64: rm, .. } => rm.displacement(),
            _ => None,
        }
    }
    fn encoding(&self) -> Encoding {
        match self {
            Mov::Imm32ToRM64 { .. } => Encoding::MI,
            Mov::Imm32ToR32 { .. } => Encoding::OI,
            Mov::R32ToRM32 { .. } | Mov::R64ToRM64 { .. } => Encoding::MR,
            _ => todo!(),
        }
    }
}

fn mov_qword(src: Operand, dst: Operand) -> Mov {
    match src {
        Operand::Immediate(imm) => match imm {
            Immediate::I8(v) => mov_imm8_to_qword(v, dst),
            Immediate::I32(_v) => mov_imm32_to_qword(imm, dst),
            _ => panic_gen_binop_inst("mov", src, dst),
        },
        Operand::GeneralReg(reg) => mov_qword_reg_to(reg, dst),
        _ => panic_gen_binop_inst("mov", src, dst),
    }
}

fn mov_dword(src: Operand, dst: Operand) -> Mov {
    match src {
        Operand::Immediate(imm) => match imm {
            Immediate::I8(v) => mov_imm8_to_dword(v, dst),
            Immediate::I32(_v) => mov_imm32_to_dword(imm, dst),
            _ => panic_gen_binop_inst("mov", src, dst),
        },
        Operand::GeneralReg(reg) => mov_dword_reg_to(reg, dst),
        _ => panic_gen_binop_inst("mov", src, dst),
    }
}

/// パース時にi8に収まる即値であれば `Immediate::I8` が作られるが，
/// 例えばr/m32にmov可能なのはimm32のみである．
/// よって，引数にはImmediateではなくraw valueを渡しておく．
fn mov_imm8_to_dword(imm: i8, dst: Operand) -> Mov {
    match dst {
        Operand::GeneralReg(reg) => Mov::Imm32ToR32 {
            imm32: Immediate::I32(imm as i32),
            r32: reg,
        },
        _ => panic_gen_binop_inst("mov", imm, dst),
    }
}
fn mov_imm32_to_dword(imm: Immediate, dst: Operand) -> Mov {
    match dst {
        Operand::GeneralReg(reg) => Mov::Imm32ToR32 {
            imm32: imm,
            r32: reg,
        },
        _ => panic_gen_binop_inst("mov", imm, dst),
    }
}

/// パース時にi8に収まる即値であれば `Immediate::I8` が作られるが，
/// 例えばr/m64にmov可能なのはimm32のみである．
/// よって，引数にはImmediateではなくraw valueを渡しておく．
fn mov_imm8_to_qword(imm: i8, dst: Operand) -> Mov {
    match &dst {
        Operand::GeneralReg(_) => Mov::Imm32ToRM64 {
            imm32: Immediate::I32(imm as i32),
            rm64: dst,
        },
        Operand::Memory(_mem) => Mov::Imm32ToRM64 {
            imm32: Immediate::I32(imm as i32),
            rm64: dst,
        },
        _ => panic_gen_binop_inst("mov", imm, dst),
    }
}
fn mov_imm32_to_qword(imm: Immediate, dst: Operand) -> Mov {
    match &dst {
        // レジスタサイズが64bitの時はバイト数削減の為にr/m64としてアセンブルする．
        // これはGNU asも用いている手法．
        Operand::GeneralReg(_) => Mov::Imm32ToRM64 {
            imm32: imm,
            rm64: dst,
        },
        Operand::Memory(_mem) => Mov::Imm32ToRM64 {
            imm32: imm,
            rm64: dst,
        },
        _ => panic_gen_binop_inst("mov", imm, dst),
    }
}

fn mov_dword_reg_to(reg: GeneralPurposeRegister, dst: Operand) -> Mov {
    match &dst {
        Operand::GeneralReg(_dst) => Mov::R32ToRM32 {
            r32: reg,
            rm32: dst,
        },
        Operand::Memory(_mem) => Mov::R32ToRM32 {
            r32: reg,
            rm32: dst,
        },
        _ => panic_gen_binop_inst("mov", reg, dst),
    }
}
fn mov_qword_reg_to(reg: GeneralPurposeRegister, dst: Operand) -> Mov {
    match &dst {
        Operand::GeneralReg(_dst) => Mov::R64ToRM64 {
            r64: reg,
            rm64: dst,
        },
        Operand::Memory(_mem) => Mov::R64ToRM64 {
            r64: reg,
            rm64: dst,
        },
        _ => panic_gen_binop_inst("mov", reg, dst),
    }
}

#[cfg(test)]
mod imm32_to_rm32_tests {
    use super::*;

    #[test]
    fn test1() {
        // movl %eax, %ebx
        let inst = to_reg_setup(GeneralPurposeRegister::RAX, GeneralPurposeRegister::RBX);
        assert_eq!(vec![0x89, 0xc3], inst.assemble());
    }

    fn to_reg_setup(r32: GeneralPurposeRegister, reg: GeneralPurposeRegister) -> Mov {
        Mov::R32ToRM32 {
            r32,
            rm32: Operand::GeneralReg(reg),
        }
    }
}

#[cfg(test)]
mod imm_to_r32_tests {
    use super::*;

    #[test]
    fn test1() {
        let inst = to_reg_setup(Immediate::I32(42), GeneralPurposeRegister::EAX);

        assert_eq!(vec![0xb8, 0x2a, 0x00, 0x00, 0x00], inst.assemble());
    }

    #[test]
    fn test2() {
        let inst = to_reg_setup(Immediate::I32(42), GeneralPurposeRegister::R15D);

        assert_eq!(vec![0x41, 0xbf, 0x2a, 0x00, 0x00, 0x00], inst.assemble());
    }

    fn to_reg_setup(imm: Immediate, reg: GeneralPurposeRegister) -> Mov {
        Mov::Imm32ToR32 {
            imm32: imm,
            r32: reg,
        }
    }
}

#[cfg(test)]
mod imm_to_rm64_tests {
    use super::*;

    #[test]
    fn test1() {
        let inst = to_reg_setup(Immediate::I32(30), GeneralPurposeRegister::RAX);

        assert_eq!(
            vec![0x48, 0xc7, 0xc0, 0x1e, 0x00, 0x00, 0x00],
            inst.assemble()
        );
    }

    #[test]
    fn test2() {
        let inst = to_reg_setup(Immediate::I32(30), GeneralPurposeRegister::R15);

        assert_eq!(
            vec![0x49, 0xc7, 0xc7, 0x1e, 0x00, 0x00, 0x00],
            inst.assemble()
        );
    }

    #[test]
    fn test3() {
        let inst = to_mem_setup(
            Immediate::I32(30),
            OpMemory {
                base: GeneralPurposeRegister::RBP,
                disp: Some(Displacement::Disp8(-8)),
                index: None,
                scale: None,
            },
        );

        assert_eq!(
            vec![0x48, 0xc7, 0x45, 0xf8, 0x1e, 0x00, 0x00, 0x00],
            inst.assemble()
        );
    }

    fn to_reg_setup(imm: Immediate, reg: GeneralPurposeRegister) -> Mov {
        Mov::Imm32ToRM64 {
            imm32: imm,
            rm64: Operand::GeneralReg(reg),
        }
    }
    fn to_mem_setup(imm: Immediate, mem: OpMemory) -> Mov {
        Mov::Imm32ToRM64 {
            imm32: imm,
            rm64: Operand::Memory(mem),
        }
    }
}

#[cfg(test)]
mod r64_to_rm64_tests {
    use super::*;

    #[test]
    fn test1() {
        // movq %rbx, %rax
        let inst = to_reg_setup(GeneralPurposeRegister::RBX, GeneralPurposeRegister::RAX);
        assert_eq!(vec![0x48, 0x89, 0xd8], inst.assemble());
    }
    #[test]
    fn test2() {
        // movq %rbx, %r8
        let inst = to_reg_setup(GeneralPurposeRegister::RBX, GeneralPurposeRegister::R8);
        assert_eq!(vec![0x49, 0x89, 0xd8], inst.assemble());
    }
    #[test]
    fn test3() {
        // movq %r8, %rax
        let inst = to_reg_setup(GeneralPurposeRegister::R8, GeneralPurposeRegister::RAX);
        assert_eq!(vec![0x4c, 0x89, 0xc0], inst.assemble());
    }
    #[test]
    fn test4() {
        // movq %r15, %r8
        let inst = to_reg_setup(GeneralPurposeRegister::R15, GeneralPurposeRegister::R8);
        assert_eq!(vec![0x4d, 0x89, 0xf8], inst.assemble());
    }

    #[test]
    fn test5() {
        // movq %rax, -0x8(%rbp)
        let inst = to_mem_setup(
            GeneralPurposeRegister::RAX,
            OpMemory {
                base: GeneralPurposeRegister::RBP,
                disp: Some(Displacement::Disp8(-8)),
                index: None,
                scale: None,
            },
        );
        assert_eq!(vec![0x48, 0x89, 0x45, 0xf8], inst.assemble());
    }

    fn to_reg_setup(r64: GeneralPurposeRegister, reg: GeneralPurposeRegister) -> Mov {
        Mov::R64ToRM64 {
            r64,
            rm64: Operand::GeneralReg(reg),
        }
    }
    fn to_mem_setup(r64: GeneralPurposeRegister, mem: OpMemory) -> Mov {
        Mov::R64ToRM64 {
            r64,
            rm64: Operand::Memory(mem),
        }
    }
}

#[cfg(test)]
mod r32_to_rm32_tests {
    use super::*;

    #[test]
    fn test1() {
        // movl %eax, %ebx
        let inst = to_reg_setup(GeneralPurposeRegister::RAX, GeneralPurposeRegister::RBX);
        assert_eq!(vec![0x89, 0xc3], inst.assemble());
    }

    fn to_reg_setup(r32: GeneralPurposeRegister, reg: GeneralPurposeRegister) -> Mov {
        Mov::R32ToRM32 {
            r32,
            rm32: Operand::GeneralReg(reg),
        }
    }
}
