use std::collections::BTreeMap;

use crate::resources::*;
use std::str::SplitAsciiWhitespace;

struct Context {
    state: State,
    syms: BTreeMap<String, Symbol>,
}

#[derive(Eq, Ord, PartialOrd, PartialEq, Debug, Clone)]
enum State {
    TopLevel,
    InSymbol(String),
}

pub fn parse_atandt(source: String) {
    let lines_iter = source.lines();
    let mut context = Context {
        state: State::TopLevel,
        syms: Default::default(),
    };

    for l in lines_iter {
        match context.state.clone() {
            State::TopLevel => context.toplevel(l),
            State::InSymbol(sym_name) => context.in_symbol(l, &sym_name),
        }
    }
}

impl Context {
    fn toplevel(&mut self, line: &str) {
        // 空行だったら無視
        if Self::is_blank_line(line) {
            return;
        }

        // シンボル名の場合
        if line.trim_end().ends_with(':') {
            let sym_name = Self::remove_pat_and_newline(line, ":");
            self.state = State::InSymbol(sym_name.clone());
            self.syms.entry(sym_name).or_insert_with(Symbol::default);
            return;
        }

        let mut iterator = line.split_ascii_whitespace();
        let directive = iterator.next().unwrap();

        match directive {
            ".global" | ".globl" => {
                let sym_name = iterator.next().unwrap().to_string();

                self.syms
                    .entry(sym_name)
                    .or_insert_with(Symbol::default)
                    .as_global();
                assert!(iterator.next().is_none());
                return;
            }
            ".type" => {
                let sym_name = Self::remove_pat_and_newline(iterator.next().unwrap(), ",");
                let sym_type = iterator.next().unwrap();
                assert_eq!(sym_type, "@function");

                self.syms
                    .entry(sym_name)
                    .or_insert_with(Symbol::default)
                    .as_function();
                return;
            }
            _ => {}
        }

        unreachable!()
    }

    // シンボル名をパース後
    fn in_symbol(&mut self, line: &str, sym_name: &str) {
        if Self::is_blank_line(line) {
            return;
        }

        let mut iterator = line.split_ascii_whitespace();

        let opcode = iterator.next().unwrap();

        // シンボル内のラベル
        if opcode.starts_with(".L") {
            self.push_group(sym_name, &Self::remove_pat_and_newline(opcode, ":"));
            return;
        }

        // 各命令ごとにパース
        match opcode {
            "pushq" => self.parse_pushq_inst(&mut iterator, sym_name),
            "ret" => self.push_inst_cur_sym(
                sym_name,
                Instruction {
                    opcode: Opcode::RET,
                },
            ),
            _ => panic!("unsupported instruction -> {}", line),
        }
    }

    fn parse_pushq_inst(&mut self, iter: &mut SplitAsciiWhitespace, sym_name: &str) {
        let operand = iter.next();
        assert!(operand.is_some());

        let operand = Self::parse_operand(operand.unwrap());
        let opcode = match operand {
            Operand::Immediate(imm) => Opcode::PUSHIMM32 { imm },
            Operand::GENERALREGISTER(reg) => Opcode::PUSHR64 { r64: reg },
            _ => unreachable!(),
        };
        self.push_inst_cur_sym(sym_name, Instruction { opcode });

        assert!(iter.next().is_none());
    }

    fn parse_operand(operand: &str) -> Operand {
        // レジスタの場合
        if operand.starts_with('%') {
            return Operand::GENERALREGISTER(GeneralPurposeRegister::from_at_string(operand));
        }

        // 即値の場合
        let immediate = operand.trim_start_matches('$');
        match immediate.parse::<i8>() {
            Ok(v) => {
                return Operand::Immediate(Immediate::I8(v));
            }
            Err(_e) => match immediate.parse::<i32>() {
                Ok(v) => {
                    return Operand::Immediate(Immediate::I32(v));
                }
                Err(_e) => {}
            },
        }

        Operand::LABEL(operand.to_string())
    }

    fn push_inst_cur_sym(&mut self, sym_name: &str, inst: Instruction) {
        if let Some(sym) = self.syms.get_mut(sym_name) {
            if sym.labels.is_empty() {
                sym.labels
                    .push(Group::new(&format!(".L{}_entry", sym_name)));
            }

            let group_idx = sym.labels.len() - 1;
            sym.labels[group_idx].insts.push(inst);

            return;
        }

        unreachable!();
    }

    fn push_group(&mut self, sym_name: &str, label_name: &str) {
        self.syms
            .get_mut(sym_name)
            .unwrap()
            .labels
            .push(Group::new(label_name));
    }

    fn remove_pat_and_newline(line: &str, pat: &str) -> String {
        line.trim_end().trim_end_matches(pat).to_string()
    }

    fn is_blank_line(line: &str) -> bool {
        line.trim_end().is_empty()
    }
}

#[cfg(test)]
mod parse_tests {
    use super::*;

    #[test]
    fn parse_symname_test() {
        let mut ctxt = new_context();
        ctxt.toplevel("main:    \n");

        assert!(!ctxt.syms.is_empty());
        assert_eq!(State::InSymbol("main".to_string()), ctxt.state);
        assert!(ctxt.syms.get("main").is_some());
    }

    #[test]
    fn parse_global_directive_test() {
        let mut ctxt = new_context();
        ctxt.toplevel("    .global main    \n");

        assert!(!ctxt.syms.is_empty());
        assert_eq!(State::TopLevel, ctxt.state);
        assert!(ctxt.syms.get("main").unwrap().is_global());
    }

    #[test]
    fn parse_type_directive_test() {
        let mut ctxt = new_context();
        ctxt.toplevel("    .type    main, @function    \n");

        assert!(!ctxt.syms.is_empty());
        assert_eq!(State::TopLevel, ctxt.state);
        assert!(ctxt.syms.get("main").unwrap().is_function());
    }

    #[test]
    fn parse_pushq_test() {
        let mut ctxt = new_context();
        ctxt.toplevel("main:    \n");
        ctxt.in_symbol("pushq %rax", "main");
        assert_eq!(
            Opcode::PUSHR64 {
                r64: GeneralPurposeRegister::RAX
            },
            ctxt.syms.get("main").unwrap().labels[0].insts[0].opcode
        );
    }

    #[test]
    fn parse_ret_test() {
        let mut ctxt = new_context();
        ctxt.toplevel("main:    \n");

        ctxt.in_symbol("  ret\n", "main");
        assert_eq!(State::InSymbol("main".to_string()), ctxt.state);
        assert_eq!(1, ctxt.syms.get("main").unwrap().labels[0].insts.len());
        assert_eq!(
            Opcode::RET,
            ctxt.syms.get("main").unwrap().labels[0].insts[0].opcode
        );
    }

    #[test]
    fn parse_operand_test() {
        assert_eq!(
            Operand::GENERALREGISTER(GeneralPurposeRegister::RAX),
            Context::parse_operand("%rax")
        );
        assert_eq!(
            Operand::Immediate(Immediate::I8(30)),
            Context::parse_operand("$30")
        );
    }

    #[test]
    fn is_blank_line_test() {
        assert!(Context::is_blank_line("\n"));
        assert!(Context::is_blank_line("        \n"));
        assert!(Context::is_blank_line("\t\t\t\t\n"));
    }

    fn new_context() -> Context {
        Context {
            state: State::TopLevel,
            syms: BTreeMap::new(),
        }
    }
}
