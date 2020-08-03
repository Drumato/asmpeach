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

    let mut builder = ELFBuilder::new(output_file.to_string());

    // (NULL) セクション
    builder.add_section(elf_utilities::section::Section64::new_null_section());
    // .text セクション
    builder.add_text_section(&symbols);
    // .symtab セクション
    builder.add_symbol_table_section(&symbols);
    // .strtab セクション
    builder.add_symtab_string_section(&symbols);
    // .rela.text セクション
    // object_file_builder.add_relatext_section(&generator);
    // .nodata セクション
    builder.add_nodata_section();
    // .rodata セクション
    // object_file_builder.add_rodata_section(&symbols);
    // .shstrtab セクション
    builder.add_shstrtab_string_section();

    // ヘッダの調整
    builder.condition_elf_header();

    Ok(builder)
}
