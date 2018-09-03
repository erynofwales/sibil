/* lexer/src/states/number/prefix.rs
 * Eryn Wells <eryn@erynwells.me>
 */

use super::{Radix, Exact};
use states::{State, StateResult};
use states::number::Builder;
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
        match c {
            '#' => StateResult::advance(Box::new(Hash(self.0))),
            _ => StateResult::fail(format!("invalid char: {}", c).as_str())
        }
    }

    fn none(&mut self) -> Result<Option<Token>, String> {
        Err("blah".to_string())
    }
}

impl State for Hash {
    fn lex(&mut self, c: char) -> StateResult {
        if let Some(ex) = Exact::from(c) {
            if !self.0.seen_exact() {
                self.0.push_exact(ex);
                StateResult::advance(Box::new(Prefix::new(self.0)))
            } else {
                StateResult::fail(format!("invalid char: {}", c).as_str())
            }
        } else if let Some(rx) = Radix::from(c) {
            if !self.0.seen_radix() {
                self.0.push_radix(rx);
                StateResult::advance(Box::new(Prefix::new(self.0)))
            } else {
                StateResult::fail(format!("invalid char: {}", c).as_str())
            }
        } else {
            StateResult::fail(format!("invalid char: {}", c).as_str())
        }
    }

    fn none(&mut self) -> Result<Option<Token>, String> {
        Err("blah".to_string())
    }
}
