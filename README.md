[![x64_asm at crates.io](https://img.shields.io/crates/v/x64_asm.svg)](https://crates.io/crates/x64_asm)  [![x64_asm at docs.rs](https://docs.rs/x64_asm/badge.svg)](https://docs.rs/x64_asm)

# x64_asm
x86_64 assembler

## Get Started

### How to use as an assembler command

```
cargo build
./target/debug/x64_asm <assembly-file in AT&T syntax>
```

### How to use as a Rust crate

See **[documentation](https://docs.rs/x64_asm)**

```rust
use x64_asm;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    // you can pass a file(or string).
    let elf_builder = x64_asm::assemble_file("asm.s", "obj.o", x64_asm::Syntax::ATANDT)?;
    
    elf_builder.generate_elf_file(0o644);

    Ok(())
}
```

## Test

```
cargo test
cargo test --features sample
```

##  Dependencies

- [Drumato/elf-utilities](https://github.com/Drumato/elf-utilities)
