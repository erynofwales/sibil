/* lexer/src/states/mod.rs
 * Eryn Wells <eryn@erynwells.me>
 */

use std::fmt::Debug;
use error::Error;
use token::Token;

mod begin;
mod bool;
mod hash;
mod number;
mod id;

pub use self::begin::Begin;

#[derive(Debug)]
pub enum StateResult {
    /// Consume the character, remain on this state.
    Continue,
    /// Consume the character, advance to the provided state.
    Advance { to: Box<State> },
    /// Emit a Lex with the provided Token and the accumulated buffer. The Resume value indicates
    /// whether to revisit the current input character or advance to the next one.
    Emit(Token, Resume),
    Fail(Error)
}

#[derive(Debug, Eq, PartialEq)]
pub enum Resume {
    /// Revisit the current input character in the next state.
    Here,
    /// Advance the input to the next character before beginning the next token.
    AtNext
} 

pub trait State: Debug {
    fn lex(&mut self, c: char) -> StateResult;
    fn none(&mut self) -> Result<Option<Token>, Error>;
}

impl StateResult {
    pub fn advance(to: Box<State>) -> StateResult {
        StateResult::Advance { to }
    }

    pub fn emit(token: Token, at: Resume) -> StateResult {
        StateResult::Emit(token, at)
    }

    pub fn fail(err: Error) -> StateResult {
        StateResult::Fail(err)
    }
}
