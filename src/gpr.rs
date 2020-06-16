use std::fmt;
use fmt::Formatter;

#[derive(Eq, Ord, PartialOrd, PartialEq, Debug, Clone, Copy)]
pub enum GeneralPurposeRegister {
    // 8bit general-purpose registers
    AH,
    BH,
    CH,
    DH,
    AL,
    BL,
    CL,
    DL,

    // 64bit general-purpose registers
    RAX,
    RBP,
    RSP,
    RDI,
    RSI,
    RDX,
    RCX,
    RBX,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
}

impl GeneralPurposeRegister {
    pub fn number(&self) -> u8 {
        match self {
            GeneralPurposeRegister::AL | GeneralPurposeRegister::RAX | GeneralPurposeRegister::R8 => 0,
            GeneralPurposeRegister::CL | GeneralPurposeRegister::RCX | GeneralPurposeRegister::R9 => 1,
            GeneralPurposeRegister::DL | GeneralPurposeRegister::RDX | GeneralPurposeRegister::R10 => 2,
            GeneralPurposeRegister::BL | GeneralPurposeRegister::RBX | GeneralPurposeRegister::R11 => 3,
            GeneralPurposeRegister::AH | GeneralPurposeRegister::RSP | GeneralPurposeRegister::R12 => 4,
            GeneralPurposeRegister::CH | GeneralPurposeRegister::RBP | GeneralPurposeRegister::R13 => 5,
            GeneralPurposeRegister::DH | GeneralPurposeRegister::RSI | GeneralPurposeRegister::R14 => 6,
            GeneralPurposeRegister::BH | GeneralPurposeRegister::RDI | GeneralPurposeRegister::R15 => 7,
        }
    }

    #[allow(clippy::match_single_binding)]
    pub fn is_8bit(&self) -> bool {
        match self{
            _ => false,
        }
    }

    pub fn to_at(&self) -> String {
        format!("%{}", self.to_str())
    }

    pub fn to_intel(&self) -> String {
        self.to_str().to_string()
    }

    /// 拡張されたレジスタかどうかのチェック
    /// REX prefixに用いる
    pub fn is_expanded(&self) -> bool {
        match self {
            Self::R8 | Self::R9 | Self::R10 | Self::R11 | Self::R12 | Self::R13 | Self::R14 | Self::R15 => true,
            _ => false,
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            // 8bit general-purpose registers
            GeneralPurposeRegister::AH => "ah",
            GeneralPurposeRegister::BH => "bh",
            GeneralPurposeRegister::CH => "ch",
            GeneralPurposeRegister::DH => "dh",
            GeneralPurposeRegister::AL => "al",
            GeneralPurposeRegister::BL => "bl",
            GeneralPurposeRegister::CL => "cl",
            GeneralPurposeRegister::DL => "dl",


            // 64bit general-purpose registers
            Self::RAX => "rax",
            Self::RCX => "rcx",
            Self::RDX => "rdx",
            Self::RBX => "rbx",
            Self::RSP => "rsp",
            Self::RBP => "rbp",
            Self::RSI => "rsi",
            Self::RDI => "rdi",
            Self::R8 => "r8",
            Self::R9 => "r9",
            Self::R10 => "r10",
            Self::R11 => "r11",
            Self::R12 => "r12",
            Self::R13 => "r13",
            Self::R14 => "r14",
            Self::R15 => "r15",
        }
    }
}

impl fmt::Display for GeneralPurposeRegister {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Register::{}", self.to_at())
    }
}