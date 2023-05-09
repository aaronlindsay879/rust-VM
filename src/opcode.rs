/// Opcodes for VM, 8 bits\
/// Upper 6 bits = opcode\
/// Lower 2 bits = addressing mode\
/// 00 => Literal value, 01 => From memory, 10 => From Register
#[derive(Debug, PartialEq, Copy, Clone, num_derive::FromPrimitive)]
#[repr(u8)]
#[allow(clippy::upper_case_acronyms)]
pub enum Opcode {
    /// Halt
    HLT = 0b00000000,
    /// Loads byte value into register
    LDBI = 0b00000100,
    /// Loads value from memory into register
    LDBD = 0b00000101,
    /// Loads half-word value into register
    LDHI = 0b00001000,
    /// Loads half-word from memory into register
    LDHD = 0b00001001,
    /// Loads word from memory into register
    LDWD = 0b00001101,
    /// Stores byte from register into memory with address from raw value
    STRBI = 0b00010000,
    /// Stores half-word from register into memory with address from raw value
    STRHI = 0b00010100,
    /// Stores word from register into memory with address from raw value
    STRWI = 0b00011000,
    /// Copies register value
    MOV = 0b00011110,
    /// Adds two registers
    ADDR = 0b01000010,
    /// Adds a register and a literal
    ADDI = 0b01000000,
    /// Subtracts two registers
    SUBR = 0b01000110,
    /// Subtracts a register and a literal
    SUBI = 0b01000100,
    /// Multiplies two registers
    MULR = 0b01001010,
    /// Multiplies a register and a literal
    MULI = 0b01001000,
    /// Divides two registers
    DIVR = 0b01001110,
    /// Divides a register and a literal
    DIVI = 0b01001100,
    /// Illegal instruction
    IGL = 0b11111111,
}

impl From<&str> for Opcode {
    fn from(value: &str) -> Self {
        match &value.to_lowercase()[..] {
            "hlt" => Opcode::HLT,
            "ldbi" => Opcode::LDBI,
            "ldbd" => Opcode::LDBD,
            "ldhi" => Opcode::LDHI,
            "ldhd" => Opcode::LDHD,
            "ldwd" => Opcode::LDWD,
            "strbi" => Opcode::STRBI,
            "strhi" => Opcode::STRHI,
            "strwi" => Opcode::STRWI,
            "mov" => Opcode::MOV,
            "addr" => Opcode::ADDR,
            "addi" => Opcode::ADDI,
            "subr" => Opcode::SUBR,
            "subi" => Opcode::SUBI,
            "mulr" => Opcode::MULR,
            "muli" => Opcode::MULI,
            "divr" => Opcode::DIVR,
            "divi" => Opcode::DIVI,
            _ => Opcode::IGL,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_traits::cast::FromPrimitive;

    #[test]
    fn test_create_hlt() {
        let opcode = Opcode::from_u8(0);

        assert_eq!(opcode, Some(Opcode::HLT));
    }

    #[test]
    fn test_str_to_opcode() {
        let opcode = Opcode::from("ldbi");
        assert_eq!(opcode, Opcode::LDBI);

        let opcode = Opcode::from("illegal");
        assert_eq!(opcode, Opcode::IGL);
    }
}
