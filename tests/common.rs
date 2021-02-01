use std::process::Command;

pub fn assembly_file_test(file_base: &str) -> i32 {
    let target_file = format!("tests/asm/{}.s", file_base);
    let obj_file = format!("/tmp/{}.o", file_base);
    let executable_path = format!("/tmp/{}", file_base);

    let elf_builder = asmpeach::assemble_file(&target_file, asmpeach::Syntax::ATANDT).unwrap();
    elf_builder.generate_elf_file(&obj_file, 0o644).unwrap();

    let _compile_cmd = Command::new("gcc")
        .arg(&obj_file)
        .arg("-o")
        .arg(&executable_path)
        .status()
        .expect("failed to spawn a process");

    let code = Command::new(&executable_path)
        .status()
        .expect("failed to spawn a process")
        .code()
        .unwrap();

    code
}

pub fn c_program_test(file_base: &str) -> i32 {
    let target_file = format!("tests/c/{}.c", file_base);
    let asm_file = format!("/tmp/{}.s", file_base);
    let obj_file = format!("/tmp/{}.o", file_base);
    let executable_path = format!("/tmp/{}", file_base);

    let _compile_cmd = Command::new("gcc")
        .arg("-S")
        .arg(&target_file)
        .arg("-o")
        .arg(&asm_file)
        .arg("-fno-asynchronous-unwind-tables")
        .status()
        .expect("failed to spawn a process");

    let elf_builder = asmpeach::assemble_file(&asm_file, asmpeach::Syntax::ATANDT).unwrap();
    elf_builder.generate_elf_file(&obj_file, 0o644).unwrap();

    let _compile_cmd = Command::new("gcc")
        .arg(&obj_file)
        .arg("-o")
        .arg(&executable_path)
        .status()
        .expect("failed to spawn a process");

    let code = Command::new(&executable_path)
        .status()
        .expect("failed to spawn a process")
        .code()
        .unwrap();

    code
}
