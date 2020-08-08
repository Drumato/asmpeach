mod main;

pub use main::{assemble_file, assemble_code};

mod parser;
mod generator;
mod resource;
mod tests;
pub use resource::{ELFBuilder, Syntax};
