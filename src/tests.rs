#[cfg(test)]
mod assemble_tests{
    use crate::assembler;
    use std::process::Command;

    #[test]
    fn return_42_test() {
        assert_eq!(42, build_and_exec("examples/return_42.s"));
    }

    fn build_and_exec(input_file: &str) -> i32 {
        let elf_builder = assembler::assemble_file(input_file, "obj.o", assembler::Syntax::ATANDT).unwrap();
        elf_builder.generate_elf_file(0o644);

        let _compile_cmd = Command::new("gcc")
        .arg("obj.o")
        .status()
        .expect("failed to spawn a process");

        Command::new("./a.out")
        .status()
        .expect("failed to spawn a process").code().unwrap()
    }
}