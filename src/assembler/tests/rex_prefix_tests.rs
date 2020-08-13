#[cfg(test)]
mod format_tests {
    use crate::assembler::resource::*;

    #[test]
    fn display_rex_prefix_test() {
        let prefix = REXPrefix {
            w_bit: true,
            r_bit: false,
            x_bit: true,
            b_bit: false,
        };

        assert_eq!("REX(0b1001010)", format!("{}", prefix).as_str());
    }

    #[test]
    fn debug_rex_prefix_test() {
        let prefix = REXPrefix {
            w_bit: true,
            r_bit: false,
            x_bit: true,
            b_bit: false,
        };

        assert_eq!("0100W-X-", format!("{:?}", prefix).as_str());
    }
}

#[cfg(test)]
mod to_byte_tests {
    use crate::assembler::resource::*;

    #[test]
    fn to_byte_test() {
        let prefix = REXPrefix {
            w_bit: true,
            r_bit: false,
            x_bit: true,
            b_bit: false,
        };

        assert_eq!(
            REXPrefix::BASE | REXPrefix::W_BIT | REXPrefix::X_BIT,
            prefix.to_byte()
        );
    }
}
