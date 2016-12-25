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
    /// Handle self.state == State::Initial
    fn state_initial(&mut self, c: char, token: &mut Option<Token>) {
        if c.is_left_paren() {
            *token = Some(Token::LeftParen(c.to_string()));
        }
        else if c.is_right_paren() {
            *token = Some(Token::RightParen(c.to_string()));
        }

        else if c.is_identifier_single() {
            *token = Some(Token::Identifier(c.to_string()));
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
                self.handle_newline();
            }
            self.advance_begin();
        }

        else if c.is_comment_initial() {
            self.state = State::Comment;
            self.advance();
        }
    }

    /// Handle self.state == State::Identifier
    fn state_identifier(&mut self, c: char, token: &mut Option<Token>) {
        if c.is_identifier_subsequent() {
            // State in Identifier state.
            self.advance();
        }
        else {
            *token = Some(Token::Identifier(self.value()));
            self.retract();
        }
    }

    fn state_hash(&mut self, c: char, token: &mut Option<Token>) {
        if c.is_boolean_true() || c.is_boolean_false() {
            self.advance();
            *token = Some(Token::Boolean(c.is_boolean_true()));
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
            match self.state {
                State::Initial => self.state_initial(c, &mut token),
                State::Identifier => self.state_identifier(c, &mut token),
                State::Hash => self.state_hash(c, &mut token),
                State::Comment => self.state_comment(c, &mut token),
            }
        }
        self.advance_begin();
        let mut lex: Option<Lex> = None;
        if let Some(token) = token {
            lex = Some(Lex::new(token));
        }
        lex
    }
}

//
// UNIT TESTING
//

#[test]
fn lexer_finds_parens() {
    let mut lexer = Lexer::new("()");
    assert_next_token(&mut lexer, &Token::LeftParen("(".to_string()));
    assert_next_token(&mut lexer, &Token::RightParen(")".to_string()));
}

#[test]
fn lexer_finds_identifiers() {
    let s = "abc";
    let mut lexer = Lexer::new(s);
    assert_next_token(&mut lexer, &Token::Identifier(s.to_string()));
}

#[test]
fn lexer_finds_booleans() {
    let mut lexer = Lexer::new("#t #f");
    assert_next_token(&mut lexer, &Token::Boolean(true));
    assert_next_token(&mut lexer, &Token::Boolean(false));
}

#[test]
fn lexer_finds_comments() {
    let s = "; a comment";
    let mut lexer = Lexer::new(s);
    assert_next_token(&mut lexer, &Token::Comment(s.to_string()));
}

fn assert_next_token(lexer: &mut Lexer, expected: &Token) {
    let lex = lexer.next().unwrap();
    assert_eq!(lex.token, *expected);
}
