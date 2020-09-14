use crate::assembler::resource::*;
use indexmap::map::IndexMap;
use std::str::SplitAsciiWhitespace;

struct Context {
    state: State,
    syms: IndexMap<String, Symbol>,
}

#[derive(Eq, Ord, PartialOrd, PartialEq, Debug, Clone)]
enum State {
    TopLevel,
    InSymbol(String),
}

/// parse AT&T syntax assembly.
pub fn parse_atandt(source: String) -> IndexMap<String, Symbol> {
    let lines_iter = source.lines();
    let mut context = Context {
        state: State::TopLevel,
        syms: Default::default(),
    };

    // 各行に対して処理を行う
    for l in lines_iter {
        match context.state.clone() {
            State::TopLevel => context.toplevel(l),
            State::InSymbol(sym_name) => context.in_symbol(l, &sym_name),
        }
    }

    context.syms
}

impl Context {
    fn toplevel(&mut self, line: &str) {
        // 空行だったら無視
        if Self::is_blank_line(line) {
            return;
        }

        // シンボル名の場合
        if line.trim_end().ends_with(':') {
            let sym_name = Self::remove_double_quote(&Self::remove_pat_and_newline(line, ":"));
            self.state = State::InSymbol(sym_name.clone());
            self.syms.entry(sym_name).or_insert_with(Symbol::default);
            return;
        }

        let mut iterator = line.split_ascii_whitespace();

        self.parse_directive(&mut iterator);
    }

    fn parse_directive(&mut self, iterator: &mut SplitAsciiWhitespace) {
        let directive = iterator.next().unwrap();

        match directive {
            ".global" | ".globl" => self.parse_global_directive(iterator),
            ".type" => self.parse_symbol_type_directive(iterator),
            ".section" => {}
            ".text" => {}
            _ => {}
        }
    }

    /// `.global main` みたいなやつ
    fn parse_global_directive(&mut self, iterator: &mut SplitAsciiWhitespace) {
        let sym_name = Self::remove_double_quote(iterator.next().unwrap());

        self.syms
            .entry(sym_name)
            .or_insert_with(Symbol::default)
            .as_global();
        assert!(iterator.next().is_none());
    }

    /// `.type main, @function` みたいなやつ
    fn parse_symbol_type_directive(&mut self, iterator: &mut SplitAsciiWhitespace) {
        let sym_name =
            Self::remove_double_quote(&Self::remove_pat_and_newline(iterator.next().unwrap(), ","));
        let sym_type = iterator.next().unwrap();
        assert_eq!(sym_type, "@function");

        self.syms
            .entry(sym_name)
            .or_insert_with(Symbol::default)
            .as_function();
    }

    // シンボル名をパース後
    fn in_symbol(&mut self, line: &str, sym_name: &str) {
        let line = line.trim_start().trim_end();
        // シンボル名の場合
        if line.ends_with(':') {
            // ラベルかどうかチェック
            if line.starts_with(".L") || line.starts_with("\".L") {
                self.push_group(sym_name, &Self::remove_pat_and_newline(&line, ":"));
            } else {
                // ラベルではない => 別のシンボル定義と解釈
                let another_sym = Self::remove_pat_and_newline(&line, ":");
                self.state = State::InSymbol(another_sym.clone());
                self.syms.entry(another_sym).or_insert_with(Symbol::default);
            }

            return;
        }

        if Self::is_blank_line(&line) {
            return;
        }

        let mut iterator = line.split_ascii_whitespace();

        let opcode = iterator.next().unwrap();

        // .global等のディレクティブを見つけたら
        if self.is_directive_start(opcode) {
            self.state = State::TopLevel;
            self.toplevel(&line);
            return;
        }

        // オペランドの数を調べる．
        let count = iterator.clone().count();

        match count {
            0 => self.parse_no_operand_instruction(&mut iterator, sym_name, opcode),
            1 => self.parse_unary_instruction(&mut iterator, sym_name, opcode),
            2 => self.parse_binary_instruction(&mut iterator, sym_name, opcode),
            _ => panic!("unsupported instruction -> {}", line),
        }
    }

    fn parse_no_operand_instruction(
        &mut self,
        iter: &mut SplitAsciiWhitespace,
        sym_name: &str,
        opcode: &str,
    ) {
        let opcode = match opcode {
            "ret" => Opcode::RET,
            "endbr64" => Opcode::ENDBR64,
            "syscall" => Opcode::SYSCALL,
            _ => panic!("not implemented generating '{}' yet", opcode),
        };

        self.push_inst_cur_sym(sym_name, Instruction { opcode });
        assert!(iter.next().is_none());
    }

