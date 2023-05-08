use crate::parser::parse_number;
use nom::character::complete::char;
use nom::combinator::map;
use nom::sequence::preceded;
use nom::IResult;

/// Parses a register of the form $<number>
pub(super) fn parse_register(input: &str) -> IResult<&str, u8> {
    map(preceded(char('$'), parse_number), |number| number as u8)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_register() {
        assert_eq!(parse_register("$4"), Ok(("", 4)));
        assert_eq!(parse_register("$0xA"), Ok(("", 0xA)));
        assert_eq!(parse_register("$0b101"), Ok(("", 0b101)));

        assert_eq!(parse_register("$4a4"), Ok(("a4", 4)));
        assert!(parse_register("4a4").is_err());
    }
}
