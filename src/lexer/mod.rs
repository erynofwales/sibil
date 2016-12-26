/* lexer.rs
 * Eryn Wells <eryn@erynwells.me>
 */

pub mod token;
mod char;
mod charset;
mod str;

use self::char::Lexable;
use self::str::CharAt;
use self::str::RelativeIndexable;
use self::token::Lex;
use self::token::Token;

#[derive(Debug)]
enum State {
    Initial,
    Identifier,
    Hash,
    Comment,
    String,
}

pub struct Lexer {
    input: String,
    begin: usize,
    forward: usize,
    line: u32,
    state: State,
}

impl Lexer {
    pub fn new(input: &str) -> Lexer {
        Lexer {
            input: String::from(input),
            begin: 0,
            forward: 0,
            line: 1,
            state: State::Initial,
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
            *token = Some(Token::Dot);
        }
        else if c.is_hash() {
            self.state = State::Hash;
            self.advance();
        }
        else if c.is_string_quote() {
            self.state = State::String;
            self.advance();
        }

        else if c.is_identifier_single() {
            *token = Some(Token::Identifier(c.to_string()));
        }
        else if c.is_identifier_initial() {
            self.state = State::Identifier;
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
            // State in Identifier state.
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

    fn state_hash(&mut self, c: char, token: &mut Option<Token>) {
        if c.is_boolean_true() || c.is_boolean_false() {
            self.advance();
            *token = Some(Token::Boolean(c.is_boolean_true()));
        }
        else if c.is_left_paren() {
            self.advance();
            *token = Some(Token::LeftVectorParen);
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
                State::Hash => self.state_hash(c, &mut token),
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
    }

    #[test]
    fn lexer_finds_identifiers() {
        check_single_token("abc", Token::Identifier(String::from("abc")));
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

    fn check_single_token(input: &str, expected: Token) {
        let mut lexer = Lexer::new(input);
        assert_next_token(&mut lexer, &expected);
    }

    fn assert_next_token(lexer: &mut Lexer, expected: &Token) {
        let lex = lexer.next().unwrap();
        assert_eq!(lex.token, *expected);
    }
}
