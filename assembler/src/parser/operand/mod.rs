use crate::parser::operand::label::parse_label_usage;
use crate::parser::operand::register::parse_register;
use crate::parser::operand::string::parse_string;
use crate::parser::parse_number;
use nom::branch::alt;
use nom::combinator::map;
use nom::IResult;

mod label;
mod register;
mod string;

#[derive(PartialEq, Debug, Clone)]
pub enum Operand {
    Register(u8),
    Value(i32),
    Label(String),
    String(String),
}

/// Parses an operand which can either be a register, value, or label usage
pub(super) fn parse_operand(input: &str) -> IResult<&str, Operand> {
    alt((
        map(parse_register, Operand::Register),
        map(parse_number, Operand::Value),
        map(parse_label_usage, |label| Operand::Label(label.to_owned())),
        map(parse_string, |string| Operand::String(string.to_owned())),
    ))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_operand() {
        assert_eq!(parse_operand("$100"), Ok(("", Operand::Register(100))));
        assert_eq!(parse_operand("100"), Ok(("", Operand::Value(100))));
        assert_eq!(
            parse_operand("@test"),
            Ok(("", Operand::Label("test".to_owned())))
        );
        assert_eq!(
            parse_operand("'hi'"),
            Ok(("", Operand::String("hi".to_owned())))
        );

        assert!(parse_operand("@[]").is_err());
        assert!(parse_operand("test").is_err());
    }
}
