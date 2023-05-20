mod opcode;

pub use opcode::Opcode;

pub const PIE_HEADER_PREFIX: [u8; 4] = *b"EPIE";
pub const PIE_HEADER_LENGTH: usize = 64;
