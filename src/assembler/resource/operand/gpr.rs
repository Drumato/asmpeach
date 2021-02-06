use fmt::Formatter;
use std::fmt;

use super::OperandSize;

#[allow(dead_code)]
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

    // 16bit general-purpose registers
    AX,
    BX,
    CX,
    DX,
    SI,
    DI,
    BP,
    SP,

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

#[allow(dead_code)]
impl GeneralPurposeRegister {
    /// register code
    pub fn number(&self) -> u8 {
        match self {
            GeneralPurposeRegister::AL
            | GeneralPurposeRegister::AX
            | GeneralPurposeRegister::EAX
            | GeneralPurposeRegister::RAX
            | GeneralPurposeRegister::R8 => 0,
            GeneralPurposeRegister::CL
            | GeneralPurposeRegister::CX
            | GeneralPurposeRegister::ECX
            | GeneralPurposeRegister::RCX
            | GeneralPurposeRegister::R9 => 1,
            GeneralPurposeRegister::DL
            | GeneralPurposeRegister::DX
            | GeneralPurposeRegister::EDX
            | GeneralPurposeRegister::RDX
            | GeneralPurposeRegister::R10 => 2,
            GeneralPurposeRegister::BL
            | GeneralPurposeRegister::BX
            | GeneralPurposeRegister::EBX
            | GeneralPurposeRegister::RBX
            | GeneralPurposeRegister::R11 => 3,
            GeneralPurposeRegister::AH
            | GeneralPurposeRegister::SP
            | GeneralPurposeRegister::ESP
            | GeneralPurposeRegister::RSP
            | GeneralPurposeRegister::R12 => 4,
            GeneralPurposeRegister::CH
            | GeneralPurposeRegister::BP
            | GeneralPurposeRegister::EBP
            | GeneralPurposeRegister::RBP
            | GeneralPurposeRegister::R13 => 5,
            GeneralPurposeRegister::DH
            | GeneralPurposeRegister::SI
            | GeneralPurposeRegister::ESI
            | GeneralPurposeRegister::RSI
            | GeneralPurposeRegister::R14 => 6,
            GeneralPurposeRegister::BH
            | GeneralPurposeRegister::DI
            | GeneralPurposeRegister::EDI
            | GeneralPurposeRegister::RDI
            | GeneralPurposeRegister::R15 => 7,
        }
    }

    pub fn size(&self) -> OperandSize {
        match self {
            // 8bit
            GeneralPurposeRegister::AL
            | GeneralPurposeRegister::CL
            | GeneralPurposeRegister::DL
            | GeneralPurposeRegister::BL
            | GeneralPurposeRegister::AH
            | GeneralPurposeRegister::CH
            | GeneralPurposeRegister::DH
            | GeneralPurposeRegister::BH => OperandSize::Byte,

            // 32bit
            GeneralPurposeRegister::EAX
            | GeneralPurposeRegister::ECX
            | GeneralPurposeRegister::EDX
            | GeneralPurposeRegister::EBX
            | GeneralPurposeRegister::ESP
            | GeneralPurposeRegister::EBP
            | GeneralPurposeRegister::ESI
            | GeneralPurposeRegister::EDI => OperandSize::Dword,
            _ => OperandSize::Qword,
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

            // 16bit general-purpose registers
            GeneralPurposeRegister::AX => "ax",
            GeneralPurposeRegister::BX => "bx",
            GeneralPurposeRegister::CX => "cx",
            GeneralPurposeRegister::DX => "dx",
            GeneralPurposeRegister::SP => "sp",
            GeneralPurposeRegister::BP => "bp",
            GeneralPurposeRegister::DI => "di",
            GeneralPurposeRegister::SI => "si",

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

    pub fn new_8bit_from_code(code: usize) -> Self {
        match code {
            0 => GeneralPurposeRegister::AL,
            1 => GeneralPurposeRegister::CL,
            2 => GeneralPurposeRegister::DL,
            3 => GeneralPurposeRegister::BL,
            4 => GeneralPurposeRegister::AH,
            5 => GeneralPurposeRegister::CH,
            6 => GeneralPurposeRegister::DH,
            7 => GeneralPurposeRegister::BH,
            _ => unimplemented!(),
        }
    }
    pub fn new_16bit_from_code(code: usize) -> Self {
        match code {
            0 => GeneralPurposeRegister::AX,
            1 => GeneralPurposeRegister::CX,
            2 => GeneralPurposeRegister::DX,
            3 => GeneralPurposeRegister::BX,
            4 => GeneralPurposeRegister::SP,
            5 => GeneralPurposeRegister::BP,
            6 => GeneralPurposeRegister::SI,
            7 => GeneralPurposeRegister::DI,
            _ => unimplemented!(),
        }
    }
    pub fn new_32bit_from_code(code: usize) -> Self {
        match code {
            0 => GeneralPurposeRegister::EAX,
            1 => GeneralPurposeRegister::ECX,
            2 => GeneralPurposeRegister::EDX,
            3 => GeneralPurposeRegister::EBX,
            4 => GeneralPurposeRegister::ESP,
            5 => GeneralPurposeRegister::EBP,
            6 => GeneralPurposeRegister::ESI,
            7 => GeneralPurposeRegister::EDI,
            _ => unimplemented!(),
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
            8 => GeneralPurposeRegister::R8,
            9 => GeneralPurposeRegister::R9,
            10 => GeneralPurposeRegister::R10,
            11 => GeneralPurposeRegister::R11,
            12 => GeneralPurposeRegister::R12,
            13 => GeneralPurposeRegister::R13,
            14 => GeneralPurposeRegister::R14,
            15 => GeneralPurposeRegister::R15,
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
            "%r10" => GeneralPurposeRegister::R10,
            "%r11" => GeneralPurposeRegister::R11,
            "%r12" => GeneralPurposeRegister::R12,
            "%r13" => GeneralPurposeRegister::R13,
            "%r14" => GeneralPurposeRegister::R14,
            "%r15" => GeneralPurposeRegister::R15,
            _ => panic!("{} is not a register", s),
        }
    }

    pub fn to_8bit(&self) -> Self {
        Self::new_8bit_from_code(self.number() as usize)
    }
    pub fn to_16bit(&self) -> Self {
        Self::new_16bit_from_code(self.number() as usize)
    }
    pub fn to_32bit(&self) -> Self {
        Self::new_32bit_from_code(self.number() as usize)
    }

    pub fn to_64bit(&self) -> Self {
        Self::new_64bit_from_code(self.number() as usize)
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
        write!(f, "{}", self.to_at_string())
    }
}
