use nom::branch::alt;
use nom::bytes::complete::take_until;
use nom::character::complete::char;
use nom::combinator::map;
use nom::sequence::tuple;
use nom::IResult;

/// Parses a string of the form "<string>" or '<string>'
pub(super) fn parse_string(input: &str) -> IResult<&str, &str> {
    map(
        alt((
            tuple((char('\''), take_until("'"), char('\''))),
            tuple((char('\"'), take_until("\""), char('\"'))),
        )),
        |(_, string, _)| string,
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_string() {
        assert_eq!(parse_string("'hi'"), Ok(("", "hi")));
        assert_eq!(parse_string(r#""test""#), Ok(("", "test")));

        assert!(parse_string(r#"'no worky""#).is_err());
        assert!(parse_string("test").is_err());
    }
}
