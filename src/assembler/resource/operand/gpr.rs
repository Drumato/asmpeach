use fmt::Formatter;
use std::fmt;

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

    // 32bit general-purpose registers

    /// Accumulator Register
    EAX,

    /// (Stack) Base Pointer Register
    EBP,
    /// Stack Pointer Register
    ESP,
    /// Destination Index Register
    EDI,
    /// Source Index Register
    ESI,
    /// Data Register
    EDX,
    /// Counter Register
    ECX,
    /// Base Register
    EBX,

    // 64bit general-purpose registers
    /// Accumulator Register
    RAX,

    /// (Stack) Base Pointer Register
    RBP,
    /// Stack Pointer Register
    RSP,
    /// Destination Index Register
    RDI,
    /// Source Index Register
    RSI,
    /// Data Register
    RDX,
    /// Counter Register
    RCX,
    /// Base Register
    RBX,

    // x64 appended registers
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
    /// register code
    pub fn number(&self) -> u8 {
        match self {
            GeneralPurposeRegister::AL
            | GeneralPurposeRegister::EAX
            | GeneralPurposeRegister::RAX
            | GeneralPurposeRegister::R8 => 0,
            GeneralPurposeRegister::CL
            | GeneralPurposeRegister::ECX
            | GeneralPurposeRegister::RCX
            | GeneralPurposeRegister::R9 => 1,
            GeneralPurposeRegister::DL
            | GeneralPurposeRegister::EDX
            | GeneralPurposeRegister::RDX
            | GeneralPurposeRegister::R10 => 2,
            GeneralPurposeRegister::BL
            | GeneralPurposeRegister::EBX
            | GeneralPurposeRegister::RBX
            | GeneralPurposeRegister::R11 => 3,
            GeneralPurposeRegister::AH
            | GeneralPurposeRegister::ESP
            | GeneralPurposeRegister::RSP
            | GeneralPurposeRegister::R12 => 4,
            GeneralPurposeRegister::CH
            | GeneralPurposeRegister::EBP
            | GeneralPurposeRegister::RBP
            | GeneralPurposeRegister::R13 => 5,
            GeneralPurposeRegister::DH
            | GeneralPurposeRegister::ESI
            | GeneralPurposeRegister::RSI
            | GeneralPurposeRegister::R14 => 6,
            GeneralPurposeRegister::BH
            | GeneralPurposeRegister::EDI
            | GeneralPurposeRegister::RDI
            | GeneralPurposeRegister::R15 => 7,
        }
    }

    pub fn size(&self) -> RegisterSize {
        match self {
            // 8bit
            GeneralPurposeRegister::AL
            | GeneralPurposeRegister::CL
            | GeneralPurposeRegister::DL
            | GeneralPurposeRegister::BL
            | GeneralPurposeRegister::AH
            | GeneralPurposeRegister::CH
            | GeneralPurposeRegister::DH
            | GeneralPurposeRegister::BH => RegisterSize::S8,

            // 32bit
            GeneralPurposeRegister::EAX
            | GeneralPurposeRegister::ECX
            | GeneralPurposeRegister::EDX
            | GeneralPurposeRegister::EBX
            | GeneralPurposeRegister::ESP
            | GeneralPurposeRegister::EBP
            | GeneralPurposeRegister::ESI
            | GeneralPurposeRegister::EDI => RegisterSize::S32,
            _ => RegisterSize::S64,
        }
    }

    /// check whether a register is expanded after x64.
    /// 拡張されたレジスタかどうかのチェック
    /// REX prefixに用いる
    pub fn is_expanded(&self) -> bool {
        match self {
            Self::R8
            | Self::R9
            | Self::R10
            | Self::R11
            | Self::R12
            | Self::R13
            | Self::R14
            | Self::R15 => true,
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

                        // 32bit general-purpose registers
                        Self::EAX => "eax",
                        Self::ECX => "ecx",
                        Self::EDX => "edx",
                        Self::EBX => "ebx",
                        Self::ESP => "esp",
                        Self::EBP => "ebp",
                        Self::ESI => "esi",
                        Self::EDI => "edi",

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

    pub fn new_64bit_from_code(code: usize) -> Self {
        match code {
            0 => GeneralPurposeRegister::RAX,
            1 => GeneralPurposeRegister::RCX,
            2 => GeneralPurposeRegister::RDX,
            3 => GeneralPurposeRegister::RBX,
            4 => GeneralPurposeRegister::RSP,
            5 => GeneralPurposeRegister::RBP,
            6 => GeneralPurposeRegister::RSI,
            7 => GeneralPurposeRegister::RDI,
            _ => unimplemented!(),
        }
    }

    pub fn from_at_string(s: &str) -> Self {
        match s {
            // 32bit
            "%eax" => GeneralPurposeRegister::EAX,
            "%ecx" => GeneralPurposeRegister::ECX,
            "%edx" => GeneralPurposeRegister::EDX,
            "%ebx" => GeneralPurposeRegister::EBX,
            "%esp" => GeneralPurposeRegister::ESP,
            "%ebp" => GeneralPurposeRegister::EBP,
            "%esi" => GeneralPurposeRegister::ESI,
            "%edi" => GeneralPurposeRegister::EDI,

            // 64bit
            "%rax" => GeneralPurposeRegister::RAX,
            "%rcx" => GeneralPurposeRegister::RCX,
            "%rdx" => GeneralPurposeRegister::RDX,
            "%rbx" => GeneralPurposeRegister::RBX,
            "%rsp" => GeneralPurposeRegister::RSP,
            "%rbp" => GeneralPurposeRegister::RBP,
            "%rsi" => GeneralPurposeRegister::RSI,
            "%rdi" => GeneralPurposeRegister::RDI,
            "%r8" => GeneralPurposeRegister::R8,
            "%r9" => GeneralPurposeRegister::R9,
            "%10" => GeneralPurposeRegister::R10,
            "%11" => GeneralPurposeRegister::R11,
            "%12" => GeneralPurposeRegister::R12,
            "%13" => GeneralPurposeRegister::R13,
            "%14" => GeneralPurposeRegister::R14,
            "%15" => GeneralPurposeRegister::R15,
            _ => panic!("{} is not a register", s),
        }
    }

    pub fn to_32bit(&self) -> Self {
        match self {
            // 8bit general-purpose registers
            GeneralPurposeRegister::AH | GeneralPurposeRegister::AL => Self::EAX,
            GeneralPurposeRegister::BH | GeneralPurposeRegister::BL => Self::EBX,
            GeneralPurposeRegister::CH | GeneralPurposeRegister::CL => Self::ECX,
            GeneralPurposeRegister::DH | GeneralPurposeRegister::DL => Self::EDX,

            // 64bit general-purpose registers
            _ => *self,
        }
    }

    pub fn to_64bit(&self) -> Self {
        match self {
            // 8bit general-purpose registers
            GeneralPurposeRegister::AH | GeneralPurposeRegister::AL => Self::RAX,
            GeneralPurposeRegister::BH | GeneralPurposeRegister::BL => Self::RBX,
            GeneralPurposeRegister::CH | GeneralPurposeRegister::CL => Self::RCX,
            GeneralPurposeRegister::DH | GeneralPurposeRegister::DL => Self::RDX,

            // 64bit general-purpose registers
            _ => *self,
        }
    }

    pub fn to_intel_string(&self) -> String {
        self.to_str().to_string()
    }

    pub fn to_at_string(&self) -> String {
        format!("%{}", self.to_str())
    }
}

impl fmt::Display for GeneralPurposeRegister {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Register::{}", self.to_str())
    }
}

#[derive(Eq, Ord, PartialOrd, PartialEq, Debug, Clone, Copy)]
pub enum RegisterSize {
    S8,
    S16,
    S32,
    S64,
}