use super::Token;
use nom::types::CompleteStr;
#[allow(unused_imports)]
use nom::{digit, do_parse, named, tag, ws};

named!(
    pub register<CompleteStr, Token>,
    ws!(
        do_parse!(
            tag!("$") >>
            reg_num: digit >>
            ( Token::Register { reg_num: reg_num.parse().unwrap() } )
        )
    )
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_register() {
        let result = register(CompleteStr("$0"));
        assert_eq!(result.is_ok(), true);
        let (rest, token) = result.unwrap();
        assert_eq!(token, Token::Register { reg_num: 0 });
        assert_eq!(rest, CompleteStr(""));

        let result = register(CompleteStr("0"));
        assert_eq!(result.is_ok(), false);

        let result = register(CompleteStr("$a"));
        assert_eq!(result.is_ok(), false);
    }
}
