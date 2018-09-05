/* lexer/src/states/whitespace.rs
 * Eryn Wells <eryn@erynwells.me>
 */

use error::Error;
use states::{Resume, State, StateResult};
use token::Token;

#[derive(Debug)]
pub struct Whitespace;

impl Whitespace {
    pub fn new() -> Whitespace {
        Whitespace{}
    }
}

impl State for Whitespace {
    fn lex(&mut self, c: char) -> StateResult {
        if c.is_whitespace() {
            StateResult::Continue
        } else {
            StateResult::Discard(Resume::Here)
        }
    }

    fn none(&mut self) -> Result<Option<Token>, Error> {
        Ok(None)
    }
}
