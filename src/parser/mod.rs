pub mod directive;
pub mod instruction;
mod label_declaration;
mod opcode;
pub mod operand;

use crate::parser::instruction::parse_instruction;
use instruction::AssemblerInstruction;
use nom::branch::alt;
use nom::bytes::complete::{is_a, tag, take_while};
use nom::character::complete::{digit1, hex_digit1, multispace0};
use nom::combinator::{map_res, opt};
use nom::multi::many0;
use nom::sequence::{delimited, pair, separated_pair};
use nom::IResult;

#[derive(Debug)]
pub struct Program {
    pub instructions: Vec<AssemblerInstruction>,
}

impl Program {
    pub fn parse(text: &str) -> Option<Self> {
        let (_, instructions) =
            many0(delimited(multispace0, parse_instruction, multispace0))(text).ok()?;

        Some(Self { instructions })
    }
}

/// Parses a signed integer that can be decimal, hexadecimal (with 0x prefix) or binary (with 0b prefix)
fn parse_number(input: &str) -> IResult<&str, i32> {
    #[rustfmt::skip]
    fn hexadecimal(hex: &str) -> IResult<&str, i32> {
        map_res(
            separated_pair(
                opt(is_a("+-")),
                alt((tag("0x"), tag("0X"))),
                hex_digit1
            ),
            |(sign, number): (Option<&str>, &str)| {
                let string = match sign {
                    Some(sign) => format!("{sign}{number}"),
                    None => number.to_owned(),
                };

                i32::from_str_radix(&string, 16)
            },
        )(hex)
    }

    #[rustfmt::skip]
    fn binary(bin: &str) -> IResult<&str, i32> {
        map_res(
            separated_pair(
                opt(is_a("+-")),
                alt((tag("0b"), tag("0b"))),
                take_while(|c| c == '0' || c == '1')
            ),
            |(sign, number): (Option<&str>, &str)| {
                let string = match sign {
                    Some(sign) => format!("{sign}{number}"),
                    None => number.to_owned(),
                };

                i32::from_str_radix(&string, 2)
            },
        )(bin)
    }

    fn decimal(dec: &str) -> IResult<&str, i32> {
        map_res(
            pair(opt(is_a("+-")), digit1),
            |(sign, number): (Option<&str>, &str)| {
                let string = match sign {
                    Some(sign) => format!("{sign}{number}"),
                    None => number.to_owned(),
                };

                string.parse()
            },
        )(dec)
    }

    alt((hexadecimal, binary, decimal))(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instruction::Instruction;
    use crate::opcode::Opcode;
    use crate::parser::directive::Directive;
    use crate::parser::instruction::DirectiveInstruction;
    use crate::parser::operand::Operand;
    use crate::parser::operand::Operand::String;

    #[test]
    fn test_parse_number() {
        assert_eq!(parse_number("10"), Ok(("", 10)));
        assert_eq!(parse_number("-564"), Ok(("", -564)));
        assert_eq!(parse_number("-5A64"), Ok(("A64", -5)));

        assert_eq!(parse_number("0x1A"), Ok(("", 0x1A)));
        assert_eq!(parse_number("-0x5F4"), Ok(("", -0x5F4)));
        assert_eq!(parse_number("0xG"), Ok(("xG", 0)));

        assert_eq!(parse_number("0b10"), Ok(("", 0b10)));
        assert_eq!(parse_number("-0b101"), Ok(("", -0b101)));
        assert_eq!(parse_number("0b2"), Ok(("b2", 0)));

        assert!(parse_number("hello").is_err());
    }

    #[test]
    fn test_parse_program() {
        let program = r#".data
                                    hello: .asciiz 'Hello'
                                    world: .asciiz 'world!'
                                .code
                                loop:
                                    lbi 2,$0,$0
                                    lbi @loop"#;

        let program = Program::parse(&program).unwrap();

        assert_eq!(
            program.instructions,
            vec![
                AssemblerInstruction::new_directive(None, Directive::Data, &[]),
                AssemblerInstruction::new_directive(
                    Some("hello"),
                    Directive::Asciiz,
                    &[String("Hello".to_owned())]
                ),
                AssemblerInstruction::new_directive(
                    Some("world"),
                    Directive::Asciiz,
                    &[String("world!".to_owned())]
                ),
                AssemblerInstruction::new_directive(None, Directive::Code, &[]),
                AssemblerInstruction::new_opcode(
                    Some("loop"),
                    Opcode::LBI,
                    &[
                        Operand::Value(2),
                        Operand::Register(0),
                        Operand::Register(0)
                    ]
                ),
                AssemblerInstruction::new_opcode(
                    None,
                    Opcode::LBI,
                    &[Operand::Label("loop".to_owned())]
                )
            ]
        )
    }
}
