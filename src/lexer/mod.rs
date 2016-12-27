/* lexer.rs
 * Eryn Wells <eryn@erynwells.me>
 */

pub mod token;
mod char;
mod charset;
mod number;
mod str;

use self::char::Lexable;
use self::number::Exactness;
use self::number::NumberBuilder;
use self::number::Radix;
use self::number::Sign;
use self::str::CharAt;
use self::str::RelativeIndexable;
use self::token::Lex;
use self::token::Token;

#[derive(Debug)]
enum State {
    Comment,
    Initial,
    Identifier,
    Dot,
    Hash,
    Number,
    NumberExactness,
    NumberDecimal,
    NumberRadix,
    NumberSign,
    Sign,
    String,
}

pub struct Lexer {
    input: String,
    begin: usize,
    forward: usize,
    line: u32,
    state: State,
    number_builder: NumberBuilder,
}

impl Lexer {
    pub fn new(input: &str) -> Lexer {
        Lexer {
            input: String::from(input),
            begin: 0,
            forward: 0,
            line: 1,
            state: State::Initial,
            number_builder: NumberBuilder::new(),
        }
    }
}

impl Lexer {
    fn begin_lexing(&mut self) {
        self.forward = self.begin;
        self.state = State::Initial;
    }

    /// Advance the forward pointer to the next character.
    fn advance(&mut self) {
        self.forward = self.input.index_after(self.forward);
        println!("> forward={}", self.forward);
    }

    /// Retract the forward pointer to the previous character.
    fn retract(&mut self) {
        self.forward = self.input.index_before(self.forward);
        println!("< forward={}", self.forward);
    }

    /// Advance the begin pointer to prepare for the next iteration.
    fn advance_begin(&mut self) {
        self.begin = self.input.index_after(self.forward);
        self.forward = self.begin;
        println!("> begin={}, forward={}", self.begin, self.forward);
    }

    fn handle_newline(&mut self) {
        self.line += 1;
    }

    /// Get the substring between the two input indexes. This is the value to give to a new Token instance.
    fn value(&self) -> String {
        self.input[self.begin .. self.forward].to_string()
    }
}

impl Lexer {
    // TODO: Use std::result::Result for these state_* methods.
    // https://doc.rust-lang.org/1.14.0/core/result/enum.Result.html

    /// Handle self.state == State::Initial
    fn state_initial(&mut self, c: char, token: &mut Option<Token>) {
        if c.is_left_paren() {
            *token = Some(Token::LeftParen(c.to_string()));
        }
        else if c.is_right_paren() {
            *token = Some(Token::RightParen(c.to_string()));
        }
        else if c.is_dot() {
            self.state = State::Dot;
            self.advance();
        }
        else if c.is_hash() {
            self.state = State::Hash;
            self.advance();
        }
        else if c.is_string_quote() {
            self.state = State::String;
            self.advance();
        }

        else if let Some(sign) = Sign::from_char(c) {
            self.number_builder = NumberBuilder::new();
            self.number_builder.sign(sign);
            self.state = State::Sign;
            self.advance();
        }
        else if c.is_identifier_initial() {
            self.state = State::Identifier;
            self.advance();
        }

        else if c.is_digit(10) {
            self.number_builder = NumberBuilder::new();
            self.number_builder.extend_value(c);
            self.state = State::Number;
            self.advance();
        }

        else if c.is_whitespace() {
            if c.is_newline() {
                self.handle_newline();
            }
            self.advance_begin();
        }

        else if c.is_comment_initial() {
            self.state = State::Comment;
            self.advance();
        }

        else {
            assert!(false, "Invalid token character: {}", c);
        }
    }

    /// Handle self.state == State::Identifier
    fn state_identifier(&mut self, c: char, token: &mut Option<Token>) {
        if c.is_identifier_subsequent() {
            // Stay in Identifier state.
            self.advance();
        }
        else if c.is_identifier_delimiter() {
            *token = Some(Token::Identifier(self.value()));
            self.retract();
        }
        else {
            assert!(false, "Invalid token character: '{}'", c);
        }
    }

    fn state_dot(&mut self, c: char, token: &mut Option<Token>) {
        if c.is_identifier_delimiter() {
            *token = Some(Token::Dot);
            self.retract();
        }
        else if c.is_digit(10) {
            self.number_builder = NumberBuilder::new();
            self.number_builder.extend_decimal_value(c);
            self.state = State::NumberDecimal;
            self.advance();
        }
        else {
            assert!(false, "Invalid token character: '{}'", c);
        }
    }

