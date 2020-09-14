use crate::assembler::{
    generator, parser,
    resource::{ELFBuilder, Syntax},
};
use std::fs;

type ELFOrError = Result<elf_utilities::file::ELF64Dumper, Box<dyn std::error::Error>>;

/// translate assembly file into object file
pub fn assemble_file(input_file: &str, syntax: Syntax) -> ELFOrError {
    let source = fs::read_to_string(input_file)?;
    assemble(source, syntax)
}

/// translate assembly code into object file.
///
/// # Examples
///
/// ```
/// use x64_asm::*;
///
/// let s = "    .globl main
///     .type main, @function
/// main:
///     pushq %rbp
///     movq %rsp, %rbp
///     movq $42, %rax
///     popq %rbp
///     ret"
///     .to_string();
/// let elf_builder = assemble_code(s, Syntax::ATANDT).unwrap();
/// elf_builder.generate_elf_file("obj.o");
/// ```
pub fn assemble_code(assembly_code: String, syntax: Syntax) -> ELFOrError {
    assemble(assembly_code, syntax)
}

fn assemble(source: String, syntax: Syntax) -> ELFOrError {
    let mut symbols = match syntax {
        Syntax::INTEL => unimplemented!(),
        Syntax::ATANDT => parser::parse_atandt(source),
    };

    // コード生成
    // この時点で再配置シンボルが定義される
    let mut reloc_syms = generator::generate_main(&mut symbols);
    // 再配置テーブルを探索して，シンボルテーブル内に該当するエントリがあれば再配置シンボルを更新する
    generator::setup_relocation(&symbols, &mut reloc_syms);

    let mut builder = ELFBuilder::new();

    // (NULL) セクション
    builder.add_section(elf_utilities::section::Section64::new_null_section());
    // .text セクション
    builder.add_text_section(&symbols);
    // .symtab セクション
    builder.add_symbol_table_section(&symbols);
    // .strtab セクション
    builder.add_symtab_string_section(&symbols);
    // .rela.text セクション
    builder.add_relatext_section(&reloc_syms);
    // .nodata セクション
    builder.add_nodata_section();
    // .rodata セクション
    // object_file_builder.add_rodata_section(&symbols);
    // .shstrtab セクション
    builder.add_shstrtab_string_section();

    // ヘッダの調整
    builder.condition_elf_header();

    Ok(elf_utilities::file::ELF64Dumper::new(
        builder.give_file(),
        0o644,
    ))
}
