use crate::assembler::instruction_parser::{instruction, AssemblerInstruction};
use crate::assembler::SymbolTable;
use nom::types::CompleteStr;
use nom::{do_parse, many1, named};

#[derive(Debug, PartialEq)]
pub struct Program {
    pub(crate) instructions: Vec<AssemblerInstruction>,
}

impl Program {
    pub fn to_bytes(&self, symbols: &SymbolTable) -> Option<Vec<u8>> {
        let mut out = Vec::with_capacity(self.instructions.len() * 4);

        for inst in &self.instructions {
            match inst.to_bytes(symbols) {
                Ok(bytes) => out.extend_from_slice(&bytes),
                Err(_) => return None,
            }
        }

        Some(out)
    }
}

named!(pub program<CompleteStr, Program>,
    do_parse!(
        instructions: many1!(instruction) >>
        (
            Program {
                instructions
            }
        )
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assembler::Token;
    use crate::opcode::Opcode;

    #[test]
    fn test_parse_program_one() {
        let result = program(CompleteStr("load $0 #100\n"));
        assert_eq!(result.is_ok(), true);

        let (leftover, p) = result.unwrap();
        assert_eq!(leftover, CompleteStr(""));
        assert_eq!(
            p,
            Program {
                instructions: vec![AssemblerInstruction {
                    opcode: Some(Token::Op { code: Opcode::LOAD }),
                    operand1: Some(Token::Register { reg_num: 0 }),
                    operand2: Some(Token::IntegerOperand { value: 100 }),
                    ..Default::default()
                }]
            }
        )
    }

    #[test]
    fn test_parse_program_two() {
        let result = program(CompleteStr("add $3 $4 $ 5\n"));
        assert_eq!(result.is_ok(), true);

        let (leftover, p) = result.unwrap();
        assert_eq!(leftover, CompleteStr(""));
        assert_eq!(
            p,
            Program {
                instructions: vec![AssemblerInstruction {
                    opcode: Some(Token::Op { code: Opcode::ADD }),
                    operand1: Some(Token::Register { reg_num: 3 }),
                    operand2: Some(Token::Register { reg_num: 4 }),
                    operand3: Some(Token::Register { reg_num: 5 }),
                    ..Default::default()
                }]
            }
        )
    }

    #[test]
    fn test_program_to_bytes() {
        let result = program(CompleteStr("load $1 #1000\n"));
        assert_eq!(result.is_ok(), true);

        let (_, program) = result.unwrap();
        let bytecode = program.to_bytes(&SymbolTable::new()).unwrap();
        assert_eq!(bytecode.len(), 4);
        println!("{:?}", bytecode);
    }
}
