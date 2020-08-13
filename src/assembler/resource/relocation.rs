use elf_utilities::relocation::Rela64;

#[derive(Hash, Eq, Ord, PartialOrd, PartialEq, Debug, Clone)]
pub struct RelaSymbol {
    pub name: String,
    pub rela64: Rela64,
}

impl RelaSymbol {
    pub fn to_le_bytes(&self) -> Vec<u8> {
        self.rela64.to_le_bytes()
    }
}

impl Default for RelaSymbol {
    fn default() -> Self {
        Self {
            name: String::new(),
            rela64: Default::default(),
        }
    }
}
