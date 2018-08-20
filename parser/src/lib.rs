/* parser/src/lib.rs
 * Eryn Wells <eryn@erynwells.me>
 */

extern crate sibillexer;
extern crate sibiltypes;

use std::iter::Peekable;
use sibillexer::Result as LexerResult;
use sibiltypes::Object;

#[derive(Debug)]
pub struct ParseError;

pub type Result = std::result::Result<Object, ParseError>;

trait NodeParser {
}

pub struct Parser<T> where T: Iterator<Item=LexerResult> {
    input: Peekable<T>,
    parsers: Vec<Box<NodeParser>>,
}

impl<T> Parser<T> where T: Iterator<Item=LexerResult> {
    pub fn new(input: T) -> Parser<T> {
        Parser {
            input: input.peekable(),
            parsers: Vec::new(),
        }
    }
}

impl<T> Iterator for Parser<T> where T: Iterator<Item=LexerResult> {
    type Item = Result;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(lex) = self.input.next() {
                println!("{:?}", lex)
            }
            else {
                break;
            }
        }
        assert_eq!(self.parsers.len(), 0);
        None
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
