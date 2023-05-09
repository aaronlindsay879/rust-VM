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
    LBI = 0b00000100,
    /// Loads value from memory into register
    LBD = 0b00000101,
    /// Loads half-word value into register
    LHI = 0b00001000,
    /// Loads half-word from memory into register
    LHD = 0b00001001,
    /// Loads word from memory into register
    LWD = 0b00001101,
    /// Stores byte from register into memory with address from raw value
    SBI = 0b00010000,
    /// Stores half-word from register into memory with address from raw value
    SHI = 0b00010100,
    /// Stores word from register into memory with address from raw value
    SWI = 0b00011000,
    /// Copies register value
    MOV = 0b00011110,
    /// Adds two registers
    ADR = 0b01000010,
    /// Adds a register and a literal
    ADI = 0b01000000,
    /// Subtracts two registers
    SUR = 0b01000110,
    /// Subtracts a register and a literal
    SUI = 0b01000100,
    /// Multiplies two registers
    MLR = 0b01001010,
    /// Multiplies a register and a literal
    MLI = 0b01001000,
    /// Divides two registers
    DVR = 0b01001110,
    /// Divides a register and a literal
    DVI = 0b01001100,
    /// Illegal instruction
    IGL = 0b11111111,
}

impl From<&str> for Opcode {
    fn from(value: &str) -> Self {
        match &value.to_lowercase()[..] {
            "hlt" => Opcode::HLT,
            "lbi" => Opcode::LBI,
            "lbd" => Opcode::LBD,
            "lhi" => Opcode::LHI,
            "lhd" => Opcode::LHD,
            "lwd" => Opcode::LWD,
            "sbi" => Opcode::SBI,
            "shi" => Opcode::SHI,
            "swi" => Opcode::SWI,
            "mov" => Opcode::MOV,
            "adr" => Opcode::ADR,
            "adi" => Opcode::ADI,
            "sur" => Opcode::SUR,
            "sui" => Opcode::SUI,
            "mlr" => Opcode::MLR,
            "mli" => Opcode::MLI,
            "dvr" => Opcode::DVR,
            "dvi" => Opcode::DVI,
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
        let opcode = Opcode::from("lbi");
        assert_eq!(opcode, Opcode::LBI);

        let opcode = Opcode::from("illegal");
        assert_eq!(opcode, Opcode::IGL);
    }
}
