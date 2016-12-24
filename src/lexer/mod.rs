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
use self::token::Token;
use self::token::Kind;

enum State {
    Initial,
    Identifier,
    Hash,
}

pub struct Lexer {
    input: String,
    begin: usize,
    forward: usize,
    line: u32,
    state: State,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        Lexer {
            input: input,
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

    /// Get the substring between the two input indexes. This is the value to give to a new Token instance.
    fn value(&self) -> String {
        self.input[self.begin .. self.forward].to_string()
    }
}

impl Lexer {
    /// Handle self.state == State::Initial
    fn state_initial(&mut self, c: char, token: &mut Option<Token>) {
        if c.is_left_paren() {
            *token = Some(Token::new(Kind::LeftParen, c.to_string()));
        }
        else if c.is_right_paren() {
            *token = Some(Token::new(Kind::RightParen, c.to_string()));
        }

        else if c.is_identifier_single() {
            *token = Some(Token::new(Kind::Identifier, c.to_string()));
        }
        else if c.is_identifier_initial() {
            self.state = State::Identifier;
            self.advance();
        }

        else if c.is_hash() {
            self.state = State::Hash;
            self.advance();
        }

        else if c.is_whitespace() {
            if c.is_newline() {
                self.line += 1;
            }
            self.advance_begin();
        }
    }

    /// Handle self.state == State::Identifier
    fn state_identifier(&mut self, c: char, token: &mut Option<Token>) {
        if c.is_identifier_subsequent() {
            // State in Identifier state.
            self.advance();
        }
        else {
            *token = Some(Token::new(Kind::Identifier, self.value()));
            self.retract();
        }
    }

    fn state_hash(&mut self, c: char, token: &mut Option<Token>) {
        if c.is_boolean() {
            self.advance();
            *token = Some(Token::new(Kind::Boolean, self.value()));
        }
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        self.begin_lexing();
        if self.begin == self.input.len() {
            return None;
        }
        let mut token: Option<Token> = None;
        println!("Lexing '{}'", &self.input[self.begin ..]);
        while token.is_none() {
            if let Some(c) = self.input.char_at(self.forward) {
                println!("{}! c='{}'", self.state, c);
                match self.state {
                    State::Initial => self.state_initial(c, &mut token),
                    State::Identifier => self.state_identifier(c, &mut token),
                    State::Hash => self.state_hash(c, &mut token),
                }
            }
            else {
                assert!(false, "Invalid character! :-(");
            }
        }
        self.advance_begin();
        assert!(token.is_some(), "We quit the lexing loop but didn't actually have a token. :-(");
        token
    }
}
