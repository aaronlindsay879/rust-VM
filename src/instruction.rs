use crate::opcode::Opcode;
use num_traits::cast::FromPrimitive;
use std::collections::VecDeque;

/// Entire instruction for VM
#[derive(Debug, PartialEq)]
pub struct Instruction {
    pub opcode: Opcode,
    buffer: VecDeque<u8>,
}

impl Instruction {
    /// Creates an instruction with an internal buffer for reading operand values.
    /// Returns None if less than 4 bytes given
    pub fn from<T: AsRef<[u8]>>(slice: T) -> Option<Self> {
        let slice = slice.as_ref();
        if slice.len() < 4 {
            return None;
        }

        let opcode = Opcode::from_u8(slice[0]).unwrap_or(Opcode::IGL);

        let mut buffer = VecDeque::with_capacity(3);
        buffer.extend(slice[1..].iter());

        Some(Self { opcode, buffer })
    }

    /// Reads u8 from internal buffer.
    /// Will panic if buffer is empty.
    pub fn next_u8(&mut self) -> u8 {
        self.buffer.pop_front().unwrap()
    }

    /// Reads u16 from internal buffer.
    /// Will panic if buffer is empty.
    pub fn next_u16(&mut self) -> u16 {
        let bytes = [
            self.buffer.pop_front().unwrap(),
            self.buffer.pop_front().unwrap(),
        ];

        u16::from_be_bytes(bytes)
    }

    /// Reads u8 from internal buffer, and returns the value from the register with that index.
    /// Will panic if buffer is empty.
    pub fn next_register(&mut self, registers: &[i32]) -> i32 {
        registers[self.next_u8() as usize]
    }

    /// Reads u8 from internal buffer, and returns a mutable reference to the register with that index.
    /// Will panic if buffer is empty.
    pub fn next_register_mut<'a, 'b>(&'a mut self, registers: &'b mut [i32]) -> &'b mut i32 {
        &mut registers[self.next_u8() as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_instruction() {
        let instruction = Instruction::from([0, 0, 0, 0]);

        assert_eq!(instruction.unwrap().opcode, Opcode::HLT);
    }
}
