mod main;

pub use main::{assemble_code, assemble_file};

mod generator;
mod parser;
mod resource;
mod tests;
pub use resource::{ELFBuilder, Syntax};
