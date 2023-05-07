use super::Token;
use crate::opcode::Opcode;
use nom::types::CompleteStr;
use nom::{do_parse, named, tag_no_case};

named!(
    pub opcode_load<CompleteStr, Token>,
    do_parse!(
        tag_no_case!("load") >> (Token::Op { code: Opcode::LOAD })
    )
);

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_opcode_load() {
        let result = opcode_load(CompleteStr("load"));
        assert_eq!(result.is_ok(), true);
        let (rest, token) = result.unwrap();
        assert_eq!(token, Token::Op { code: Opcode::LOAD });
        assert_eq!(rest, CompleteStr(""));

        let result = opcode_load(CompleteStr("aold"));
        assert_eq!(result.is_ok(), false);
    }
}
