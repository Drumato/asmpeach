[![asmpeach at crates.io](https://img.shields.io/crates/v/asmpeach.svg)](https://crates.io/crates/asmpeach)  [![asmpeach at docs.rs](https://docs.rs/asmpeach/badge.svg)](https://docs.rs/asmpeach)

# asmpeach
x86_64 assembler

## Get Started

### How to use as an assembler command

```
cargo build
./target/debug/asmpeach <assembly-file in AT&T syntax>
```

### How to use as a Rust crate

See **[documentation](https://docs.rs/asmpeach)**

```rust
use asmpeach;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    // you can pass a file(or string).
    let elf_builder = asmpeach::assemble_file("asm.s", asmpeach::Syntax::ATANDT)?;
    
    elf_builder.generate_elf_file("obj.o", 0o644)?;

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
