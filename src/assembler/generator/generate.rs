use crate::assembler::resource::{InstName, RelaSymbol, Symbol};
use indexmap::map::IndexMap;

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Copy, Clone)]
struct RelativeJumpSpec {
    operand_offset: isize,
    address: isize,
    is_label: bool,
}

impl RelativeJumpSpec {
    fn new_label(addr: isize) -> Self {
        Self {
            operand_offset: 0,
            address: addr,
            is_label: true,
        }
    }
    fn new_jump(offset: isize) -> Self {
        Self {
            operand_offset: offset,
            address: offset,
            is_label: false,
        }
    }
}

pub fn generate_main(symbols: &mut IndexMap<String, Symbol>) -> IndexMap<String, Vec<RelaSymbol>> {
    let mut reloc_syms = IndexMap::new();

    for (sym_name, sym) in symbols.iter_mut() {
        let (mut sym_codes, relocs_in_sym) = gen_symbol_code(sym);
        reloc_syms.insert(sym_name.to_string(), relocs_in_sym);

        // アラインメント調整
        let mut extra_bytes: Vec<u8> = Vec::new();

        let rest_bytes = sym_codes.len() % 4;
        for _ in 0..(4 - rest_bytes) {
            extra_bytes.push(0x00);
        }
        sym_codes.append(&mut extra_bytes);

        sym.codes = sym_codes;
    }

    reloc_syms
}

fn gen_symbol_code(sym: &Symbol) -> (Vec<u8>, Vec<RelaSymbol>) {
    let mut relative_jump_offset: IndexMap<String, Vec<RelativeJumpSpec>> = IndexMap::new();
    let mut code_offset = 0;

    let mut symbol_codes = Vec::new();
    let mut relocations = Vec::new();

    // ラベルごとに機械語に変換
    for group in sym.groups.iter() {
        // jump系命令がラベルの前に存在した場合
        if let Some(specs) = relative_jump_offset.get(&group.label) {
            for spec in specs {
                // 相対オフセットの計算
                let relative_offset = code_offset - spec.address;

                for (idx, addr) in (relative_offset as u32).to_le_bytes().iter().enumerate() {
                    symbol_codes[idx + (spec.operand_offset - 4) as usize] = *addr;
                }
            }
        } else {
            // ラベルがjump系命令の前に存在した場合
            if !group.label.ends_with("_entry") {
                // ラベルの位置を保存しておく
                relative_jump_offset.insert(
                    group.label.to_string(),
                    vec![RelativeJumpSpec::new_label(code_offset)],
                );
            }
        }

        for inst in group.insts.iter() {
            // いくつかの命令は再配置シンボルの生成など，
            // 機械語への変換以外にも操作が必要．

            match inst.name() {
                InstName::Call => {
                    // 適当なアドレスを生成しておく
                    let mut inst_bytes = inst.assemble();

                    let rela64 = new_rela64(inst.operand_1().unwrap().copy_label(), code_offset);
                    relocations.push(rela64);

                    code_offset += inst_bytes.len() as isize;
                    symbol_codes.append(&mut inst_bytes);
                }

                // jump
                InstName::Jmp => {
                    let mut inst_bytes = inst.assemble();
                    inst_bytes.append(&mut vec![0x00, 0x00, 0x00, 0x00]);
                    code_offset += inst_bytes.len() as isize;
                    symbol_codes.append(&mut inst_bytes);

                    resolve_jump(
                        &inst.operand_1().unwrap().copy_label(),
                        code_offset,
                        &mut relative_jump_offset,
                        &mut symbol_codes,
                    );
                }
                _ => {
                    let mut inst_bytes = inst.assemble();
                    code_offset += inst_bytes.len() as isize;
                    symbol_codes.append(&mut inst_bytes);
                }
            }
        }
    }

    (symbol_codes, relocations)
}

fn resolve_jump(
    label: &str,
    length: isize,
    relative_jump: &mut IndexMap<String, Vec<RelativeJumpSpec>>,
    sym_codes: &mut Vec<u8>,
) {
    if let Some(specs) = relative_jump.get_mut(label) {
        for spec in specs.iter() {
            // jump -> jump みたいなものは無視
            if !spec.is_label {
                continue;
            }

            // 相対オフセットの計算
            let relative_offset = spec.address - length;
            for (idx, addr) in (relative_offset as i32).to_le_bytes().iter().enumerate() {
                sym_codes[idx + (length - 4) as usize] = *addr;
            }
        }

        specs.push(RelativeJumpSpec::new_jump(length));
    } else {
        // jump系命令がラベルの前に存在した場合
        relative_jump.insert(label.to_string(), vec![RelativeJumpSpec::new_jump(length)]);
    }
}

fn new_rela64(name: String, offset: isize) -> RelaSymbol {
    let mut rela64: RelaSymbol = Default::default();
    rela64.rela64.set_addend(-4);
    rela64.name = name;

    // opcode 分スキップ
    rela64.rela64.set_offset(offset as u64 + 1);

    rela64
}
