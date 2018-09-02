/* lexer/src/states/hash.rs
 * Eryn Wells <eryn@erynwells.me>
 */

//! Lexer states for handling tokens that begin with hash marks '#'.

use chars::Lexable;
use states::{Resume, State, StateResult};
use token::Token;

const TRUE_SHORT: &'static str = "t";
const TRUE: &'static str = "true";
const FALSE_SHORT: &'static str = "f";
const FALSE: &'static str = "false";

#[derive(Debug)] pub struct Hash;
#[derive(Debug)] pub struct BoolSub(String);

impl Hash {
    pub fn new() -> Hash { Hash{} }
}

impl State for Hash {
    fn lex(&mut self, c: char) -> StateResult {
        match c {
            c if TRUE.starts_with(c) || FALSE.starts_with(c) => {
                let buf = c.to_lowercase().to_string();
                StateResult::Advance { to: Box::new(BoolSub(buf)) }
            }
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

impl BoolSub {
    fn handle_delimiter(&self) -> Result<(Token, Resume), ()> {
        if self.0 == TRUE || self.0 == TRUE_SHORT {
            Ok((Token::Bool(true), Resume::Here))
        } else if self.0 == FALSE || self.0 == FALSE_SHORT {
            Ok((Token::Bool(false), Resume::Here))
        } else {
            Err(())
        }
    }
}

impl State for BoolSub {
    fn lex(&mut self, c: char) -> StateResult {
        match c {
            c if c.is_identifier_delimiter() => match self.handle_delimiter() {
                Ok((token, resume)) => StateResult::Emit(token, resume),
                Err(_) => {
                    let msg = format!("Invalid character: {}", c);
                    StateResult::Fail { msg }
                }
            },
            _ => {
                let buf = {
                    let mut b = String::from(self.0.as_str());
                    b.push(c);
                    b
                };
                if TRUE.starts_with(&buf) || FALSE.starts_with(&buf) {
                    StateResult::Advance { to: Box::new(BoolSub(buf)) }
                } else {
                    let msg = format!("Invalid character: {}", c);
                    StateResult::Fail { msg }
                }
            }
        }
    }

    fn none(&mut self) -> Result<Option<Token>, String> {
        match self.handle_delimiter() {
            Ok((token, _)) => Ok(Some(token)),
            Err(_) => {
                let msg = format!("Found EOF while trying to parse a bool");
                Err(msg)
            }
        }
    }
}

trait HashLexable {
    fn is_tf(&self) -> bool;
    fn is_slash(&self) -> bool;
}

impl HashLexable for char {
    fn is_tf(&self) -> bool { "tfTF".contains(*self) }
    fn is_slash(&self) -> bool { *self == '\\' }
}
