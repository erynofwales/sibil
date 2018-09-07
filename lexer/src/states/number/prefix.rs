/* lexer/src/states/number/prefix.rs
 * Eryn Wells <eryn@erynwells.me>
 */

use chars::Lexable;
use error::Error;
use states::{State, StateResult};
use states::number::{Builder, Radix, Exact};
use states::number::digit::Digit;
use states::number::sign::Sign;
use token::Token;

#[derive(Debug)] pub struct Prefix(Builder);
#[derive(Debug)] pub struct Hash(Builder);

impl Prefix {
    pub fn new(b: Builder) -> Prefix {
        Prefix(b)
    }

    pub fn with_char(b: &Builder, c: char) -> Option<Prefix> {
        if let Some(ex) = Exact::from(c) {
            if b.seen_exact() {
                return None;
            }
            let mut b = b.clone();
            b.push_exact(ex);
            Some(Prefix::new(b))
        } else if let Some(rx) = Radix::from(c) {
            if b.seen_radix() {
                return None;
            }
            let mut b = b.clone();
            b.push_radix(rx);
            Some(Prefix::new(b))
        } else {
            None
        }
    }
}

impl State for Prefix {
    fn lex(&mut self, c: char) -> StateResult {
        if c.is_hash() {
            StateResult::advance(Box::new(Hash::new(&self.0)))
        } else if let Some(st) = Sign::with_char(&self.0, c) {
            StateResult::advance(Box::new(st))
        } else if let Some(st) = Digit::with_char(&self.0, c) {
            StateResult::advance(Box::new(st))
        } else {
            StateResult::fail(Error::invalid_char(c))
        }
    }

    fn none(&mut self) -> Result<Option<Token>, Error> {
        Err(Error::unexpected_eof())
    }
}

impl Hash {
    fn new(b: &Builder) -> Hash {
        Hash(b.clone())
    }
}

impl State for Hash {
    fn lex(&mut self, c: char) -> StateResult {
        if let Some(st) = Prefix::with_char(&self.0, c) {
            StateResult::advance(Box::new(st))
        } else {
            StateResult::fail(Error::invalid_char(c))
        }
    }

    fn none(&mut self) -> Result<Option<Token>, Error> {
        Err(Error::new("blah".to_string()))
    }
}
