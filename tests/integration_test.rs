mod common;

#[cfg(test)]
mod c_integration_tests {
    use super::common::c_program_test;

    #[test]
    fn return_42_test() {
        assert_eq!(42, c_program_test("return_42"));
    }
    #[test]
    fn call_foo_test() {
        assert_eq!(42, c_program_test("call_foo"));
    }
    #[test]
    fn if1_test() {
        assert_eq!(1, c_program_test("if1"));
    }
    #[test]
    fn if2_test() {
        assert_eq!(0, c_program_test("if2"));
    }
    #[test]
    fn declare_autovar1_test() {
        assert_eq!(42, c_program_test("declare_autovar1"));
    }
    #[test]
    fn while1_test() {
        assert_eq!(10, c_program_test("while1"));
    }
}

#[cfg(test)]
mod asm_integration_tests {
    use super::common::assembly_file_test;

    #[test]
    fn double_quote_test() {
        assert_eq!(42, assembly_file_test("double_quote"));
    }
}
