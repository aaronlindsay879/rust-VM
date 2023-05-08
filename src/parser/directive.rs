use nom::character::complete::{alpha1, char};
use nom::combinator::map;
use nom::sequence::preceded;
use nom::IResult;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Directive {
    Ascii,
    Asciiz,
    Code,
    Data,
    Unknown,
}

impl From<&str> for Directive {
    fn from(value: &str) -> Self {
        match &value.to_lowercase()[..] {
            "ascii" => Self::Ascii,
            "asciiz" => Self::Asciiz,
            "code" => Self::Code,
            "data" => Self::Data,
            _ => Self::Unknown,
        }
    }
}

/// Parses a directive of the form .<directive>
pub(super) fn parse_directive(input: &str) -> IResult<&str, Directive> {
    map(preceded(char('.'), alpha1), Directive::from)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_directive() {
        assert_eq!(parse_directive(".asciiz"), Ok(("", Directive::Asciiz)));
        assert_eq!(parse_directive(".code.a"), Ok((".a", Directive::Code)));
        assert_eq!(
            parse_directive(".one@two"),
            Ok(("@two", Directive::Unknown))
        );

        assert!(parse_directive("asciiz").is_err());
    }
}
