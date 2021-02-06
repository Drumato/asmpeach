mod main;

pub use main::{assemble_code, assemble_file};

mod generator;
mod parser;
mod resource;
pub use resource::{ELFBuilder, Syntax};
