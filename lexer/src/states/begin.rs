/* lexer/src/states/begin.rs
 * Eryn Wells <eryn@erynwells.me>
 */

use chars::Lexable;
use error::Error;
use token::Token;
use states::{Resume, State, StateResult};
use states::id::IdSub;
use states::hash::Hash;
use states::number::{Builder, Digit};
use states::whitespace::Whitespace;

#[derive(Debug)]
pub struct Begin;

impl Begin {
    pub fn new() -> Begin {
        Begin{}
    }
}

impl State for Begin {
    fn lex(&mut self, c: char) -> StateResult {
        if c.is_whitespace() {
            StateResult::advance(Box::new(Whitespace::new()))
        } else if c.is_left_paren() {
            StateResult::Emit(Token::LeftParen, Resume::AtNext)
        } else if c.is_right_paren() {
            StateResult::Emit(Token::RightParen, Resume::AtNext)
        } else if c.is_whitespace() {
            // TODO: Figure out some way to track newlines.
            StateResult::Continue
        } else if c.is_identifier_initial() {
            StateResult::advance(Box::new(IdSub{}))
        } else if c.is_hash() {
            StateResult::advance(Box::new(Hash::new()))
        } else if let Some(st) = Digit::with_char(&Builder::new(), c) {
            StateResult::advance(Box::new(st))
        } else {
            StateResult::fail(Error::invalid_char(c))
        }
    }

    fn none(&mut self) -> Result<Option<Token>, Error> {
        Ok(None)
    }
}
