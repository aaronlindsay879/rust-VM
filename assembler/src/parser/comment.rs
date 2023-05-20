use nom::bytes::complete::take_while;
use nom::character::complete::{char, multispace0};
use nom::combinator::{map, opt};
use nom::sequence::tuple;
use nom::IResult;

/// Matches a comment of the form `; <text>`
pub(super) fn parse_comment(input: &str) -> IResult<&str, ()> {
    map(
        opt(tuple((multispace0, char(';'), take_while(|c| c != '\n')))),
        |_| (),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comment_parser() {
        assert_eq!(parse_comment("; aaaaaaa"), Ok(("", ())));
        assert_eq!(parse_comment("    ; aaaaaaa"), Ok(("", ())));
        assert_eq!(parse_comment("     aaaaaaa"), Ok(("     aaaaaaa", ())));
    }
}
