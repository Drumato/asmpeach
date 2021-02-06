use elf_utilities::symbol;

use super::Group;

#[allow(dead_code)]
pub struct Symbol {
    pub groups: Vec<Group>,
    /// Symbol Bind(GLOBAL/LOCAL/etc.)
    pub bind: symbol::Bind,
    /// Symbol Type(NOTYPE/FUNCTION/etc.)
    pub ty: symbol::Type,
    /// machine codes
    pub codes: Vec<u8>,
}

impl Default for Symbol {
    fn default() -> Self {
        Self {
            groups: Vec::new(),
            ty: symbol::Type::NoType,
            bind: symbol::Bind::Local,
            codes: Vec::new(),
        }
    }
}

#[allow(dead_code)]
impl Symbol {
    pub fn as_function(&mut self) {
        self.ty = symbol::Type::Func;
    }

    pub fn as_global(&mut self) {
        self.bind = symbol::Bind::Global;
    }

    pub fn is_function(&self) -> bool {
        self.ty == symbol::Type::Func
    }

    pub fn is_global(&self) -> bool {
        self.bind == symbol::Bind::Global
    }
}
