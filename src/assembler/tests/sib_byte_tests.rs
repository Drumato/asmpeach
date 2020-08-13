#[cfg(test)]
mod format_tests {
    use crate::assembler::resource::*;

    #[test]
    fn display_sib_byte_test() {
        let byte = SIBByte {
            base_reg: 0,
            index_reg: 2,
            scale: 4,
        };

        assert_eq!("SIB(0b10010000)", format!("{}", byte).as_str())
    }

    #[test]
    fn debug_sib_byte_test() {
        let byte = SIBByte {
            base_reg: 0,
            index_reg: 2,
            scale: 4,
        };

        assert_eq!(
            "SIB(base 0b0: index 0b10: 4x scale)",
            format!("{:?}", byte).as_str()
        )
    }
}
