use super::{label_parsers::label_usage, register_parsers::register, Token};
use nom::types::CompleteStr;
#[allow(unused_imports)]
use nom::{alt, digit, do_parse, named, tag, take_until, ws};

named!(
    integer_operand<CompleteStr, Token>,
    ws!(
        do_parse!(
            tag!("#") >>
            value: digit >>
            ( Token::IntegerOperand { value: value.parse().unwrap() } )
        )
    )
);

named!(irstring<CompleteStr, Token>,
    do_parse!(
        tag!("'") >>
        content: take_until!("'") >>
        tag!("'") >>
        ( Token::IrString { name: content.to_string() } )
    )
);

named!(pub operand<CompleteStr, Token>,
    alt!(
        integer_operand |
        label_usage |
        register |
        irstring
    )
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_integer_operand() {
        let result = integer_operand(CompleteStr("#10"));
        assert_eq!(result.is_ok(), true);
        let (rest, value) = result.unwrap();
        assert_eq!(rest, CompleteStr(""));
        assert_eq!(value, Token::IntegerOperand { value: 10 });

        let result = integer_operand(CompleteStr("10"));
        assert_eq!(result.is_ok(), false);
    }

    #[test]
    fn test_parse_string_operand() {
        let result = irstring(CompleteStr("'This is a test'"));
        assert_eq!(result.is_ok(), true);
    }
}