    fn state_hash(&mut self, c: char, token: &mut Option<Token>) {
        if c.is_boolean_true() || c.is_boolean_false() {
            *token = Some(Token::Boolean(c.is_boolean_true()));
            self.advance();
        }
        else if c.is_left_paren() {
            *token = Some(Token::LeftVectorParen);
            self.advance();
        }
        else if let Some(radix) = Radix::from_char(c) {
            self.number_builder.radix(radix);
            self.state = State::NumberRadix;
            self.advance();
        }
        else if let Some(exactness) = Exactness::from_char(c) {
            self.number_builder.exact(exactness);
            self.state = State::NumberExactness;
            self.advance();
        }
        else {
            assert!(false, "Invalid token character: '{}'", c);
        }
    }

    fn state_number(&mut self, c: char, token: &mut Option<Token>) {
        if c.is_digit(self.number_builder.radix_value()) {
            self.number_builder.extend_value(c);
            self.advance();
        }
        else if c.is_dot() {
            self.state = State::NumberDecimal;
            self.advance();
        }
        else if c.is_identifier_delimiter() {
            *token = Some(Token::Number(self.number_builder.resolve()));
            self.retract();
        }
        else {
            assert!(false, "Invalid token character: '{}'", c);
        }
    }

    fn state_number_exactness(&mut self, c: char, token: &mut Option<Token>) {
        if c.is_hash() {
            self.state = State::Hash;
            self.advance();
        }
        else if c.is_digit(self.number_builder.radix_value()) {
            self.number_builder.extend_value(c);
            self.state = State::Number;
            self.advance();
        }
        else {
            assert!(false, "Invalid token character: '{}'", c);
        }
    }

    fn state_number_decimal(&mut self, c: char, token: &mut Option<Token>) {
        if c.is_digit(Radix::Dec.value()) {
            self.number_builder.extend_decimal_value(c);
            self.advance();
        }
        else if c.is_identifier_delimiter() {
            *token = Some(Token::Number(self.number_builder.resolve()));
            self.retract();
        }
        else {
            assert!(false, "Invalid token character: '{}'", c);
        }
    }

    fn state_number_radix(&mut self, c: char, token: &mut Option<Token>) {
        if c.is_digit(self.number_builder.radix_value()) {
            self.number_builder.extend_value(c);
            self.state = State::Number;
            self.advance();
        }
        else if c.is_dot() {
            self.state = State::NumberDecimal;
            self.advance();
        }
        else if c.is_hash() {
            self.state = State::Hash;
            self.advance();
        }
        else if let Some(sign) = Sign::from_char(c) {
            self.number_builder.sign(sign);
            self.state = State::NumberSign;
            self.advance();
        }
        else {
            assert!(false, "Invalid token character: '{}'", c);
        }
    }

    fn state_number_sign(&mut self, c: char, token: &mut Option<Token>) {
        if c.is_digit(self.number_builder.radix_value()) {
            self.number_builder.extend_value(c);
            self.state = State::Number;
            self.advance();
        }
        else if c.is_dot() {
            self.state = State::NumberDecimal;
            self.advance();
        }
        else {
            assert!(false, "Invalid token character: '{}'", c);
        }
    }

    fn state_sign(&mut self, c: char, token: &mut Option<Token>) {
        if c.is_digit(Radix::Dec.value()) {
            self.number_builder.extend_value(c);
            self.state = State::Number;
            self.advance();
        }
        else if c.is_identifier_delimiter() {
            *token = Some(Token::Identifier(self.value()));
            self.retract();
        }
        else {
            assert!(false, "Invalid token character: '{}'", c);
        }
    }

    fn state_string(&mut self, c: char, token: &mut Option<Token>) {
        self.advance();
        if c.is_string_quote() {
            *token = Some(Token::String(self.value()));
        }
    }

    fn state_comment(&mut self, c: char, token: &mut Option<Token>) {
        if c.is_newline() {
            self.handle_newline();
            *token = Some(Token::Comment(self.value()));
        }
        else if c.is_eof() {
            *token = Some(Token::Comment(self.value()));
        }
        // Consume all characters.
        self.advance();
    }
}

impl Iterator for Lexer {
    type Item = Lex;

