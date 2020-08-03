[![x64_asm at crates.io](https://img.shields.io/crates/v/x64_asm.svg)](https://crates.io/crates/x64_asm)  [![x64_asm at docs.rs](https://docs.rs/x64_asm/badge.svg)](https://docs.rs/x64_asm)

# x64_asm
x86_64 assembler

## Get Started

See **[documentation](https://docs.rs/x64_asm)**

```rust
use x64_asm;
use x64_asm::resources;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    // you can pass a file(or string).
    let elf_builder = x64_asm::assemble_file("asm.s", "obj.o", resources::ATANDT)?;
    
    elf_builder.generate_elf_file(0o644);

    Ok(())
}
```

##  Dependencies

- [Drumato/elf-utilities](https://github.com/Drumato/elf-utilities)