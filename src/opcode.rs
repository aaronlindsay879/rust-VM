/// Opcodes for VM, 8 bits
#[derive(Debug, PartialEq, Copy, Clone)]
#[repr(u8)]
pub enum Opcode {
    /// Loads number into register such that {REGISTER} = {16 BIT NUMBER}\
    /// LOAD {REGISTER} {16 bit number}
    LOAD,
    /// Adds two registers such that {OUT REG} = {IN REG 1} + {IN REG 2}\
    /// ADD {IN REG 1} {IN REG 2} {OUT REG}
    ADD,
    /// Subtracts two registers such that {OUT REG} = {IN REG 1} - {IN REG 2}\
    /// SUB {IN REG 1} {IN REG 2} {OUT REG}
    SUB,
    /// Multiplies two registers such that {OUT REG} = {IN REG 1} * {IN REG 2}\
    /// MUL {IN REG 1} {IN REG 2} {OUT REG}
    MUL,
    /// Divides two registers such that {OUT REG} = {IN REG 1} / {IN REG 2}, with remainder stored\
    /// DIV {IN REG 1} {IN REG 2} {OUT REG}
    DIV,
    /// Halt
    HLT,
    /// Jumps to location specified by value in register {REGISTER}\
    /// JMP {REGISTER}
    JMP,
    /// Jumps forwards by the value specified in register {REG}\
    /// JMPF {REGISTER}
    JMPF,
    /// Jumps backwards by the value specified in register {REG}\
    /// JMPB {REGISTER}
    JMPB,
    /// Stores the result of {REG 1} == {REG 2} in equality register\
    /// EQ {REG 1} {REG 2}
    EQ,
    /// Stores the result of {REG 1} != {REG 2} in equality register\
    /// NEQ {REG 1} {REG 2}
    NEQ,
    /// Stores the result of {REG 1} > {REG 2} in equality register\
    /// GT {REG 1} {REG 2}
    GTE,
    /// Stores the result of {REG 1} <= {REG 2} in equality register\
    /// LTE {REG 1} {REG 2}
    GT,
    /// Stores the result of {REG 1} < {REG 2} in equality register\
    /// LT {REG 1} {REG 2}
    LTE,
    /// Jumps to location specified by value in register {REGISTER} if equality register is true\
    /// JMPE {REGISTER}
    LT,
    /// Stores the result of {REG 1} >= {REG 2} in equality register\
    /// GTE {REG 1} {REG 2}
    JMPE,
    /// Jumps to location specified by value in register {REGISTER} if equality register is false\
    /// JMPNE {REGISTER}
    JMPNE,
    /// Illegal opcode
    IGL,
}

impl From<u8> for Opcode {
    fn from(value: u8) -> Self {
        match value {
            0 => Opcode::LOAD,
            1 => Opcode::ADD,
            2 => Opcode::SUB,
            3 => Opcode::MUL,
            4 => Opcode::DIV,
            5 => Opcode::HLT,
            6 => Opcode::JMP,
            7 => Opcode::JMPF,
            8 => Opcode::JMPB,
            9 => Opcode::EQ,
            10 => Opcode::NEQ,
            11 => Opcode::GTE,
            12 => Opcode::GT,
            13 => Opcode::LTE,
            14 => Opcode::LT,
            15 => Opcode::JMPE,
            16 => Opcode::JMPNE,
            _ => Opcode::IGL,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_hlt() {
        let opcode = Opcode::HLT;

        assert_eq!(opcode, Opcode::HLT);
    }
}
