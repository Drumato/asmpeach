pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() != 2 {
        eprintln!("usage: ./x64_asm <file-path>");
        std::process::exit(1);
    }

    let elf_builder = asmpeach::assemble_file(&args[1], asmpeach::Syntax::ATANDT)?;

    elf_builder.generate_elf_file("obj.o", 0o644)?;

    Ok(())
}
