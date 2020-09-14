pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() != 2 {
        eprintln!("usage: ./x64_asm <file-path>");
        std::process::exit(1);
    }

    let elf_builder = x64_asm::assemble_file(&args[1], x64_asm::Syntax::ATANDT)?;
    
    elf_builder.generate_elf_file("obj.o")?;

    Ok(())
}