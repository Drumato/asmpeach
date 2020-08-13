use crate::assembler::resource::Group;
use elf_utilities::symbol;

#[derive(Eq, Ord, PartialOrd, PartialEq, Debug, Clone)]
pub struct Symbol {
    pub groups: Vec<Group>,
    /// Symbol Visibility(GLOBAL/LOCAL/etc.)
    pub visibility: u8,
    /// Symbol Type(NOTYPE/FUNCTION/etc.)
    pub ty: u8,
    /// machine codes
    pub codes: Vec<u8>,
}

impl Default for Symbol {
    fn default() -> Self {
        Self {
            groups: Vec::new(),
            ty: 0,
            visibility: symbol::STB_LOCAL,
            codes: Vec::new(),
        }
    }
}

impl Symbol {
    pub fn as_function(&mut self) {
        self.ty = symbol::STT_FUNC;
    }

    pub fn as_global(&mut self) {
        self.visibility = symbol::STB_GLOBAL;
    }

    pub fn is_function(&self) -> bool {
        (self.ty & symbol::STT_FUNC) != 0
    }

    pub fn is_global(&self) -> bool {
        (self.visibility & symbol::STB_GLOBAL) != 0
    }
}
