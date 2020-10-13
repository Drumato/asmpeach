use elf_utilities::file::ELF64;

pub struct ELFBuilder {
    pub file: ELF64,
}

impl ELFBuilder {
    pub fn new() -> Self {
        Self {
            file: ELF64::new(Self::initialize_elf64_header()),
        }
    }
    pub fn give_file(self) -> elf_utilities::file::ELF64 {
        self.file
    }

    pub fn add_section(&mut self, section: elf_utilities::section::Section64) {
        self.file.add_section(section);
    }

    fn initialize_elf64_header() -> elf_utilities::header::Ehdr64 {
        let mut ehdr: elf_utilities::header::Ehdr64 = Default::default();

        // アーキテクチャ -> X86_64
        ehdr.set_machine(elf_utilities::header::Machine::X8664);

        // クラス -> 64bit
        ehdr.set_class(elf_utilities::header::Class::Bit64);

        // タイプ -> RELOCATION
        ehdr.set_elf_type(elf_utilities::header::Type::Rel);

        // データ -> Little Endian
        ehdr.set_data(elf_utilities::header::Data::LSB2);

        // バージョン -> EV_CURRENT
        ehdr.set_file_version(elf_utilities::header::Version::Current);

        ehdr
    }
}
