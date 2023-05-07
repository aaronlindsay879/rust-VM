use super::{opcode_parsers::*, operand_parsers::*, register_parsers::register, Token};
use nom::types::CompleteStr;
use nom::{alt, do_parse, many_m_n, multispace, named, opt};

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

        if out.len() < 4 {
            out.resize(4, 0);
        }

        Ok(out)
    }
}

// instruction of form [opcode] [register] [integer]
named!(
    instruction_one<CompleteStr, AssemblerInstruction>,
    do_parse!(
        o: opcode >>
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

// instruction of form [opcode] [register]? [register]? [register]?
named!(
    instruction_two<CompleteStr, AssemblerInstruction>,
    do_parse!(
        o: opcode >>
        registers: many_m_n!(0, 3, register) >>
        opt!(multispace) >>
        (
            match registers.len() {
                1 => AssemblerInstruction {
                    opcode: o,
                    operand1: Some(registers[0]),
                    operand2: None,
                    operand3: None,
                },
                2 => AssemblerInstruction {
                    opcode: o,
                    operand1: Some(registers[0]),
                    operand2: Some(registers[1]),
                    operand3: None,
                },
                3 => AssemblerInstruction {
                    opcode: o,
                    operand1: Some(registers[0]),
                    operand2: Some(registers[1]),
                    operand3: Some(registers[2]),
                },
                _ => AssemblerInstruction {
                    opcode: o,
                    operand1: None,
                    operand2: None,
                    operand3: None,
                }
            }
        )
    )
);

named!(pub instruction<CompleteStr, AssemblerInstruction>,
    do_parse!(
        ins: alt!(
            instruction_one |
            instruction_two
        ) >>
        (
            ins
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

    #[test]
    fn test_parse_instruction_form_two_0() {
        let result = instruction_two(CompleteStr("hlt\n"));
        assert_eq!(
            result,
            Ok((
                CompleteStr(""),
                AssemblerInstruction {
                    opcode: Token::Op { code: Opcode::HLT },
                    operand1: None,
                    operand2: None,
                    operand3: None
                }
            ))
        );
    }

    #[test]
    fn test_parse_instruction_form_two_1() {
        let result = instruction_two(CompleteStr("jmpe $4\n"));
        assert_eq!(
            result,
            Ok((
                CompleteStr(""),
                AssemblerInstruction {
                    opcode: Token::Op { code: Opcode::JMPE },
                    operand1: Some(Token::Register { reg_num: 4 }),
                    operand2: None,
                    operand3: None
                }
            ))
        );
    }

    #[test]
    fn test_parse_instruction_form_two_2() {
        let result = instruction_two(CompleteStr("eq $3 $4\n"));
        assert_eq!(
            result,
            Ok((
                CompleteStr(""),
                AssemblerInstruction {
                    opcode: Token::Op { code: Opcode::EQ },
                    operand1: Some(Token::Register { reg_num: 3 }),
                    operand2: Some(Token::Register { reg_num: 4 }),
                    operand3: None
                }
            ))
        );
    }

    #[test]
    fn test_parse_instruction_form_two_3() {
        let result = instruction_two(CompleteStr("add $3 $4 $ 5\n"));
        assert_eq!(
            result,
            Ok((
                CompleteStr(""),
                AssemblerInstruction {
                    opcode: Token::Op { code: Opcode::ADD },
                    operand1: Some(Token::Register { reg_num: 3 }),
                    operand2: Some(Token::Register { reg_num: 4 }),
                    operand3: Some(Token::Register { reg_num: 5 })
                }
            ))
        );
    }
}
