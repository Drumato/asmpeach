mod main;

pub use main::{assemble_file, assemble_code};

mod parser;
mod generator;
mod resources;
mod tests;
pub use resources::{ELFBuilder, Syntax};
