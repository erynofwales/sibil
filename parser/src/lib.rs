/* parser/src/lib.rs
 * Eryn Wells <eryn@erynwells.me>
 */

extern crate sibillexer;
extern crate sibiltypes;

mod list_parser;
mod node_parser;
mod sym_parser;

use std::iter::Peekable;
use sibillexer::Result as LexerResult;
use sibiltypes::Obj;
use node_parser::NodeParser;
use sym_parser::SymParser;

/// The output of calling `parse()` on a Parser is one of these Result objects.
pub type Result = std::result::Result<Obj, ParseError>;

#[derive(Debug)]
pub struct ParseError;

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
                if let Ok(lex) = lex {
                } else {
                }
            } else {
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
