use crate::assembler::resource::{Group, Opcode, RelaSymbol, Symbol};
use indexmap::map::IndexMap;

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Copy, Clone)]
struct RelativeJumpSpec {
    operand_offset: isize,
    address: isize,
}

pub fn generate_main(symbols: &mut IndexMap<String, Symbol>) -> IndexMap<String, Vec<RelaSymbol>> {
    let mut reloc_syms = IndexMap::new();

    for (sym_name, sym) in symbols.iter_mut() {
        let (mut sym_codes, relative_jump_offset, relocs_in_sym) = gen_symbol_code(sym);
        reloc_syms.insert(sym_name.to_string(), relocs_in_sym);

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

    reloc_syms
}

fn gen_symbol_code(sym: &Symbol) -> (Vec<u8>,  IndexMap<String, Vec<RelativeJumpSpec>>, Vec<RelaSymbol>) {
    let mut relative_jump_offset = IndexMap::new();
    let mut code_offset = 0;

    let mut symbol_codes = Vec::new();
    let mut relocations = Vec::new();

    // ラベルごとに機械語に変換
    for group in sym.groups.iter() {
        let (mut codes_in_group, mut relocs_in_group) =
            gen_group_code(&mut code_offset, group, &mut relative_jump_offset);
        symbol_codes.append(&mut codes_in_group);

        // グループ内の再配置情報を合成
        relocations.append(&mut relocs_in_group)
    }
    (symbol_codes, relative_jump_offset, relocations)
}

fn gen_group_code(
    code_offset: &mut isize,
    group: &Group,
    relative_jump_offset: &mut IndexMap<String, Vec<RelativeJumpSpec>>,
) -> (Vec<u8>, Vec<RelaSymbol>) {
    let mut codes_in_group = Vec::new();
    let mut relocs_in_group = Vec::new();

    // jump系命令がラベルの前に存在した場合
    if let Some(specs) = relative_jump_offset.get_mut(&group.label) {
        for spec in specs {
            // ラベルまでのバイト数 - ジャンプ命令の位置 => 相対オフセット
            // operand部までのオフセットから求めるため, -1する
            spec.address = *code_offset - spec.operand_offset - 4;
        }
    } else {
        // ラベルがjump系命令の前に存在した場合
        if !group.label.ends_with("_entry") {
            // ラベルの位置を保存しておく
            relative_jump_offset.insert(
                group.label.to_string(),
                vec![RelativeJumpSpec {
                    operand_offset: 0,
                    address: *code_offset,
                }],
            );
        }
    }

    for inst in group.insts.iter() {
        // いくつかの命令は再配置シンボルの生成など，
        // 機械語への変換以外にも操作が必要．
        let mut inst_codes = match &inst.opcode {
            Opcode::CALLFUNC(func) => {
                // 再配置用にシンボルを作る
                let mut rela64: RelaSymbol = Default::default();
                rela64.rela64.set_addend(-4);
                rela64.name = func.copy_label();

                // opcode 分スキップして，再配置オフセットを設定
                rela64.rela64.set_offset(*code_offset as u64 + 1);
                relocs_in_group.push(rela64);

                // 適当なアドレスを生成しておく．
                vec![0xe8, 0x00, 0x00, 0x00, 0x00]
            }

            // jump
            Opcode::JELABEL { label } => {
                // オペランド部はオペコード分,2バイト+した場所にある
                let length = *code_offset + 2;

                if let Some(specs) = relative_jump_offset.get_mut(label) {
                    specs.push(RelativeJumpSpec {
                        operand_offset: length,
                        address: length,
                    });

                    for spec in specs {
                        // ラベルがjump系命令の前に存在した場合
                        // ラベル位置とjumpのオペランド部の差分を計算する.
                        spec.address = length - spec.address - 3;
                    }
                } else {
                    // jump系命令がラベルの前に存在した場合
                    relative_jump_offset.insert(
                        label.to_string(),
                        vec![RelativeJumpSpec {
                            operand_offset: length,
                            address: length,
                        }],
                    );
                }

                let mut base_bytes = inst.to_bytes();
                base_bytes.append(&mut vec![0x00, 0x00, 0x00, 0x00]);

                base_bytes
            }
            Opcode::JMPLABEL { label } => {
                // オペランド部はオペコード分,1バイト+した場所にある
                let length = *code_offset + 1;
                if let Some(specs) = relative_jump_offset.get_mut(label) {
                    specs.push(RelativeJumpSpec {
                        operand_offset: length,
                        address: length,
                    });

                    for spec in specs {
                        // ラベルがjump系命令の前に存在した場合
                        // ラベル位置とjumpのオペランド部の差分を計算する.
                        spec.address = length - spec.address - 3;
                    }
                } else {

                    // jump系命令がラベルの前に存在した場合
                    relative_jump_offset.insert(
                        label.to_string(),
                        vec![RelativeJumpSpec {
                            operand_offset: length,
                            address: length,
                        }],
                    );
                }

                let mut base_bytes = inst.to_bytes();
                base_bytes.append(&mut vec![0x00, 0x00, 0x00, 0x00]);

                base_bytes
            }
            _ => inst.to_bytes(),
        };

        // call,jmp用にオフセットを更新
        *code_offset += inst_codes.len() as isize;
        codes_in_group.append(&mut inst_codes);
    }

    (codes_in_group, relocs_in_group)
}

/// 0パディングされたjumpのオフセットを更新する.
fn resolve_relative_offset_jump(
    sym_codes: &mut Vec<u8>,
    relative_jump_offset: &IndexMap<String, Vec<RelativeJumpSpec>>,
) {
    for (name, specs) in relative_jump_offset.iter() {
        if name.ends_with("_entry") {
            continue;
        }

        for spec in specs{
            for (idx, addr) in (spec.address as u32).to_le_bytes().iter().enumerate() {
                sym_codes[idx + spec.operand_offset as usize] = *addr;
            }
        }
    }
}
