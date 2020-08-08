use crate::assembler::resource::Instruction;

#[derive(Eq, Ord, PartialOrd, PartialEq, Debug, Clone)]
pub struct Group {
    pub label: String,
    pub insts: Vec<Instruction>,
}

impl Group {
    pub fn new(label: &str) -> Self {
        assert!(label.starts_with(".L"), "label name must start with '.L'");

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