    fn parse_unary_instruction(
        &mut self,
        iter: &mut SplitAsciiWhitespace,
        sym_name: &str,
        opcode: &str,
    ) {
        let operand = iter.next();
        assert!(operand.is_some());

        let operand = Self::parse_operand(operand.unwrap());
        let opcode = match opcode {
            "pushq" => Opcode::push(OperandSize::QWORD, operand),
            "popq" => Opcode::pop(OperandSize::QWORD, operand),
            "call" => Opcode::call(operand),
            _ => panic!("not implemented generating '{}' yet", opcode),
        };

        self.push_inst_cur_sym(sym_name, Instruction { opcode });
        assert!(iter.next().is_none());
    }

    fn parse_binary_instruction(
        &mut self,
        iter: &mut SplitAsciiWhitespace,
        sym_name: &str,
        opcode: &str,
    ) {
        let src = iter.next();
        assert!(src.is_some());
        let src_op = Self::parse_operand(src.unwrap());

        let dst = iter.next();
        assert!(dst.is_some());
        let dst_op = Self::parse_operand(dst.unwrap());

        let opcode = match opcode {
            "addl" => Opcode::add(OperandSize::DWORD, src_op.to_32bit(), dst_op.to_32bit()),
            "addq" => Opcode::add(OperandSize::QWORD, src_op.to_64bit(), dst_op.to_64bit()),
            "subq" => Opcode::sub(OperandSize::QWORD, src_op.to_64bit(), dst_op.to_64bit()),
            "leaq" => Opcode::lea(OperandSize::QWORD, src_op.to_64bit(), dst_op.to_64bit()),
            "imulq" => Opcode::imul(OperandSize::QWORD, src_op.to_64bit(), dst_op.to_64bit()),
            "movb" => Opcode::mov(OperandSize::BYTE, src_op.to_8bit(), dst_op.to_8bit()),
            "movw" => Opcode::mov(OperandSize::WORD, src_op.to_16bit(), dst_op.to_16bit()),
            "movl" => Opcode::mov(OperandSize::DWORD, src_op.to_32bit(), dst_op.to_32bit()),
            "movq" => Opcode::mov(OperandSize::QWORD, src_op.to_64bit(), dst_op.to_64bit()),
            _ => panic!("not implemented generating '{}' yet", opcode),
        };

        self.push_inst_cur_sym(sym_name, Instruction { opcode });
        assert!(iter.next().is_none());
    }

    fn remove_double_quote(op: &str) -> String {
        op.trim_start_matches('"').trim_end_matches('"').to_string()
    }

    fn parse_operand(operand: &str) -> Operand {
        let stripped = Self::remove_pat_and_newline(operand, ",");

        // レジスタの場合
        if stripped.starts_with('%') {
            return Operand::GENERALREGISTER(GeneralPurposeRegister::from_at_string(&stripped));
        }

        // 即値の場合
        let immediate = stripped.trim_start_matches('$');
        match immediate.parse::<i8>() {
            Ok(v) => {
                return Operand::Immediate(Immediate::I8(v));
            }
            Err(_e) => match immediate.parse::<i32>() {
                Ok(v) => {
                    return Operand::Immediate(Immediate::I32(v));
                }
                // 即値オペランドでなかった場合
                Err(_e) => {}
            },
        }

        // '(' がない => label
        if !stripped.contains("(") {
            return Operand::LABEL(Self::remove_double_quote(&stripped));
        }

        // メモリオペランド
        let mut splitted = stripped.split('(');
        let disp_str = splitted.next();

        let displacement = match disp_str.unwrap() {
            // 単純なでリファレンス
            "" => None,
            disp => match disp.parse::<i8>() {
                Ok(v) => Some(Displacement::DISP8(v)),
                Err(_e) => match stripped.parse::<i32>() {
                    Ok(v) => Some(Displacement::DISP32(v)),
                    // offset無し
                    Err(_e) => None,
                },
            },
        };

        let base_reg = splitted.next().unwrap();
        let mut memory_operand_str = base_reg.trim_end_matches(')').split(',');
        let base_reg = GeneralPurposeRegister::from_at_string(memory_operand_str.next().unwrap());

        let index_reg = match memory_operand_str.next() {
            Some(ireg_str) => Some(GeneralPurposeRegister::from_at_string(
                ireg_str.trim_start(),
            )),
            None => None,
        };
        let scale = match memory_operand_str.next() {
            Some(scale_str) => Some(scale_str.trim_start().parse::<u8>().unwrap()),
            None => None,
        };

        Operand::ADDRESSING {
            index: index_reg,
            base: base_reg,
            disp: displacement,
            scale,
        }
    }

    fn is_directive_start(&self, directive: &str) -> bool {
        match directive {
            ".globl" | ".global" | ".type" | ".section" | ".text" => true,
            _ => false,
        }
    }

    fn push_inst_cur_sym(&mut self, sym_name: &str, inst: Instruction) {
        if let Some(sym) = self.syms.get_mut(sym_name) {
            if sym.groups.is_empty() {
                sym.groups
                    .push(Group::new(&format!(".L{}_entry", sym_name)));
            }

            let group_idx = sym.groups.len() - 1;
            sym.groups[group_idx].insts.push(inst);

            return;
        }

        unreachable!();
    }

