/* lexer/src/states/dot.rs
 * Eryn Wells <eryn@erynwells.me>
 */

use chars::Lexable;
use error::Error;
use states::{Resume, State, StateResult};
use states::number::{Builder, Digit};
use token::Token;

#[derive(Debug)] pub struct Dot;

impl Dot {
    pub fn new() -> Dot {
        Dot{}
    }
}

impl State for Dot {
    fn lex(&mut self, c: char) -> StateResult {
        if c.is_identifier_delimiter() {
            StateResult::emit(Token::Dot, Resume::Here)
        } else if let Some(st) = Digit::with_char(&Builder::new(), c) {
            StateResult::advance(Box::new(st))
        } else {
            StateResult::fail(Error::invalid_char(c))
        }
    }

    fn none(&mut self) -> Result<Option<Token>, Error> {
        Ok(Some(Token::Dot))
    }
}
