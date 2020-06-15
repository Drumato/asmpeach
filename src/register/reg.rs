pub trait Registerable {
    type RegisterNumber;

    fn number(&self) -> Self::RegisterNumber;

    /// for AT&T syntax.
    fn to_at(&self) -> String;

    /// for intel syntax.
    fn to_intel(&self) -> String;

    /// check whether x64 appended register
    fn is_expanded(&self) -> bool;
}

/// Registerableを実装する標準API
pub enum DefaultRegister {
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

impl Registerable for DefaultRegister {
    type RegisterNumber = usize;

    fn number(&self) -> Self::RegisterNumber {
        match self {
            Self::RAX | Self::R8 => 0,
            Self::RCX | Self::R9 => 1,
            Self::RDX | Self::R10 => 2,
            Self::RBX | Self::R11 => 3,
            Self::RSP | Self::R12 => 4,
            Self::RBP | Self::R13 => 5,
            Self::RSI | Self::R14 => 6,
            Self::RDI | Self::R15 => 7,
        }
    }

    fn to_at(&self) -> String {
        format!("%{}", self.to_str())
    }

    fn to_intel(&self) -> String {
        self.to_str().to_string()
    }

    /// 拡張されたレジスタかどうかのチェック
    /// REX prefixに用いる
    fn is_expanded(&self) -> bool {
        match self {
            Self::R8 | Self::R9 | Self::R10 | Self::R11 | Self::R12 | Self::R13 | Self::R14 | Self::R15 => true,
            _ => false,
        }
    }
}

impl DefaultRegister {
    fn to_str(&self) -> &'static str {
        match self {
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