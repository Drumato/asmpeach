use crate::assembler::resources::{RelaSymbol, Symbol};
use indexmap::IndexMap;

/// 再配置情報の更新
/// 再配置シンボルに対応するシンボルをsymbolsから探し出し，infoを更新する
pub fn setup_relocation(symbols: &IndexMap<String, Symbol>, reloc_syms: &mut IndexMap<String, Vec<RelaSymbol>>) {

    let mut current_offset = 0;

    for (sym_idx, (sym_name, sym)) in symbols.iter().enumerate() {

        if let Some(relocations) = reloc_syms.get_mut(sym_name){
            for rela in relocations.iter_mut() {
                // シンボル内でのオフセットからテーブル全体でのオフセットに
                let offset_in_symbol = rela.rela64.get_offset();
                rela.rela64.set_offset(offset_in_symbol + current_offset);

                // 呼び出された名前(再配置対象)と一致しなければ関係ない
                if &rela.name != sym_name {
                    continue;
                }

                // NULL シンボル + セクションシンボル数のことを考えて+2する
                let relation_idx = (sym_idx + 2) as u64;
                // シンボルテーブルのインデックスはr_infoのうち上位32bitを使う
                let relation_idx = relation_idx << 32;
                rela.rela64.set_info    (relation_idx + elf_utilities::relocation::R_X86_64_PLT32);
            }
        }

        current_offset += sym.codes.len() as u64;
    }
}