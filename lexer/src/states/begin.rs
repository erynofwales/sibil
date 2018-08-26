/* lexer/src/states/begin.rs
 * Eryn Wells <eryn@erynwells.me>
 */

use chars::Lexable;
use token::Token;
use states::{Resume, State, StateResult};
use states::id::IdSub;
use states::hash::Hash;

#[derive(Debug)]
pub struct Begin;

impl State for Begin {
    fn lex(&mut self, c: char) -> StateResult {
        match c {
            c if c.is_left_paren() => StateResult::Emit(Token::LeftParen, Resume::AtNext),
            c if c.is_right_paren() => StateResult::Emit(Token::RightParen, Resume::AtNext),
            // TODO: Figure out some way to track newlines.
            c if c.is_whitespace() => StateResult::Continue,
            c if c.is_identifier_initial() => StateResult::Advance { to: Box::new(IdSub{}) },
            c if c.is_hash() => StateResult::Advance { to: Box::new(Hash{}) },
            _ => {
                let msg = format!("Invalid character: {}", c);
                StateResult::Fail { msg }
            }
        }
    }

    fn none(&mut self) -> Result<Option<Token>, String> {
        Ok(None)
    }
}

trait BeginLexable {
    fn is_hash(&self) -> bool;
}

impl BeginLexable for char {
    fn is_hash(&self) -> bool { *self == '#' }
}
