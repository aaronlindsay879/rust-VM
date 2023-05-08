use nom::character::complete::{alpha1, char};
use nom::sequence::preceded;
use nom::IResult;

/// Parses a directive of the form .<directive>
pub(super) fn parse_directive(input: &str) -> IResult<&str, &str> {
    preceded(char('.'), alpha1)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_directive() {
        assert_eq!(parse_directive(".asciiz"), Ok(("", "asciiz")));
        assert_eq!(parse_directive(".test"), Ok(("", "test")));
        assert_eq!(parse_directive(".one@two"), Ok(("@two", "one")));

        assert!(parse_directive("asciiz").is_err());
    }
}
