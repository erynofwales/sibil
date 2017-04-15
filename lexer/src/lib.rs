extern crate sibiltypes;

mod char;
mod charset;
mod lexer;
mod named_char;
mod number;
mod str;
mod token;

pub use lexer::Lexer;
pub use token::Token;

pub fn lex(input: &str) -> Lexer {
    Lexer::new(&input)
}

#[cfg(test)]
mod tests {
    use sibiltypes::{Bool, Char, Number};
    use std::iter::Iterator;
    use super::lex;
    use lexer::Lexer;
    use token::Token;

    #[test]
    fn finds_parens() {
        check_single_token("(", Token::LeftParen);
        check_single_token(")", Token::RightParen);
        check_single_token("#(", Token::LeftVectorParen);
    }

    #[test]
    fn finds_characters() {
        check_single_token("#\\a", Token::Character(Char('a')));
        check_single_token("#\\n", Token::Character(Char('n')));
        check_single_token("#\\s", Token::Character(Char('s')));
    }

    #[test]
    fn finds_named_characters() {
        check_single_token("#\\newline", Token::Character(Char('\n')));
        check_single_token("#\\null", Token::Character(Char('\0')));
        check_single_token("#\\space", Token::Character(Char(' ')));
    }

    #[test]
    fn finds_dots() {
        check_single_token(".", Token::Dot);

        let mut lexer = Lexer::new("abc . abc");
        assert_next_token(&mut lexer, &Token::Id(String::from("abc")));
        assert_next_token(&mut lexer, &Token::Dot);
        assert_next_token(&mut lexer, &Token::Id(String::from("abc")));
    }

    #[test]
    fn finds_identifiers() {
        let tok = |s: &str| { check_single_token(s, Token::Id(String::from(s))); };
        tok("abc");
        tok("number?");
        tok("+");
        tok("-");
    }

    #[test]
    fn finds_booleans() {
        check_single_token("#t", Token::Boolean(Bool(true)));
        check_single_token("#f", Token::Boolean(Bool(false)));
    }

    #[test]
    fn finds_comments() {
        let s = "; a comment";
        check_single_token(s, Token::Comment(String::from(s)));
    }

    #[test]
    fn finds_escaped_characters_in_strings() {
        check_single_token("\"\\\\\"", Token::String(String::from("\\")));
        check_single_token("\"\\\"\"", Token::String(String::from("\"")));
        check_single_token("\"\\n\"", Token::String(String::from("\n")));
    }

    #[test]
    fn finds_numbers() {
        check_single_token("34", Token::Number(Number::from_int(34, true)));
        check_single_token(".34", Token::Number(Number::from_float(0.34, false)));
        check_single_token("0.34", Token::Number(Number::from_float(0.34, false)));
    }

    #[test]
    fn finds_rational_numbers() {
        check_single_token("3/2", Token::Number(Number::from_quotient(3, 2, true)));
        check_single_token("-3/2", Token::Number(Number::from_quotient(-3, 2, true)));
    }

    #[test]
    fn finds_negative_numbers() {
        check_single_token("-3", Token::Number(Number::from_int(-3, true)));
        check_single_token("-0", Token::Number(Number::from_int(-0, true)));
        check_single_token("-0.56", Token::Number(Number::from_float(-0.56, false)));
        check_single_token("-3.14159", Token::Number(Number::from_float(-3.14159, false)));
    }

    #[test]
    fn finds_bin_numbers() {
        check_single_token("#b0", Token::Number(Number::from_int(0b0, true)));
        check_single_token("#b01011", Token::Number(Number::from_int(0b01011, true)));
    }

    #[test]
    fn finds_dec_numbers() {
        check_single_token("34", Token::Number(Number::from_int(34, true)));
        check_single_token("#d89", Token::Number(Number::from_int(89, true)));
    }

    #[test]
    fn finds_oct_numbers() {
        check_single_token("#o45", Token::Number(Number::from_int(0o45, true)));
    }

    #[test]
    fn finds_exact_numbers() {
        check_single_token("#e45", Token::Number(Number::from_int(45, true)));
        check_single_token("#e-45", Token::Number(Number::from_int(-45, true)));
        check_single_token("#e4.5", Token::Number(Number::from_float(4.5, true)));
    }

    #[test]
    fn finds_hex_numbers() {
        check_single_token("#h4A65", Token::Number(Number::from_int(0x4A65, true)));
    }

    #[test]
    fn finds_quote() {
        check_single_token("'", Token::Quote);
    }

    #[test]
    fn finds_strings() {
        check_single_token("\"\"", Token::String(String::from("")));
        check_single_token("\"abc\"", Token::String(String::from("abc")));
    }

    #[test]
    fn lexes_simple_expression() {
        check_tokens("(+ 3.4 6.8)", vec![
                     Token::LeftParen,
                     Token::Id(String::from("+")),
                     Token::Number(Number::from_float(3.4, false)),
                     Token::Number(Number::from_float(6.8, false)),
                     Token::RightParen]);
    }

    #[test]
    fn lexes_quoted_identifier() {
        check_tokens("'abc", vec![Token::Quote, Token::Id(String::from("abc"))]);
    }

    fn check_single_token(input: &str, expected: Token) {
        let mut lexer = Lexer::new(input);
        assert_next_token(&mut lexer, &expected);
    }

    fn check_tokens(input: &str, expected: Vec<Token>) {
        let lexer = lex(input);
        let mut expected_iter = expected.iter();
        for lex in lexer {
            if let Some(expected_token) = expected_iter.next() {
                assert_eq!(lex.token, *expected_token);
            }
            else {
                assert!(false, "Found a token we didn't expect: {:?}", lex.token);
            }
        }
        // TODO: Check that all expected tokens are consumed.
    }

    fn assert_next_token(lexer: &mut Lexer, expected: &Token) {
        let lex = lexer.next().unwrap();
        assert_eq!(lex.token, *expected);
    }
}
