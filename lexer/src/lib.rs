/* lexer/src/lib.rs
 * Eryn Wells <eryn@erynwells.me>
 */

use std::iter::Peekable;
use states::{Begin, Resume, StateResult};

mod chars;
mod error;
mod states;
mod token;

pub use error::Error;
pub use token::{Lex, Token};

pub type Result = std::result::Result<Lex, Error>;

pub struct Lexer<T> where T: Iterator<Item=char> {
    /// The input stream.
    input: Peekable<T>,

    /// Current line number.
    line: usize,
    /// Character offset from the start of the input.
    offset: usize,
}

impl<T> Lexer<T> where T: Iterator<Item=char> {
    pub fn new(input: T) -> Lexer<T> {
        Lexer {
            input: input.peekable(),
            line: 0,
            offset: 0,
        }
    }

    fn next(&mut self) -> Option<T::Item> {
        let out = self.input.next();
        if let Some(c) = out {
            self.update_offsets(c);
        }
        out
    }

    fn handle_error(&self, err: Error) {
        panic!("{}:{}: {}", self.line, self.offset, err.msg())
    }

    fn prepare_offsets(&mut self) { }

    fn update_offsets(&mut self, c: char) {
        self.offset += 1;
        match c {
            '\n' => {
                self.line += 1;
                self.offset = 0;
            },
            _ => {
                self.offset += 1;
            },
        }
    }
}

impl<T> Iterator for Lexer<T> where T: Iterator<Item=char> {
    type Item = Result;

    fn next(&mut self) -> Option<Self::Item> {
        self.prepare_offsets();

        let mut buffer = String::new();
        let mut state: Box<states::State> = Box::new(Begin::new());
        let mut out: Option<Self::Item> = None;
        loop {
            let peek = self.input.peek().map(char::clone);
            println!("lexing {:?} in state {:?}, buffer = {:?}", peek, state, buffer);
            match peek {
                None => match state.none() {
                    Ok(None) => break,
                    Ok(Some(token)) => {
                        out = Some(Ok(Lex::new(token, &buffer, self.line, self.offset)));
                        break;
                    },
                    Err(err) => self.handle_error(err)
                },
                Some(c) => {
                    let result = state.lex(c);
                    match result {
                        StateResult::Continue => {
                            buffer.push(c);
                            self.next();
                        },
                        StateResult::Advance { to } => {
                            buffer.push(c);
                            self.next();
                            state = to;
                        },
                        StateResult::Discard(resume) => {
                            buffer.clear();
                            state = Box::new(Begin::new());
                            if resume == Resume::AtNext {
                                self.next();
                            }
                        },
                        StateResult::Emit(token, resume) => {
                            if resume == Resume::AtNext {
                                buffer.push(c);
                                self.next();
                            }
                            out = Some(Ok(Lex::new(token, &buffer, self.line, self.offset)));
                            break;
                        },
                        StateResult::Fail(err) => self.handle_error(err),
                    }
                },
            }
        }
        out
    }
}
