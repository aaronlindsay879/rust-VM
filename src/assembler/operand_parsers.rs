use super::{label_parsers::label_usage, register_parsers::register, Token};
use nom::types::CompleteStr;
#[allow(unused_imports)]
use nom::{alt, digit, do_parse, named, tag, ws};

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

named!(pub operand<CompleteStr, Token>,
    alt!(
        integer_operand |
        label_usage |
        register
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
}
