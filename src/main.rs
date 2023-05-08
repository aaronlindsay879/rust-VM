use crate::repl::REPL;

mod assembler;
mod instruction;
mod opcode;
mod parser;
mod repl;
mod vm;

const PIE_HEADER_PREFIX: [u8; 4] = *b"EPIE";
const PIE_HEADER_LENGTH: usize = 64;

fn main() {
    let mut repl = REPL::default();
    repl.run();
}
