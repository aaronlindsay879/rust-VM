mod instruction_parser;
mod opcode_parsers;
mod operand_parsers;
mod program_parsers;
mod register_parsers;

use crate::opcode::Opcode;
pub use program_parsers::program;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Token {
    Op { code: Opcode },
    Register { reg_num: u8 },
    IntegerOperand { value: i32 },
}
