use crate::assembler::resource::Group;
use elf_utilities::symbol;

#[allow(dead_code)]
#[derive(Eq, Ord, PartialOrd, PartialEq, Debug)]
pub struct Symbol {
    pub groups: Vec<Group>,
    /// Symbol Bind(GLOBAL/LOCAL/etc.)
    pub bind: symbol::BIND,
    /// Symbol Type(NOTYPE/FUNCTION/etc.)
    pub ty: symbol::TYPE,
    /// machine codes
    pub codes: Vec<u8>,
}

impl Default for Symbol {
    fn default() -> Self {
        Self {
            groups: Vec::new(),
            ty: symbol::TYPE::NOTYPE,
            bind: symbol::BIND::LOCAL,
            codes: Vec::new(),
        }
    }
}

#[allow(dead_code)]
impl Symbol {
    pub fn as_function(&mut self) {
        self.ty = symbol::TYPE::FUNC;
    }

    pub fn as_global(&mut self) {
        self.bind = symbol::BIND::GLOBAL;
    }

    pub fn is_function(&self) -> bool {
        self.ty == symbol::TYPE::FUNC
    }

    pub fn is_global(&self) -> bool {
        self.bind == symbol::BIND::GLOBAL
    }
}
