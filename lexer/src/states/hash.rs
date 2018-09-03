/* lexer/src/states/hash.rs
 * Eryn Wells <eryn@erynwells.me>
 */

use chars::Lexable;
use error::Error;
use states::{State, StateResult};
use states::bool::Bool;
use states::number::Prefix;
use token::Token;

trait HashLexable {
    fn is_bool_initial(&self) -> bool;
    fn is_slash(&self) -> bool;
}

#[derive(Debug)] pub struct Hash;

impl Hash {
    pub fn new() -> Hash { Hash{} }
}

impl State for Hash {
    fn lex(&mut self, c: char) -> StateResult {
        match c {
            c if c.is_bool_initial() => {
                let buf = c.to_ascii_lowercase().to_string();
                StateResult::advance(Box::new(Bool::new(buf.as_str())))
            },
            c if c.is_radix() || c.is_exactness() => {
                if let Some(st) = Prefix::with_char(c) {
                    StateResult::advance(Box::new(st))
                } else {
                    StateResult::fail(Error::new(format!("invalid numeric prefix character: {}", c)))
                }
            },
            _ => StateResult::fail(Error::invalid_char(c)),
        }
    }

    fn none(&mut self) -> Result<Option<Token>, Error> {
        Ok(None)
    }
}

impl HashLexable for char {
    fn is_bool_initial(&self) -> bool { "tf".contains(self.to_ascii_lowercase()) }
    fn is_slash(&self) -> bool { *self == '\\' }
}
