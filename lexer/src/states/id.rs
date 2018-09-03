/* lexer/src/states/id.rs
 * Eryn Wells <eryn@erynwells.me>
 */

use chars::Lexable;
use error::Error;
use states::{Resume, State, StateResult};
use token::Token;

#[derive(Debug)]
pub struct IdSub;

impl State for IdSub {
    fn lex(&mut self, c: char) -> StateResult {
        match c {
            c if c.is_identifier_subsequent() => StateResult::Continue,
            c if c.is_identifier_delimiter() => StateResult::Emit(Token::Id, Resume::Here),
            _ => StateResult::fail(Error::invalid_char(c)),
        }
    }

    fn none(&mut self) -> Result<Option<Token>, Error> {
        Ok(Some(Token::Id))
    }
}
