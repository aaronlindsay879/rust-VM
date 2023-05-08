use crate::opcode::Opcode;
use crate::parser::directive::{parse_directive, Directive};
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
    /// Used in testing
    #[allow(unused)]
    pub fn new_opcode(label: Option<&str>, opcode: Opcode, operands: &[Operand]) -> Self {
        Self::Opcode(OpcodeInstruction {
            label: label.map(str::to_owned),
            opcode,
            operands: operands.to_vec(),
        })
    }

    /// Used in testing
    #[allow(unused)]
    pub fn new_directive(label: Option<&str>, directive: Directive, operands: &[Operand]) -> Self {
        Self::Directive(DirectiveInstruction {
            label: label.map(str::to_owned),
            directive,
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
    pub directive: Directive,
    pub operands: Vec<Operand>,
}

impl DirectiveInstruction {
    /// Size of 4-byte aligned null terminated string. If alignment is None, default to 4 bytes.
    pub(crate) fn size(&self, alignment: Option<usize>) -> usize {
        let alignment = alignment.unwrap_or(4);

        match self.directive {
            Directive::Align => 0,
            Directive::Ascii => self
                .operands
                .first()
                .and_then(|operand| {
                    if let Operand::String(string) = operand {
                        let len = string.len();
                        let len = Self::align(len, alignment);

                        Some(len)
                    } else {
                        None
                    }
                })
                .unwrap_or(0),
            Directive::Asciiz => self
                .operands
                .first()
                .and_then(|operand| {
                    if let Operand::String(string) = operand {
                        let len = string.len() + 1;
                        let len = Self::align(len, alignment);

                        Some(len)
                    } else {
                        None
                    }
                })
                .unwrap_or(0),
            Directive::Byte => {
                let count = self
                    .operands
                    .iter()
                    .filter(|operand| matches!(operand, Operand::Value { .. }))
                    .count();

                Self::align(count, alignment)
            }
            Directive::Half => {
                let count = self
                    .operands
                    .iter()
                    .filter(|operand| matches!(operand, Operand::Value { .. }))
                    .count();

                Self::align(count * 2, alignment)
            }
            Directive::Word => {
                let count = self
                    .operands
                    .iter()
                    .filter(|operand| matches!(operand, Operand::Value { .. }))
                    .count();

                Self::align(count * 4, alignment)
            }
            Directive::Space => self
                .operands
                .first()
                .and_then(|operand| {
                    if let Operand::Value(value) = operand {
                        Some(*value as usize)
                    } else {
                        None
                    }
                })
                .unwrap_or(0),
            _ => 0,
        }
    }

    /// Creates a null terminated string. If alignment is None, default to 4 bytes.
    pub(crate) fn aligned_bytes(&self, alignment: Option<usize>) -> Option<Vec<u8>> {
        let size = self.size(alignment);

        let mut bytes = match self.directive {
            Directive::Ascii => match self.operands.first() {
                Some(Operand::String(string)) => string.as_bytes().to_vec(),
                _ => vec![],
            },
            Directive::Asciiz => match self.operands.first() {
                Some(Operand::String(string)) => {
                    let mut string = string.clone();
                    string.push('\0');

                    string.as_bytes().to_vec()
                }
                _ => vec![],
            },
            Directive::Byte => self
                .operands
                .iter()
                .filter_map(|operand| {
                    if let &Operand::Value(value) = operand {
                        Some(value as u8)
                    } else {
                        None
                    }
                })
                .collect(),
            Directive::Half => self
                .operands
                .iter()
                .filter_map(|operand| {
                    if let &Operand::Value(value) = operand {
                        Some((value as u16).to_be_bytes())
                    } else {
                        None
                    }
                })
                .flatten()
                .collect(),
            Directive::Word => self
                .operands
                .iter()
                .filter_map(|operand| {
                    if let &Operand::Value(value) = operand {
                        Some((value as u32).to_be_bytes())
                    } else {
                        None
                    }
                })
                .flatten()
                .collect(),
            _ => vec![],
        };

        if bytes.len() < size {
            bytes.resize(size, 0);
        }

        Some(bytes)
    }

    fn align(value: usize, alignment: usize) -> usize {
        ((value + alignment - 1) / alignment) * alignment
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
    use nom::AsBytes;

    #[test]
    fn test_parse_instruction() {
        assert_eq!(
            parse_instruction("label: LBI 1, $4, $0"),
            Ok((
                "",
                AssemblerInstruction::Opcode(OpcodeInstruction {
                    label: Some("label".into()),
                    opcode: Opcode::LBI,
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
            parse_opcode_instruction("LBI $0"),
            Ok((
                "",
                OpcodeInstruction {
                    label: None,
                    opcode: Opcode::LBI,
                    operands: vec![Operand::Register(0)],
                }
            ))
        );

        assert_eq!(
            parse_opcode_instruction("LBI   @label"),
            Ok((
                "",
                OpcodeInstruction {
                    label: None,
                    opcode: Opcode::LBI,
                    operands: vec![Operand::Label("label".into())],
                }
            ))
        );

        assert_eq!(
            parse_opcode_instruction("LBI 1,$0,$0"),
            Ok((
                "",
                OpcodeInstruction {
                    label: None,
                    opcode: Opcode::LBI,
                    operands: vec![
                        Operand::Value(1),
                        Operand::Register(0),
                        Operand::Register(0)
                    ],
                }
            ))
        );

        assert_eq!(
            parse_opcode_instruction("label: LBI 1, $4, $0"),
            Ok((
                "",
                OpcodeInstruction {
                    label: Some("label".into()),
                    opcode: Opcode::LBI,
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
                directive: Directive::Asciiz,
                operands: vec![Operand::String("hi".to_owned())],
            }
            .aligned_bytes(None),
            Some("hi\0\0".as_bytes().to_vec())
        );

        assert_eq!(
            DirectiveInstruction {
                label: None,
                directive: Directive::Asciiz,
                operands: vec![Operand::String("hey".to_owned())],
            }
            .aligned_bytes(None),
            Some("hey\0".as_bytes().to_vec())
        );

        assert_eq!(
            DirectiveInstruction {
                label: None,
                directive: Directive::Asciiz,
                operands: vec![Operand::String("hiii".to_owned())],
            }
            .aligned_bytes(None),
            Some("hiii\0\0\0\0".as_bytes().to_vec())
        );
    }
}
