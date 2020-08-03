use elf_utilities::{
    file::ELF64,
};
use std::io::{BufWriter, Write};
use std::os::unix::fs::OpenOptionsExt;
use crate::Symbol;
use indexmap::map::IndexMap;

pub struct ELFBuilder {
    output_filepath: String,
    file: ELF64,
}

impl ELFBuilder {
    pub fn new(file_path: String) -> Self {
        Self {
            output_filepath: file_path,
            file: ELF64::new(Self::initialize_elf64_header()),
        }
    }
    pub fn generate_elf_file(&self, mode: u32) {
        let bytes = self.file.to_le_bytes();

        let file = std::fs::OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .mode(mode)
            .open(&self.output_filepath)
            .unwrap();
        let mut writer = BufWriter::new(file);
        match writer.write_all(&bytes) {
            Ok(_) => (),
            Err(e) => eprintln!("{}", e),
        }
        match writer.flush() {
            Ok(_) => (),
            Err(e) => eprintln!("{}", e),
        }
    }

    pub fn add_section(&mut self, section: elf_utilities::section::Section64) {
        self.file.add_section(section);
    }

    pub fn add_text_section(&mut self, symbols: &IndexMap<String, Symbol>) {
        // すべてのシンボルのコードを結合する
        let mut all_symbol_codes: Vec<u8> = Vec::new();

        for (_name, sym) in symbols.iter() {
            let mut symbol_codes = sym.codes.clone();
            all_symbol_codes.append(&mut symbol_codes);
        }

        // .textセクションの生成
        let text_shdr =
            self.init_text_section_header(all_symbol_codes.len());
        let mut text_section =
            elf_utilities::section::Section64::new(".text".to_string(), text_shdr);
        text_section.bytes = Some(all_symbol_codes);

        self.add_section(text_section);
    }

    pub fn add_symbol_table_section(&mut self, symbols: &IndexMap<String, Symbol>) {
        // NULLシンボル + .textシンボル (+ .rodataシンボル)
        let mut elf_symbols = vec![
            elf_utilities::symbol::Symbol64::new_null_symbol(),
            self.create_section_symbol(1),
        ];

        // シンボルを走査する
        // name_indexの操作も行う.
        // また,各シンボルのオフセットも計算する.
        let mut symbol_name_index: elf_utilities::Elf64Word = 1; // 最初のnull文字を飛ばす
        let mut symbol_offset: elf_utilities::Elf64Addr = 0; // st_value用

        for (symbol_name, symbol_info) in symbols.iter() {
            let symbol_code_length = symbol_info.codes.len();
            let symbol_name_length = symbol_name.len();

            let mut global_symbol = self.create_global_symbol(
                symbol_name_index,
                symbol_code_length as u64,
                symbol_offset,
            );
            global_symbol.set_symbol_name(symbol_name.to_string());
            elf_symbols.push(global_symbol);

            // シンボル名を指すインデックスの更新( null byte を見越して+1する)
            symbol_name_index += symbol_name_length as elf_utilities::Elf64Word + 1;

            // オフセットの更新
            // 後ろのシンボルのオフセット <- 前のシンボルのサイズの総合値
            symbol_offset += symbol_code_length as elf_utilities::Elf64Addr;
        }

        let symbol_table_size = elf_symbols.len() * elf_utilities::symbol::Symbol64::size() as usize;
        // セクションの追加
        let symtab_section_header =
            self.init_symbol_table_section_header(symbol_table_size as u64);
        let mut symtab_section =
            elf_utilities::section::Section64::new(".symtab".to_string(), symtab_section_header);
        symtab_section.symbols = Some(elf_symbols);
        self.add_section(symtab_section);
    }


    pub fn add_symtab_string_section(&mut self, symbols: &IndexMap<String, Symbol>) {
        // シンボルマップをイテレートして,名前を集める.
        let symbol_names: Vec<&str> = symbols
            .iter()
            .map(|(name, _)| name.as_str())
            .collect::<Vec<&str>>();

        let symbol_string_table = elf_utilities::section::build_string_table(symbol_names);
        let strtab_header =
            self.init_string_table_header(symbol_string_table.len() as elf_utilities::Elf64Xword);
        let mut strtab_section =
            elf_utilities::section::Section64::new(".strtab".to_string(), strtab_header);
        strtab_section.bytes = Some(symbol_string_table);
        self.add_section(strtab_section);
    }

    pub fn add_nodata_section(&mut self) {
        let nodata_header = self.init_nodata_header();
        let mut nodata_section =
            elf_utilities::section::Section64::new(".nodata".to_string(), nodata_header);
        nodata_section.bytes = Some(Vec::new());
        self.add_section(nodata_section);
    }

