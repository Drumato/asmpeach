[![x64_asm at crates.io](https://img.shields.io/crates/v/x64_asm.svg)](https://crates.io/crates/x64_asm)  [![x64_asm at docs.rs](https://docs.rs/x64_asm/badge.svg)](https://docs.rs/x64_asm)

# x64_asm
x86_64 assembler

## Get Started

See **[documentation](https://docs.rs/x64_asm)**

```rust
use x64_asm::*;

fn main() {
    // mov [rax + rbx * 4], rcx
    let inst = Instruction {
        opcode: Opcode::MOVRM64R64 {
            rm64: Operand::ADDRESSING {
                base_reg: GeneralPurposeRegister::RAX,
                index_reg: Some(GeneralPurposeRegister::RBX),
                displacement: None,
                scale: Some(0x4),
            },
            r64: Operand::GENERALREGISTER(GeneralPurposeRegister::RCX),
        }
    };

    // we can assemble(translate to machine-code) Every instruction.
    assert_eq!(inst.to_bytes(), vec![0x48, 0x89, 0x0c, 0x98]);
    
    // also can generate AT&T/Intel syntax's string for creating assembly files.
    assert_eq!(inst.to_intel_string(), "mov QWORD PTR [rax + rbx * 4], rcx");
    assert_eq!(inst.to_at_string(), "movq %rcx, (%rax, %rbx, 4)");
}
```