    fn next(&mut self) -> Option<Lex> {
        self.begin_lexing();
        if self.begin == self.input.len() {
            return None;
        }
        let mut token: Option<Token> = None;
        println!("Lexing '{}'", &self.input[self.begin ..]);
        while token.is_none() {
            let c = match self.input.char_at(self.forward) {
                Some(c) => c,
                None => '\0',
            };
            println!("{:?}! c='{}'", self.state, c);
            let previous_forward = self.forward;
            match self.state {
                State::Initial => self.state_initial(c, &mut token),
                State::Identifier => self.state_identifier(c, &mut token),
                State::Dot => self.state_dot(c, &mut token),
                State::Hash => self.state_hash(c, &mut token),
                State::Number => self.state_number(c, &mut token),
                State::NumberExactness => self.state_number_exactness(c, &mut token),
                State::NumberDecimal => self.state_number_decimal(c, &mut token),
                State::NumberRadix => self.state_number_radix(c, &mut token),
                State::NumberSign => self.state_number_sign(c, &mut token),
                State::Sign => self.state_sign(c, &mut token),
                State::String => self.state_string(c, &mut token),
                State::Comment => self.state_comment(c, &mut token),
            }
            assert!(token.is_some() || self.forward != previous_forward, "No lexing progress made!");
        }
        self.advance_begin();
        match token {
            Some(t) => Some(Lex::new(t)),
            None => None,
        }
    }
}

//
// UNIT TESTING
//

#[cfg(test)]
mod tests {
    use std::iter::Iterator;
    use super::*;
    use super::number::*;
    use super::token::*;

    #[test]
    fn lexer_finds_parens() {
        check_single_token("(", Token::LeftParen(String::from("(")));
        check_single_token(")", Token::RightParen(String::from(")")));
        check_single_token("#(", Token::LeftVectorParen);
    }

    #[test]
    fn lexer_finds_dots() {
        check_single_token(".", Token::Dot);

        let mut lexer = Lexer::new("abc . abc");
        assert_next_token(&mut lexer, &Token::Identifier(String::from("abc")));
        assert_next_token(&mut lexer, &Token::Dot);
        assert_next_token(&mut lexer, &Token::Identifier(String::from("abc")));
    }

    #[test]
    fn lexer_finds_identifiers() {
        check_single_token("abc", Token::Identifier(String::from("abc")));
        check_single_token("+", Token::Identifier(String::from("+")));
        check_single_token("-", Token::Identifier(String::from("-")));
    }

    #[test]
    fn lexer_finds_booleans() {
        check_single_token("#t", Token::Boolean(true));
        check_single_token("#f", Token::Boolean(false));
    }

    #[test]
    fn lexer_finds_comments() {
        let s = "; a comment";
        check_single_token(s, Token::Comment(String::from(s)));
    }

    #[test]
    fn lexer_finds_strings() {
        check_single_token("\"\"", Token::String(String::from("\"\"")));
        check_single_token("\"abc\"", Token::String(String::from("\"abc\"")));
    }

    #[test]
    fn finds_numbers() {
        check_single_token(".34", Token::Number(Number::new(0.34)));
        check_single_token("0.34", Token::Number(Number::new(0.34)));
    }

    #[test]
    fn finds_negative_numbers() {
        check_single_token("-3", Token::Number(Number::from_int(-3)));
        check_single_token("-0", Token::Number(Number::from_int(-0)));
        check_single_token("-0.56", Token::Number(Number::new(-0.56)));
        check_single_token("-3.14159", Token::Number(Number::new(-3.14159)));
    }

    #[test]
    fn finds_bin_numbers() {
        check_single_token("#b0", Token::Number(Number::from_int(0b0)));
        check_single_token("#b01011", Token::Number(Number::from_int(0b01011)));
    }

    #[test]
    fn finds_dec_numbers() {
        check_single_token("34", Token::Number(Number::new(34.0)));
        check_single_token("#d89", Token::Number(Number::from_int(89)));
    }

    #[test]
    fn finds_oct_numbers() {
        check_single_token("#o45", Token::Number(Number::from_int(0o45)));
    }

    #[test]
    fn finds_hex_numbers() {
        check_single_token("#h4A65", Token::Number(Number::from_int(0x4A65)));
    }

    fn check_single_token(input: &str, expected: Token) {
        let mut lexer = Lexer::new(input);
        assert_next_token(&mut lexer, &expected);
    }

    fn assert_next_token(lexer: &mut Lexer, expected: &Token) {
        let lex = lexer.next().unwrap();
        assert_eq!(lex.token, *expected);
    }
}
