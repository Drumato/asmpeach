use elf_utilities::{
    file::ELF64,
};
use std::io::{BufWriter, Write};
use std::os::unix::fs::OpenOptionsExt;

pub struct ELFBuilder {
    output_filepath: String,
    file: ELF64,
}

impl ELFBuilder {
    pub fn new(file_path: String) -> Self {
        Self {
            output_filepath: file_path,
            file: ELF64::new(Default::default()),
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
}