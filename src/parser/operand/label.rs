use nom::character::complete::{alphanumeric1, char};
use nom::sequence::preceded;
use nom::IResult;

/// Parses a label usage of the form @<string>, where string consists of alphanumeric characters
pub(super) fn parse_label_usage(input: &str) -> IResult<&str, &str> {
    preceded(char('@'), alphanumeric1)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_label_usage() {
        assert_eq!(parse_label_usage("@test"), Ok(("", "test")));
        assert_eq!(parse_label_usage("@aaa"), Ok(("", "aaa")));
        assert_eq!(parse_label_usage("@100"), Ok(("", "100")));
        assert_eq!(parse_label_usage("@100@"), Ok(("@", "100")));

        assert!(parse_label_usage("test").is_err());
    }
}
