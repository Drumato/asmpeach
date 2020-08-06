#[cfg(test)]
mod assemble_tests{
    use crate::assembler;
    use std::process::Command;

    #[test]
    fn return_42_tests() {
        assert_eq!(42, build_and_exec("return_42"));
    }

    #[test]
    fn call_foo_test() {
        assert_eq!(30, build_and_exec("call_foo"));
    }

    fn build_and_exec(file_base: &str) -> i32 {
        let input_file = format!("examples/{}.s", file_base);
        let output_file = format!("/tmp/{}.o", file_base);
        let binary_path = format!("./{}", file_base);
        let elf_builder = assembler::assemble_file(&input_file, &output_file, assembler::Syntax::ATANDT).unwrap();
        elf_builder.generate_elf_file(0o644);

        let _compile_cmd = Command::new("gcc")
        .arg(&output_file)
        .arg("-o")
        .arg(file_base)
        .status()
        .expect("failed to spawn a process");

        let code = Command::new(&binary_path)
        .status()
        .expect("failed to spawn a process").code().unwrap();

        Command::new("rm")
        .arg(file_base)
        .status()
        .expect("failed to spawn a process").code().unwrap();

        code
    }
}