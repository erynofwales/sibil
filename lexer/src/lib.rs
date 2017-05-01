/* lexer/src/lib.rs
 * Eryn Wells <eryn@erynwells.me>
 */

use std::iter::Peekable;

#[derive(Debug)]
pub enum Token { LeftParen, RightParen, Id(String), }

enum Resume { Here, AtNext } 

enum IterationResult {
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
        while let Some(peek) = self.input.peek().map(char::clone) {
            let result = if buffer.is_empty() {
                match peek {
                    '(' => self.emit(Token::LeftParen, Resume::AtNext),
                    ')' => self.emit(Token::RightParen, Resume::AtNext),
                    c if c.is_whitespace() => IterationResult::Continue,
                    c if c.is_alphabetic() => {
                        buffer.push(c);
                        IterationResult::Continue
                    },
                    c => self.fail(format!("Invalid character: {}", c)),
                }
            }
            else {
                match peek {
                    c if c.is_alphabetic() => {
                        buffer.push(c);
                        IterationResult::Continue
                    }
                    c if c == '(' || c == ')' || c.is_whitespace() =>
                        self.emit(Token::Id(buffer.clone()), Resume::Here),
                    c => self.fail(format!("Invalid character: {}", c)),
                }
            };
            match result {
                IterationResult::Continue => self.input.next(),
                IterationResult::Emit(token, resume) => {
                    match resume {
                        Resume::AtNext => self.input.next(),
                        Resume::Here => None,
                    };
                    return Some(Ok(token))
                },
                IterationResult::Error(msg) => return Some(Err(msg)),
            };
        }
        None
    }
}
