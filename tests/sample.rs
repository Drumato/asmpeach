#[cfg(test)]
mod integration_tests {
    use std::process::Command;
    use x64_asm;

    #[test]
    fn return_42_tests() {
        assert_eq!(42, build_and_exec("return_42"));
    }

    #[test]
    fn call_foo_test() {
        assert_eq!(30, build_and_exec("call_foo"));
    }

    #[test]
    fn double_quote_test() {
        assert_eq!(42, build_and_exec("double_quote"));
    }

    #[test]
    fn store_and_load_test() {
        assert_eq!(8, build_and_exec("store_and_load"));
    }

    #[test]
    fn simple_32bit_inst_test() {
        assert_eq!(8, build_and_exec("simple_32bit_inst"));
    }

    #[test]
    #[ignore]
    fn various_move_test() {
        assert_eq!(0, build_and_exec("various_move"));
    }
    #[test]
    fn small_test() {
        assert_eq!(42, build_and_exec("small"));
    }
    #[test]
    fn small_2_test() {
        assert_eq!(30, build_and_exec("small_2"));
    }
    #[test]
    fn small_3_test() {
        assert_eq!(9, build_and_exec("small_3"));
    }
    #[test]
    fn addq_test() {
        assert_eq!(30, build_and_exec("addq"));
    }
    #[test]
    fn branch1_test() {
        assert_eq!(15, build_and_exec("branch1"));
    }

    fn build_and_exec(file_base: &str) -> i32 {
        let input_file = format!("examples/{}.s", file_base);
        let output_file = format!("/tmp/{}.o", file_base);
        let binary_path = format!("./{}", file_base);
        let elf_builder = x64_asm::assemble_file(&input_file, x64_asm::Syntax::ATANDT).unwrap();
        elf_builder.generate_elf_file(&output_file).unwrap();

        let _compile_cmd = Command::new("gcc")
            .arg(&output_file)
            .arg("-o")
            .arg(file_base)
            .status()
            .expect("failed to spawn a process");

        let code = Command::new(&binary_path)
            .status()
            .expect("failed to spawn a process")
            .code()
            .unwrap();

        Command::new("rm")
            .arg(file_base)
            .status()
            .expect("failed to spawn a process")
            .code()
            .unwrap();

        code
    }
}
