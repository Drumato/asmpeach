use crate::assembler::{generator, parser};
use crate::resources::{ELFBuilder, Syntax};
use std::fs;

/// translate assembly file into object file
pub fn assemble_file(
    input_file: &str,
    output_file: &str,
    syntax: Syntax,
) -> Result<ELFBuilder, Box<dyn std::error::Error>> {
    let source = fs::read_to_string(input_file)?;
    let mut symbols = match syntax {
        Syntax::INTEL => unimplemented!(),
        Syntax::ATANDT => parser::parse_atandt(source),
    };
    generator::generate_main(&mut symbols);

    let builder = ELFBuilder::new(output_file.to_string());

    Ok(builder)
}
