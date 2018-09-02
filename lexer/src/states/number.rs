/* lexer/src/states/number.rs
 * Eryn Wells <eryn@erynwells.me>
 */

use super::{State, StateResult, Token};

#[derive(Debug, Eq, PartialEq)]
pub enum Base {
    Bin = 2,
    Oct = 8,
    Dec = 10,
    Hex = 16
}

#[derive(Debug)]
pub struct Builder {
    base: Base
}

#[derive(Debug)]
pub struct BeginNumber(Builder);

impl Builder {
    pub fn new() -> Builder {
        Builder { base: Base::Dec }
    }
}

impl BeginNumber {
    pub fn new() -> BeginNumber {
        BeginNumber(Builder::new())
    }
}

impl State for BeginNumber {
    fn lex(&mut self, c: char) -> StateResult {
        // TODO: Implement.
        StateResult::fail("blah")
    }

    fn none(&mut self) -> Result<Option<Token>, String> {
        // TODO: Implement.
        Err("blah".to_string())
    }
}