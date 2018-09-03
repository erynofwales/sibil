/* lexer/src/states/number/prefix.rs
 * Eryn Wells <eryn@erynwells.me>
 */

use chars::Lexable;
use error::Error;
use states::{State, StateResult};
use states::number::{Builder, Radix, Exact};
use states::number::sign::Sign;
use token::Token;

#[derive(Debug)] pub struct Prefix(Builder);
#[derive(Debug)] pub struct Hash(Builder);

impl Prefix {
    pub fn new(b: Builder) -> Prefix {
        Prefix(b)
    }

    pub fn with_char(c: char) -> Option<Prefix> {
        let mut builder = Builder::new();
        if let Some(ex) = Exact::from(c) {
            builder.push_exact(ex);
        } else if let Some(rx) = Radix::from(c) {
            builder.push_radix(rx);
        } else {
            return None;
        }
        Some(Prefix::new(builder))
    }
}

impl State for Prefix {
    fn lex(&mut self, c: char) -> StateResult {
        if c.is_hash() {
            StateResult::advance(Box::new(Hash(self.0)))
        } else if let Some(sn) = Sign::with_char(self.0, c) {
            StateResult::advance(Box::new(sn))
        } else {
            StateResult::fail(Error::invalid_char(c))
        }
    }

    fn none(&mut self) -> Result<Option<Token>, Error> {
        Err(Error::new("blah".to_string()))
    }
}

impl State for Hash {
    fn lex(&mut self, c: char) -> StateResult {
        if let Some(ex) = Exact::from(c) {
            if !self.0.seen_exact() {
                self.0.push_exact(ex);
                StateResult::advance(Box::new(Prefix::new(self.0)))
            } else {
                StateResult::fail(Error::invalid_char(c))
            }
        } else if let Some(rx) = Radix::from(c) {
            if !self.0.seen_radix() {
                self.0.push_radix(rx);
                StateResult::advance(Box::new(Prefix::new(self.0)))
            } else {
                StateResult::fail(Error::invalid_char(c))
            }
        } else {
            StateResult::fail(Error::invalid_char(c))
        }
    }

    fn none(&mut self) -> Result<Option<Token>, Error> {
        Err(Error::new("blah".to_string()))
    }
}
