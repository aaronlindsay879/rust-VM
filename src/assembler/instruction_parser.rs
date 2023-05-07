use super::{opcode_parsers::*, operand_parsers::*, register_parsers::register, Token};
use nom::types::CompleteStr;
use nom::{do_parse, named};

#[derive(Debug, PartialEq)]
pub struct AssemblerInstruction {
    pub opcode: Token,
    pub operand1: Option<Token>,
    pub operand2: Option<Token>,
    pub operand3: Option<Token>,
}

impl AssemblerInstruction {
    pub fn to_bytes(&self) -> Result<Vec<u8>, &str> {
        let mut out = Vec::with_capacity(4);

        // make sure opcode is actually an opcode
        match self.opcode {
            Token::Op { code } => out.push(code as u8),
            _ => return Err("Non-opcode in opcode position"),
        };

        for operand in [self.operand1, self.operand2, self.operand3] {
            if let Some(token) = operand {
                match token {
                    Token::Register { reg_num } => out.push(reg_num),
                    Token::IntegerOperand { value } => {
                        let shortened = value as u16;
                        out.extend_from_slice(&shortened.to_be_bytes());
                    }
                    _ => return Err("Opcode in operand position"),
                }
            }
        }

        Ok(out)
    }
}

named!(
    pub instruction_one<CompleteStr, AssemblerInstruction>,
    do_parse!(
        o: opcode_load >>
        r: register >>
        i: integer_operand >>
        (
            AssemblerInstruction{
                opcode: o,
                operand1: Some(r),
                operand2: Some(i),
                operand3: None
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
    fn test_parse_instruction_form_one() {
        let result = instruction_one(CompleteStr("load $0 #100\n"));
        assert_eq!(
            result,
            Ok((
                CompleteStr(""),
                AssemblerInstruction {
                    opcode: Token::Op { code: Opcode::LOAD },
                    operand1: Some(Token::Register { reg_num: 0 }),
                    operand2: Some(Token::IntegerOperand { value: 100 }),
                    operand3: None
                }
            ))
        );
    }
}