    pub fn add_shstrtab_string_section(&mut self) {
        // TODO: 決め打ち
        let section_names = vec![
            ".text",
            ".symtab",
            ".strtab",
            ".nodata",
            ".shstrtab",
        ];

        let section_string_table = elf_utilities::section::build_string_table(section_names);
        let shstrtab_header =
            self.init_string_table_header(section_string_table.len() as elf_utilities::Elf64Xword);
        let mut shstrtab_section =
            elf_utilities::section::Section64::new(".shstrtab".to_string(), shstrtab_header);
        shstrtab_section.bytes = Some(section_string_table);
        self.add_section(shstrtab_section);
    }

    pub fn condition_elf_header(&mut self) {
        self.file.condition();
    }

    fn init_text_section_header(
        &self,
        length: usize,
    ) -> elf_utilities::section::Shdr64 {
        let mut shdr: elf_utilities::section::Shdr64 = Default::default();

        shdr.set_type(elf_utilities::section::TYPE::PROGBITS);
        shdr.set_size(length as elf_utilities::Elf64Xword);
        shdr.set_addralign(1);
        shdr.set_flags(elf_utilities::section::SHF_ALLOC | elf_utilities::section::SHF_EXECINSTR);

        shdr
    }

    fn init_symbol_table_section_header(
        &self,
        length: elf_utilities::Elf64Xword,
    ) -> elf_utilities::section::Shdr64 {
        let mut shdr: elf_utilities::section::Shdr64 = Default::default();

        shdr.set_type(elf_utilities::section::TYPE::SYMTAB);
        shdr.set_size(length);
        shdr.set_addralign(1);
        shdr.set_entry_size(elf_utilities::symbol::Symbol64::size());

        // TODO: .strtabが3番目にあることを決め打ち
        shdr.set_link(3);

        // TODO: 最初のグローバルシンボルが3番目にあることを決め打ち
        shdr.set_info(2);
        shdr
    }

    fn init_string_table_header(
        &self,
        length: elf_utilities::Elf64Xword,
    ) -> elf_utilities::section::Shdr64 {
        let mut shdr: elf_utilities::section::Shdr64 = Default::default();

        shdr.set_type(elf_utilities::section::TYPE::STRTAB);
        shdr.set_size(length);
        shdr.set_addralign(1);

        shdr
    }


    fn init_nodata_header(&self) -> elf_utilities::section::Shdr64 {
        let mut shdr: elf_utilities::section::Shdr64 = Default::default();

        shdr.set_type(elf_utilities::section::TYPE::NULL);
        shdr
    }

    fn create_global_symbol(
        &self,
        st_name: elf_utilities::Elf64Word,
        st_size: elf_utilities::Elf64Xword,
        st_offset: elf_utilities::Elf64Addr,
    ) -> elf_utilities::symbol::Symbol64 {
        let mut symbol: elf_utilities::symbol::Symbol64 = Default::default();
        symbol.set_name(st_name);
        symbol.set_size(st_size);
        symbol.set_value(st_offset);

        // TODO: .textが1番目にあることを決め打ち
        symbol.set_shndx(1);

        // グローバル + Function属性
        let sym_info = elf_utilities::symbol::symbol_info(
            elf_utilities::symbol::STB_GLOBAL,
            elf_utilities::symbol::STT_FUNC,
        );
        symbol.set_info(sym_info);

        symbol
    }

    fn create_section_symbol(&self, shndx: u16) -> elf_utilities::symbol::Symbol64 {
        let mut symbol: elf_utilities::symbol::Symbol64 = Default::default();

        symbol.set_shndx(shndx);

        // ローカル + SECTION属性
        let sym_info = elf_utilities::symbol::symbol_info(
            elf_utilities::symbol::STB_LOCAL,
            elf_utilities::symbol::STT_SECTION,
        );
        symbol.set_info(sym_info);

        symbol
    }

    fn initialize_elf64_header() -> elf_utilities::header::Ehdr64 {
        let mut ehdr: elf_utilities::header::Ehdr64 = Default::default();

        // アーキテクチャ -> X86_64
        ehdr.set_machine(elf_utilities::header::ELFMACHINE::EMX8664);

        // クラス -> 64bit
        ehdr.set_class(elf_utilities::header::ELFCLASS::CLASS64);

        // タイプ -> RELOCATION
        ehdr.set_elf_type(elf_utilities::header::ELFTYPE::REL);

        // データ -> Little Endian
        ehdr.set_data(elf_utilities::header::ELFDATA::DATA2LSB);

        // バージョン -> EV_CURRENT
        ehdr.set_version(elf_utilities::header::ELFVERSION::VERSIONCURRENT);

        ehdr
    }
}