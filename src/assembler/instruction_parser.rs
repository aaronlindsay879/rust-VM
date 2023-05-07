use super::{label_parsers::label_declaration, opcode_parsers::*, operand_parsers::*, Token};
use nom::types::CompleteStr;
use nom::{alt, do_parse, many_m_n, multispace, named, opt};

#[derive(Debug, PartialEq, Default)]
pub struct AssemblerInstruction {
    pub opcode: Option<Token>,
    pub label: Option<Token>,
    pub directive: Option<Token>,
    pub operand1: Option<Token>,
    pub operand2: Option<Token>,
    pub operand3: Option<Token>,
}

impl AssemblerInstruction {
    pub fn to_bytes(&self) -> Result<Vec<u8>, &str> {
        let mut out = Vec::with_capacity(4);

        // make sure opcode is actually an opcode
        match self.opcode {
            Some(Token::Op { code }) => out.push(code as u8),
            _ => return Err("Non-opcode in opcode position"),
        };

        for operand in [&self.operand1, &self.operand2, &self.operand3] {
            if let Some(token) = operand {
                match token {
                    Token::Register { reg_num } => out.push(*reg_num),
                    Token::IntegerOperand { value } => {
                        let shortened = *value as u16;
                        out.extend_from_slice(&shortened.to_be_bytes());
                    }
                    _ => return Err("Opcode in operand position"),
                }
            }
        }

        if out.len() < 4 {
            out.resize(4, 0);
        }

        Ok(out)
    }
}

named!(pub instruction<CompleteStr, AssemblerInstruction>,
    do_parse!(
        l: opt!(label_declaration) >>
        o: opcode >>
        o1: opt!(operand) >>
        o2: opt!(operand) >>
        o3: opt!(operand) >>
        opt!(multispace) >>
        (
            AssemblerInstruction{
                opcode: Some(o),
                label: l,
                directive: None,
                operand1: o1,
                operand2: o2,
                operand3: o3,
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
        let result = instruction(CompleteStr("load $0 #100\n"));
        assert_eq!(
            result,
            Ok((
                CompleteStr(""),
                AssemblerInstruction {
                    opcode: Some(Token::Op { code: Opcode::LOAD }),
                    operand1: Some(Token::Register { reg_num: 0 }),
                    operand2: Some(Token::IntegerOperand { value: 100 }),
                    ..Default::default()
                }
            ))
        );
    }

    #[test]
    fn test_parse_instruction_form_two_0() {
        let result = instruction(CompleteStr("hlt\n"));
        assert_eq!(
            result,
            Ok((
                CompleteStr(""),
                AssemblerInstruction {
                    opcode: Some(Token::Op { code: Opcode::HLT }),
                    ..Default::default()
                }
            ))
        );
    }

    #[test]
    fn test_parse_instruction_form_two_1() {
        let result = instruction(CompleteStr("jmpe $4\n"));
        assert_eq!(
            result,
            Ok((
                CompleteStr(""),
                AssemblerInstruction {
                    opcode: Some(Token::Op { code: Opcode::JMPE }),
                    operand1: Some(Token::Register { reg_num: 4 }),
                    ..Default::default()
                }
            ))
        );
    }

    #[test]
    fn test_parse_instruction_form_two_2() {
        let result = instruction(CompleteStr("eq $3 $4\n"));
        assert_eq!(
            result,
            Ok((
                CompleteStr(""),
                AssemblerInstruction {
                    opcode: Some(Token::Op { code: Opcode::EQ }),
                    operand1: Some(Token::Register { reg_num: 3 }),
                    operand2: Some(Token::Register { reg_num: 4 }),
                    ..Default::default()
                }
            ))
        );
    }

    #[test]
    fn test_parse_instruction_form_two_3() {
        let result = instruction(CompleteStr("add $3 $4 $ 5\n"));
        assert_eq!(
            result,
            Ok((
                CompleteStr(""),
                AssemblerInstruction {
                    opcode: Some(Token::Op { code: Opcode::ADD }),
                    operand1: Some(Token::Register { reg_num: 3 }),
                    operand2: Some(Token::Register { reg_num: 4 }),
                    operand3: Some(Token::Register { reg_num: 5 }),
                    ..Default::default()
                }
            ))
        );
    }
}
