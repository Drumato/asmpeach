#[derive(Eq, Ord, PartialOrd, PartialEq, Debug, Clone, Copy)]
pub enum Encoding {
    ZO,

    /// Ope1 -> ModRM:reg,   Ope2 -> ModRM:r/m
    RM,
    /// Ope1 -> ModRM:r/m,   Ope2 -> ModRM:reg
    MR,
    /// Ope1 -> ModRM:r/m,   Ope2 -> imm8/16/32/64
    MI,
    /// Ope1 -> opcode + rd, Ope2 -> imm8/16/32/64
    OI,
    /// Ope1 -> opcode + rd
    O,
    /// Ope1 -> imm8/16/32
    I,
    /// Ope1 -> ModRM:r/m
    M,

}