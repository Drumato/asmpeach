//! An x86_64 assembler.

mod assembler;

pub use assembler::{assemble_code, assemble_file, Syntax};
