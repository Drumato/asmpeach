mod add;
pub use add::*;
mod call;
pub use call::*;
mod cmp;
pub use cmp::*;
mod mov;
pub use mov::*;
mod push;
pub use push::*;
mod pop;
pub use pop::*;
mod sub;
pub use sub::*;
mod lea;
pub use lea::*;
mod imul;
pub use imul::*;
mod idiv;
pub use idiv::*;
mod inc;
pub use inc::*;
mod jmp;
pub use jmp::*;
mod neg;
pub use neg::*;
mod syscall;
pub use syscall::*;
mod ret;
pub use ret::*;
mod endbr64;
pub use endbr64::*;

use super::{Displacement, Immediate, ModRM, Operand, REXPrefix, SIBByte};

pub trait Instruction {
    fn opcode(&self) -> Vec<u8>;
    fn name(&self) -> InstName;
    fn encoding(&self) -> Encoding {
        Encoding::ZO
    }
    fn operand_1(&self) -> Option<Operand> {
        None
    }
    fn rex_prefix(&self) -> Option<REXPrefix> {
        None
    }
    fn modrm(&self) -> Option<ModRM> {
        None
    }
    fn sib_byte(&self) -> Option<SIBByte> {
        None
    }
    fn displacement(&self) -> Option<Displacement> {
        None
    }
    fn immediate(&self) -> Option<Immediate> {
        None
    }

    fn assemble(&self) -> Vec<u8> {
        let mut codes = Vec::new();

        if let Some(rex_prefix) = self.rex_prefix() {
            codes.push(rex_prefix.to_byte());
        }

        codes.append(&mut self.opcode());

        if let Some(modrm) = self.modrm() {
            codes.push(modrm.to_byte());
        }

        if let Some(sib_byte) = self.sib_byte() {
            codes.push(sib_byte.to_byte());
        }

        if let Some(disp) = self.displacement() {
            codes.append(&mut disp.to_bytes());
        }

        if let Some(imm) = self.immediate() {
            codes.append(&mut imm.to_bytes());
        }
        codes
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
pub enum InstName {
    Call,
    Jmp,
}

pub struct Group {
    pub label: String,
    pub insts: Vec<Box<dyn Instruction>>,
}

#[allow(dead_code)]
#[derive(Eq, Ord, PartialOrd, PartialEq, Debug, Clone, Copy)]
pub enum Encoding {
    ZO,
    D,

    /// Ope1 -> ModRM:reg,   Ope2 -> ModRM:r/m
    RM,
    /// Ope1 -> ModRM:r/m,   Ope2 -> ModRM:reg
    MR,
    /// Ope1 -> ModRM:r/m,   Ope2 -> imm8/16/32/64
    MI,
    /// Ope1 -> opcode + rd, Ope2 -> imm8/16/32/64
    OI,
    /// Ope1 -> opcode + rd
    O,
    /// Ope1 -> imm8/16/32
    I,
    /// Ope1 -> ModRM:r/m
    M,
}

impl Group {
    pub fn new(label: &str) -> Self {
        Self {
            label: label.to_string(),
            insts: Vec::new(),
        }
    }
}

impl Default for Group {
    fn default() -> Self {
        Self {
            label: String::new(),
            insts: Vec::new(),
        }
    }
}
