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
            ".section" | ".text" | ".size" | ".ident" | ".align" | ".long" | ".string" => {}
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
        let inst: Box<dyn Instruction> = match opcode {
            "retq" | "ret" => Box::new(Ret::Near),
            "endbr64" => Box::new(EndBr64()),
            "syscall" => Box::new(SysCall()),
            _ => panic!("not implemented generating '{}' yet", opcode),
        };

        self.push_inst_cur_sym(sym_name, inst);
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
        let inst: Box<dyn Instruction> = match opcode {
            "pushq" => Box::new(Push::new(OperandSize::Qword, operand)),
            "popq" => Box::new(Pop::new(OperandSize::Qword, operand)),
            "call" => Box::new(Call::new(operand)),
            "jle" => Box::new(Jmp::LessThanEqual(operand)),
            "je" => Box::new(Jmp::Equal(operand)),
            "jmp" => Box::new(Jmp::Unconditional(operand)),
            _ => panic!("not implemented generating '{}' yet", opcode),
        };

        self.push_inst_cur_sym(sym_name, inst);
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

        let inst: Box<dyn Instruction> = match opcode {
            "addl" => Box::new(Add::new(OperandSize::Dword, src_op, dst_op)),
            "addq" => Box::new(Add::new(OperandSize::Qword, src_op, dst_op)),
            "cmpq" => Box::new(Cmp::new(OperandSize::Qword, src_op, dst_op)),
            "subq" => Box::new(Sub::new(OperandSize::Qword, src_op, dst_op)),
            "leaq" => Box::new(Lea::new(OperandSize::Qword, src_op, dst_op)),
            "imulq" => Box::new(IMul::new(OperandSize::Qword, src_op, dst_op)),
            "movb" => Box::new(Mov::new(OperandSize::Byte, src_op, dst_op)),
            "movw" => Box::new(Mov::new(OperandSize::Word, src_op, dst_op)),
            "movl" => Box::new(Mov::new(OperandSize::Dword, src_op, dst_op)),
            "movq" => Box::new(Mov::new(OperandSize::Qword, src_op, dst_op)),
            _ => panic!("not implemented generating '{}' yet", opcode),
        };

        self.push_inst_cur_sym(sym_name, inst);
        assert!(iter.next().is_none());
    }

    fn remove_double_quote(op: &str) -> String {
        op.trim_start_matches('"').trim_end_matches('"').to_string()
    }

    fn parse_operand(operand: &str) -> Operand {
        let stripped = Self::remove_pat_and_newline(operand, ",");

        // レジスタの場合
        if stripped.starts_with('%') {
            return Operand::GeneralReg(GeneralPurposeRegister::from_at_string(&stripped));
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
            return Operand::Label(Self::remove_double_quote(&stripped));
        }

        // メモリオペランド
        let mut splitted = stripped.split('(');
        let disp_str = splitted.next();

        let displacement = match disp_str.unwrap() {
            // 単純なでリファレンス
            "" => None,
            disp => match disp.parse::<i8>() {
                Ok(v) => Some(Displacement::Disp8(v)),
                Err(_e) => match stripped.parse::<i32>() {
                    Ok(v) => Some(Displacement::Disp32(v)),
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

        Operand::Memory(OpMemory {
            index: index_reg,
            base: base_reg,
            disp: displacement,
            scale,
        })
    }

    fn is_directive_start(&self, directive: &str) -> bool {
        match directive {
            ".globl" | ".global" | ".type" | ".section" | ".text" | ".size" | ".align"
            | ".long" | ".string" | "" => true,
            _ => false,
        }
    }

    fn push_inst_cur_sym(&mut self, sym_name: &str, inst: Box<dyn Instruction>) {
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
