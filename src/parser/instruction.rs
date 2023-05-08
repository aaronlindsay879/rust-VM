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
pub enum AssemblerInstruction {
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

    pub fn size(&self) -> usize {
        match self {
            AssemblerInstruction::Opcode(_) => 4,
            AssemblerInstruction::Directive(directive) => directive.size(),
        }
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
pub struct OpcodeInstruction {
    pub label: Option<String>,
    pub opcode: Opcode,
    pub operands: Vec<Operand>,
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
pub struct DirectiveInstruction {
    pub label: Option<String>,
    pub directive: String,
    pub operands: Vec<Operand>,
}

impl DirectiveInstruction {
    /// Size of 4-byte aligned null terminated string
    pub(crate) fn size(&self) -> usize {
        match &self.directive[..] {
            "asciiz" => self
                .operands
                .first()
                .and_then(|operand| {
                    if let Operand::String(string) = operand {
                        let len = string.len() + 1;
                        let len = ((len + 3) / 4) * 4;

                        Some(len)
                    } else {
                        None
                    }
                })
                .unwrap_or(0),
            _ => 0,
        }
    }

    /// Creates a null terminated string, aligned to 4 byte boundaries
    pub(crate) fn aligned_null_string(&self) -> Option<String> {
        let size = self.size();

        self.operands.first().and_then(|operand| {
            if let Operand::String(string) = operand {
                // null terminate
                let mut string = string.clone();

                // pad to 4 byte increment
                while string.len() < size {
                    string.push('\0');
                }

                Some(string)
            } else {
                None
            }
        })
    }
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

    #[test]
    fn test_string_alignment() {
        assert_eq!(
            DirectiveInstruction {
                label: None,
                directive: "asciiz".to_string(),
                operands: vec![Operand::String("hi".to_owned())],
            }
            .aligned_null_string(),
            Some("hi\0\0".to_owned())
        );

        assert_eq!(
            DirectiveInstruction {
                label: None,
                directive: "asciiz".to_string(),
                operands: vec![Operand::String("hey".to_owned())],
            }
            .aligned_null_string(),
            Some("hey\0".to_owned())
        );

        assert_eq!(
            DirectiveInstruction {
                label: None,
                directive: "asciiz".to_string(),
                operands: vec![Operand::String("hiii".to_owned())],
            }
            .aligned_null_string(),
            Some("hiii\0\0\0\0".to_owned())
        );
    }
}
