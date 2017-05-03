/* lexer/src/lib.rs
 * Eryn Wells <eryn@erynwells.me>
 */

use std::iter::Peekable;

#[derive(Debug, Eq, PartialEq)]
pub enum Token { LeftParen, RightParen, Id(String), }

#[derive(Debug, Eq, PartialEq)]
enum Resume { Here, AtNext } 

#[derive(Debug, Eq, PartialEq)]
enum IterationResult {
    Finish,
    Continue,
    Emit(Token, Resume),
    Error(String),
}

pub struct Lexer<T> where T: Iterator<Item=char> {
    input: Peekable<T>,
}

impl<T> Lexer<T> where T: Iterator<Item=char> {
    pub fn new(input: T) -> Lexer<T> {
        Lexer { input: input.peekable() }
    }

    fn emit(&self, token: Token, resume: Resume) -> IterationResult {
        IterationResult::Emit(token, resume)
    }

    fn fail(&self, msg: String) -> IterationResult {
        IterationResult::Error(msg)
    }
}

impl<T> Iterator for Lexer<T> where T: Iterator<Item=char> {
    type Item = Result<Token, String>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buffer = String::new();
        loop {
            let peek = self.input.peek().map(char::clone);
            let result = if buffer.is_empty() {
                match peek {
                    Some('(') => self.emit(Token::LeftParen, Resume::AtNext),
                    Some(')') => self.emit(Token::RightParen, Resume::AtNext),
                    Some(c) if c.is_whitespace() => IterationResult::Continue,
                    Some(c) if c.is_alphabetic() => {
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
                    Some(c) if c.is_alphabetic() => {
                        buffer.push(c);
                        IterationResult::Continue
                    }
                    Some(c) if c == '(' || c == ')' || c.is_whitespace() =>
                        self.emit(Token::Id(buffer.clone()), Resume::Here),
                    Some(c) => self.fail(format!("Invalid character: {}", c)),
                    // Found EOF. Emit what we have and finish.
                    // Note: the Resume argument doesn't matter in this case since the input
                    // iterator will always be None from here on.
                    None => self.emit(Token::Id(buffer.clone()), Resume::Here),
                }
            };
            match result {
                IterationResult::Finish => break,
                IterationResult::Continue => self.input.next(),
                IterationResult::Emit(token, resume) => {
                    if resume == Resume::AtNext {
                        self.input.next();
                    }
                    return Some(Ok(token))
                },
                IterationResult::Error(msg) => return Some(Err(msg)),
            };
        }
        None
    }
}
