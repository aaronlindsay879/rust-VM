use crate::opcode::Opcode;
use crate::parser::directive::parse_directive;
use crate::parser::label_declaration::parse_label_declaration;
use crate::parser::opcode::parse_opcode;
use crate::parser::operand::{parse_operand, Operand};
use nom::branch::alt;
use nom::character::complete::{char, multispace0};
use nom::combinator::{map, opt};
use nom::multi::many0;
use nom::sequence::{delimited, tuple};
use nom::IResult;

#[derive(PartialEq, Debug, Clone)]
pub(crate) enum AssemblerInstruction {
    Opcode(OpcodeInstruction),
    Directive(DirectiveInstruction),
}

impl AssemblerInstruction {
    pub fn new_opcode(label: Option<&str>, opcode: Opcode, operands: &[Operand]) -> Self {
        Self::Opcode(OpcodeInstruction {
            label: label.map(str::to_owned),
            opcode,
            operands: operands.to_vec(),
        })
    }

    pub fn new_directive(label: Option<&str>, directive: &str, operands: &[Operand]) -> Self {
        Self::Directive(DirectiveInstruction {
            label: label.map(str::to_owned),
            directive: directive.to_owned(),
            operands: operands.to_vec(),
        })
    }
}

/// Parses an instruction of the form <label?> <opcode | directive> <operands?>
pub(super) fn parse_instruction(input: &str) -> IResult<&str, AssemblerInstruction> {
    alt((
        map(parse_opcode_instruction, AssemblerInstruction::Opcode),
        map(parse_directive_instruction, AssemblerInstruction::Directive),
    ))(input)
}

#[derive(PartialEq, Debug, Clone)]
pub(crate) struct OpcodeInstruction {
    pub(crate) label: Option<String>,
    pub(crate) opcode: Opcode,
    pub(crate) operands: Vec<Operand>,
}

/// Parses an instruction of the form <label?> <opcode> <operands?>
fn parse_opcode_instruction(input: &str) -> IResult<&str, OpcodeInstruction> {
    map(
        tuple((
            opt(parse_label_declaration),
            multispace0,
            parse_opcode,
            many0(delimited(multispace0, parse_operand, opt(char(',')))),
        )),
        |(label, _, opcode, operands)| OpcodeInstruction {
            label: label.map(str::to_owned),
            opcode,
            operands,
        },
    )(input)
}

#[derive(PartialEq, Debug, Clone)]
pub(crate) struct DirectiveInstruction {
    pub(crate) label: Option<String>,
    pub(crate) directive: String,
    pub(crate) operands: Vec<Operand>,
}

/// Parses an instruction of the form <label?> <directive> <operands?>
fn parse_directive_instruction(input: &str) -> IResult<&str, DirectiveInstruction> {
    map(
        tuple((
            opt(parse_label_declaration),
            multispace0,
            parse_directive,
            many0(delimited(multispace0, parse_operand, opt(char(',')))),
        )),
        |(label, _, directive, operands)| DirectiveInstruction {
            label: label.map(str::to_owned),
            directive: directive.to_owned(),
            operands,
        },
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_instruction() {
        assert_eq!(
            parse_instruction("label: ADD 1, $4, $0"),
            Ok((
                "",
                AssemblerInstruction::Opcode(OpcodeInstruction {
                    label: Some("label".into()),
                    opcode: Opcode::ADD,
                    operands: vec![
                        Operand::Value(1),
                        Operand::Register(4),
                        Operand::Register(0)
                    ],
                })
            ))
        );

        assert_eq!(
            parse_instruction("text: .asciiz 'hi'"),
            Ok((
                "",
                AssemblerInstruction::Directive(DirectiveInstruction {
                    label: Some("text".into()),
                    directive: "asciiz".into(),
                    operands: vec![Operand::String("hi".into())],
                })
            ))
        );
    }

    #[test]
    fn test_parse_opcode_instruction() {
        assert_eq!(
            parse_opcode_instruction("HLT"),
            Ok((
                "",
                OpcodeInstruction {
                    label: None,
                    opcode: Opcode::HLT,
                    operands: vec![],
                }
            ))
        );

        assert_eq!(
            parse_opcode_instruction("JMP $0"),
            Ok((
                "",
                OpcodeInstruction {
                    label: None,
                    opcode: Opcode::JMP,
                    operands: vec![Operand::Register(0)],
                }
            ))
        );

        assert_eq!(
            parse_opcode_instruction("JMP   @label"),
            Ok((
                "",
                OpcodeInstruction {
                    label: None,
                    opcode: Opcode::JMP,
                    operands: vec![Operand::Label("label".into())],
                }
            ))
        );

        assert_eq!(
            parse_opcode_instruction("ADD 1,$0,$0"),
            Ok((
                "",
                OpcodeInstruction {
                    label: None,
                    opcode: Opcode::ADD,
                    operands: vec![
                        Operand::Value(1),
                        Operand::Register(0),
                        Operand::Register(0)
                    ],
                }
            ))
        );

        assert_eq!(
            parse_opcode_instruction("label: ADD 1, $4, $0"),
            Ok((
                "",
                OpcodeInstruction {
                    label: Some("label".into()),
                    opcode: Opcode::ADD,
                    operands: vec![
                        Operand::Value(1),
                        Operand::Register(4),
                        Operand::Register(0)
                    ],
                }
            ))
        );
    }
    #[test]
    fn test_parse_directive_instruction() {
        assert_eq!(
            parse_directive_instruction(".asciiz"),
            Ok((
                "",
                DirectiveInstruction {
                    label: None,
                    directive: "asciiz".into(),
                    operands: vec![],
                }
            ))
        );

        assert_eq!(
            parse_directive_instruction(".asciiz 'hi'"),
            Ok((
                "",
                DirectiveInstruction {
                    label: None,
                    directive: "asciiz".into(),
                    operands: vec![Operand::String("hi".into())],
                }
            ))
        );

        assert_eq!(
            parse_directive_instruction("text: .asciiz 'hi'"),
            Ok((
                "",
                DirectiveInstruction {
                    label: Some("text".into()),
                    directive: "asciiz".into(),
                    operands: vec![Operand::String("hi".into())],
                }
            ))
        );
    }
}
