/* lexer/src/states/hash.rs
 * Eryn Wells <eryn@erynwells.me>
 */

use states::{State, StateResult};
use states::bool::Bool;
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
            _ => {
                let msg = format!("Invalid character: {}", c);
                StateResult::fail(msg.as_str())
            },
        }
    }

    fn none(&mut self) -> Result<Option<Token>, String> {
        Ok(None)
    }
}

impl HashLexable for char {
    fn is_bool_initial(&self) -> bool { "tf".contains(self.to_ascii_lowercase()) }
    fn is_slash(&self) -> bool { *self == '\\' }
}
