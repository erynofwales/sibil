/* lexer/src/states/bool.rs
 * Eryn Wells <eryn@erynwells.me>
 */

use chars::Lexable;
use states::{Resume, State, StateResult};
use token::Token;

const TRUE_SHORT: &'static str = "t";
const TRUE: &'static str = "true";
const FALSE_SHORT: &'static str = "f";
const FALSE: &'static str = "false";

#[derive(Debug)] pub struct Bool(String);

impl Bool {
    pub fn new(buf: &str) -> Bool {
        Bool(buf.to_string())
    }

    fn handle_delimiter(&self) -> Option<Token> {
        if self.0 == TRUE || self.0 == TRUE_SHORT {
            Some(Token::Bool(true))
        } else if self.0 == FALSE || self.0 == FALSE_SHORT {
            Some(Token::Bool(false))
        } else {
            None
        }
    }
}

impl State for Bool {
    fn lex(&mut self, c: char) -> StateResult {
        match c {
            c if c.is_identifier_delimiter() => match self.handle_delimiter() {
                Some(token) => StateResult::Emit(token, Resume::Here),
                None => {
                    let msg = format!("Invalid character: {}", c);
                    StateResult::fail(msg.as_str())
                },
            },
            _ => {
                let buf = {
                    let mut b = String::from(self.0.as_str());
                    b.push(c);
                    b
                };
                if TRUE.starts_with(&buf) || FALSE.starts_with(&buf) {
                    StateResult::advance(Box::new(Bool(buf)))
                } else {
                    let msg = format!("Invalid character: {}", c);
                    StateResult::fail(msg.as_str())
                }
            },
        }
    }

    fn none(&mut self) -> Result<Option<Token>, String> {
        match self.handle_delimiter() {
            Some(token) => Ok(Some(token)),
            None => {
                let msg = format!("Found EOF while trying to parse a bool");
                Err(msg)
            }
        }
    }
}

