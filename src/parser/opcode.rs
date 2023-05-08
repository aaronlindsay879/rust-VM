use crate::opcode::Opcode;
use nom::character::complete::alpha1;
use nom::combinator::map;
use nom::IResult;

/// Parses an opcode, such as LOAD
pub(super) fn parse_opcode(input: &str) -> IResult<&str, Opcode> {
    map(alpha1, Opcode::from)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_opcode() {
        assert_eq!(parse_opcode("load"), Ok(("", Opcode::LOAD)));
        assert_eq!(parse_opcode("lOaD"), Ok(("", Opcode::LOAD)));
        assert_eq!(parse_opcode("hlt"), Ok(("", Opcode::HLT)));

        assert_eq!(parse_opcode("unknown"), Ok(("", Opcode::IGL)));
    }
}
