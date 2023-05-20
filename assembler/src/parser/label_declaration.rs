use nom::character::complete::{alphanumeric1, char};
use nom::sequence::terminated;
use nom::IResult;

/// Parses a label declaration of the form <label>:
pub(super) fn parse_label_declaration(input: &str) -> IResult<&str, &str> {
    terminated(alphanumeric1, char(':'))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_label_declaration() {
        assert_eq!(parse_label_declaration("test:"), Ok(("", "test")));
        assert_eq!(parse_label_declaration("aaa:"), Ok(("", "aaa")));
        assert_eq!(parse_label_declaration("100:"), Ok(("", "100")));

        assert!(parse_label_declaration(":100:").is_err());
        assert!(parse_label_declaration("test").is_err());
    }
}
