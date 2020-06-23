use crate::*;

/// grouping instructions with label
///
/// # Examples
///
/// ```rust
///
/// ```
#[derive(Eq, Ord, PartialOrd, PartialEq, Debug, Clone)]
pub struct Group {
    pub label: String,
    pub insts: Vec<Instruction>,
}

impl Default for Group {
    fn default() -> Self {
        Self {
            label: String::new(),
            insts: Vec::new(),
        }
    }
}

impl Group {
    pub fn to_intel_string(&self) -> String {
        let mut s = format!("{}:\n", self.label);

        for i in self.insts.iter() {
            s += format!("  {}\n", i.to_intel_string()).as_str();
        }

        s
    }

    pub fn to_at_string(&self) -> String {
        let mut s = format!("{}:\n", self.label);

        for i in self.insts.iter() {
            s += format!("  {}\n", i.to_at_string()).as_str();
        }

        s
    }
}