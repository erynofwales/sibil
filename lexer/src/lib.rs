/* lexer/src/lib.rs
 * Eryn Wells <eryn@erynwells.me>
 */

use std::iter::Peekable;
use chars::Lexable;

mod chars;
mod error;
mod token;

pub use error::Error;
pub use token::{Lex, Token};

pub type Result = std::result::Result<Lex, Error>;

#[derive(Debug, Eq, PartialEq)]
enum Resume { Here, AtNext } 

#[derive(Debug, Eq, PartialEq)]
enum IterationResult {
    Finish,
    Continue,
    Emit(Token, Resume),
    Error(Error),
}

pub struct Lexer<T> where T: Iterator<Item=char> {
    input: Peekable<T>,
    line: usize,
    offset: usize,
}

impl<T> Lexer<T> where T: Iterator<Item=char> {
    pub fn new(input: T) -> Lexer<T> {
        Lexer {
            input: input.peekable(),
            line: 0,
            offset: 0
        }
    }

    fn emit(&self, token: Token, resume: Resume) -> IterationResult {
        IterationResult::Emit(token, resume)
    }

    fn fail(&self, msg: String) -> IterationResult {
        IterationResult::Error(Error::new(msg))
    }
}

impl<T> Lexer<T> where T: Iterator<Item=char> {
    fn handle_whitespace(&mut self, c: char) {
        if c == '\n' {
            self.line += 1;
            self.offset = 0;
        }
        else {
            self.offset += 1;
        }
    }
}

impl<T> Iterator for Lexer<T> where T: Iterator<Item=char> {
    type Item = Result;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buffer = String::new();
        loop {
            let peek = self.input.peek().map(char::clone);
            let result = if buffer.is_empty() {
                match peek {
                    Some(c) if c.is_left_paren() => {
                        buffer.push(c);
                        self.emit(Token::LeftParen, Resume::AtNext)
                    },
                    Some(c) if c.is_right_paren() => {
                        buffer.push(c);
                        self.emit(Token::RightParen, Resume::AtNext)
                    },
                    Some(c) if c.is_whitespace() => {
                        self.handle_whitespace(c);
                        IterationResult::Continue
                    },
                    Some(c) if c.is_identifier_initial() => {
                        buffer.push(c);
                        IterationResult::Continue
                    },
                    Some(c) => self.fail(format!("Invalid character: {}", c)),
                    // We found EOF and there's no pending string, so just finish.
                    None => IterationResult::Finish,
                }
            }
            else {
                match peek {
                    Some(c) if c.is_identifier_subsequent() => {
                        buffer.push(c);
                        IterationResult::Continue
                    }
                    Some(c) if c.is_identifier_delimiter() =>
                        self.emit(Token::Id, Resume::Here),
                    Some(c) => self.fail(format!("Invalid character: {}", c)),
                    // Found EOF. Emit what we have and finish.
                    // Note: the Resume argument doesn't matter in this case since the input
                    // iterator will always be None from here on.
                    None => self.emit(Token::Id, Resume::Here),
                }
            };
            match result {
                IterationResult::Finish => break,
                IterationResult::Continue => self.input.next(),
                IterationResult::Emit(token, resume) => {
                    if resume == Resume::AtNext {
                        self.input.next();
                    }
                    let lex = Lex::new(token, &buffer, self.line, self.offset);
                    return Some(Ok(lex))
                },
                IterationResult::Error(err) => return Some(Err(err)),
            };
        }
        None
    }
}