    fn push_group(&mut self, sym_name: &str, label_name: &str) {
        self.syms
            .get_mut(sym_name)
            .unwrap()
            .groups
            .push(Group::new(&Self::remove_double_quote(label_name)));
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

        ctxt.toplevel("\"aarch64::main\":    \n");

        assert!(!ctxt.syms.is_empty());
        assert_eq!(State::InSymbol("aarch64::main".to_string()), ctxt.state);
        assert!(ctxt.syms.get("aarch64::main").is_some());
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
            ctxt.syms.get("main").unwrap().groups[0].insts[0].opcode
        );
    }

    #[test]
    fn parse_popq_test() {
        let mut ctxt = new_context();
        ctxt.toplevel("main:    \n");
        ctxt.in_symbol("    popq %rax", "main");
        assert_eq!(
            Opcode::POPR64 {
                r64: GeneralPurposeRegister::RAX
            },
            ctxt.syms.get("main").unwrap().groups[0].insts[0].opcode
        );

        ctxt.in_symbol("    popq     %rbp", "main");
        assert_eq!(
            Opcode::POPR64 {
                r64: GeneralPurposeRegister::RBP
            },
            ctxt.syms.get("main").unwrap().groups[0].insts[1].opcode
        );
    }

    #[test]
    fn parse_moveq_test() {
        let mut ctxt = new_context();
        ctxt.toplevel("main:    \n");
        ctxt.in_symbol("movq $42, %rax", "main");
        assert_eq!(
            Opcode::MOVRM64IMM32 {
                imm: Immediate::I32(42),
                rm64: Operand::GENERALREGISTER(GeneralPurposeRegister::RAX),
            },
            ctxt.syms.get("main").unwrap().groups[0].insts[0].opcode
        );

        ctxt.in_symbol("movq $3, -24(%rbp)", "main");
        assert_eq!(
            Opcode::MOVRM64IMM32 {
                imm: Immediate::I32(3),
                rm64: Operand::ADDRESSING {
                    base: GeneralPurposeRegister::RBP,
                    index: None,
                    disp: Some(Displacement::DISP8(-24)),
                    scale: None,
                },
            },
            ctxt.syms.get("main").unwrap().groups[0].insts[1].opcode
        );
    }

    #[test]
    fn parse_ret_test() {
        let mut ctxt = new_context();
        ctxt.toplevel("main:    \n");

        ctxt.in_symbol("  ret\n", "main");
        assert_eq!(State::InSymbol("main".to_string()), ctxt.state);
        assert_eq!(1, ctxt.syms.get("main").unwrap().groups[0].insts.len());
        assert_eq!(
            Opcode::RET,
            ctxt.syms.get("main").unwrap().groups[0].insts[0].opcode
        );
    }

    #[test]
    fn parse_operand_test() {
        assert_eq!(
            Operand::GENERALREGISTER(GeneralPurposeRegister::RAX),
            Context::parse_operand("%rax")
        );
        assert_eq!(
            Operand::GENERALREGISTER(GeneralPurposeRegister::RAX),
            Context::parse_operand("%rax,")
        );
        assert_eq!(
            Operand::Immediate(Immediate::I8(30)),
            Context::parse_operand("$30")
        );
        assert_eq!(
            Operand::ADDRESSING {
                base: GeneralPurposeRegister::RAX,
                index: None,
                disp: None,
                scale: None,
            },
            Context::parse_operand("(%rax)"),
        );
        assert_eq!(
            Operand::ADDRESSING {
                base: GeneralPurposeRegister::RAX,
                index: None,
                disp: Some(Displacement::DISP8(-8)),
                scale: None,
            },
            Context::parse_operand("-8(%rax)"),
        );
        assert_eq!(
            Operand::ADDRESSING {
                base: GeneralPurposeRegister::RAX,
                index: Some(GeneralPurposeRegister::RBX),
                disp: Some(Displacement::DISP8(-8)),
                scale: None,
            },
            Context::parse_operand("-8(%rax, %rbx)"),
        );
        assert_eq!(
            Operand::ADDRESSING {
                base: GeneralPurposeRegister::RAX,
                index: Some(GeneralPurposeRegister::RBX),
                disp: Some(Displacement::DISP8(16)),
                scale: Some(4),
            },
            Context::parse_operand("16(%rax, %rbx, 4)"),
        );
    }

    #[test]
    fn is_blank_line_test() {
        assert!(Context::is_blank_line("\n"));
        assert!(Context::is_blank_line("        \n"));
        assert!(Context::is_blank_line("\t\t\t\t\n"));
    }

    #[test]
    fn remove_double_quote_test() {
        assert_eq!(
            "main::entry".to_string(),
            Context::remove_double_quote("\"main::entry\"")
        );
    }

    fn new_context() -> Context {
        Context {
            state: State::TopLevel,
            syms: IndexMap::new(),
        }
    }
}
