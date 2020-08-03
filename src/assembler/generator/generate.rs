use crate::{Group, Opcode, Symbol};
use indexmap::map::IndexMap;

type OffsetForRelativeJump = IndexMap<String, (isize, isize)>;

pub fn generate_main(symbols: &mut IndexMap<String, Symbol>) {
    for (_sym_name, sym) in symbols.iter_mut() {
        let (mut sym_codes, relative_jump_offset) = gen_symbol_code(sym);

        resolve_relative_offset_jump(&mut sym_codes, &relative_jump_offset);

        // アラインメント調整
        let mut extra_bytes: Vec<u8> = Vec::new();

        let rest_bytes = sym_codes.len() % 4;
        for _ in 0..(4 - rest_bytes) {
            extra_bytes.push(0x00);
        }
        sym_codes.append(&mut extra_bytes);

        sym.codes = sym_codes;
    }
}

fn gen_symbol_code(sym: &Symbol) -> (Vec<u8>, OffsetForRelativeJump) {
    let mut relative_jump_offset = IndexMap::new();
    let mut code_offset = 0;

    let mut symbol_codes = Vec::new();

    // ラベルごとに機械語に変換
    for group in sym.groups.iter() {
        let mut codes_in_group = gen_group_code(&mut code_offset, group, &mut relative_jump_offset);
        symbol_codes.append(&mut codes_in_group);
    }

    (symbol_codes, relative_jump_offset)
}

fn gen_group_code(
    code_offset: &mut isize,
    group: &Group,
    relative_jump_offset: &mut OffsetForRelativeJump,
) -> Vec<u8> {
    let mut codes_in_group = Vec::new();

    // jump系命令がラベルの前に存在した場合
    if let Some(tup) = relative_jump_offset.get_mut(&group.label) {
        // ラベルまでのバイト数 - ジャンプの位置 - 1 => 相対オフセット
        tup.1 = *code_offset - tup.1 - 1;
    } else {
        // ラベルがjump系命令の前に存在した場合
        if !group.label.ends_with("_entry") {
            // ラベルの位置を保存しておく
            relative_jump_offset.insert(group.label.to_string(), (0, *code_offset));
        }
    }

    for inst in group.insts.iter() {
        let mut inst_codes = match &inst.opcode {
            Opcode::CALLFUNC(_func) => {
                // 適当なアドレスを生成しておく．
                vec![0xe8, 0x00, 0x00, 0x00, 0x00]
            }
            // jump
            Opcode::JELABEL { label } => {
                let length = *code_offset + 2;

                if let Some(tup) = relative_jump_offset.get_mut(label) {
                    // ラベルがjump系命令の前に存在した場合
                    tup.0 = length;
                    tup.1 = !(length + 4 - tup.1) + 1;
                } else {
                    // jump系命令がラベルの前に存在した場合
                    relative_jump_offset.insert(label.to_string(), (length, length + 3));
                }

                let mut base_bytes = inst.to_bytes();
                base_bytes.append(&mut vec![0x00, 0x00, 0x00, 0x00]);

                base_bytes
            }
            Opcode::JMPLABEL { label } => {
                let length = *code_offset + 1;

                if let Some(tup) = relative_jump_offset.get_mut(label) {
                    // ラベルがjump系命令の前に存在した場合
                    tup.0 = length;
                    tup.1 = !(length + 4 - tup.1) + 1;
                } else {
                    // jump系命令がラベルの前に存在した場合
                    relative_jump_offset.insert(label.to_string(), (length, length + 3));
                }

                let mut base_bytes = inst.to_bytes();
                base_bytes.append(&mut vec![0x00, 0x00, 0x00, 0x00]);

                base_bytes
            }
            _ => inst.to_bytes(),
        };

        // jmp用にオフセットを更新
        *code_offset += inst_codes.len() as isize;

        codes_in_group.append(&mut inst_codes);
    }

    codes_in_group
}

/// 相対ジャンプを解決する
fn resolve_relative_offset_jump(sym_codes: &mut Vec<u8>, relative_jump_offset: &OffsetForRelativeJump) {
    for (_name, (dst, offset)) in relative_jump_offset.iter() {
        for (idx, byte) in (*offset as u32).to_le_bytes().iter().enumerate() {
            sym_codes[idx + *dst as usize] = *byte;
        }
    }
}