use crate::assembler::resources::{RelaSymbol, Symbol};
use indexmap::IndexMap;

/// 再配置情報の更新
/// 再配置シンボルに対応するシンボルをsymbolsから探し出し，infoを更新する
pub fn setup_relocation(symbols: &IndexMap<String, Symbol>, reloc_syms: &mut IndexMap<String, Vec<RelaSymbol>>) {

    let mut current_offset = 0;

    for (sym_name, sym) in symbols.iter() {

        if let Some(relocations) = reloc_syms.get_mut(sym_name){
            for rela in relocations.iter_mut() {
                // シンボル内でのオフセットからテーブル全体でのオフセットに
                let offset_in_symbol = rela.rela64.get_offset();
                rela.rela64.set_offset(offset_in_symbol + current_offset);

                // 呼び出された名前(再配置対象)と一致しなければ関係ない
                // eprintln!("{} == {}", rela.name, sym_name);
                // if &rela.name != sym_name {
                //     continue;
                // }

                // NULL シンボル + セクションシンボル数のことを考えて+2する
                // 存在しない場合はリンカがあとから関連付けるので，0としておく．
                let relation_idx = match symbols.get_index_of(&rela.name) {
                    Some(callee_idx) => callee_idx as u64 + 2,
                    None => 0,
                };
                // シンボルテーブルのインデックスはr_infoのうち上位32bitを使う
                let relation_idx = relation_idx << 32;
                rela.rela64.set_info    (relation_idx + elf_utilities::relocation::R_X86_64_PLT32);
            }
        }

        current_offset += sym.codes.len() as u64;
    }
}