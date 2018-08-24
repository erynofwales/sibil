/* parser/src/lib.rs
 * Eryn Wells <eryn@erynwells.me>
 */

extern crate sibillexer;
extern crate sibiltypes;

mod list_parser;
mod node_parser;
mod program_parser;
mod sym_parser;

use std::iter::Peekable;
use sibillexer::Result as LexerResult;
use sibiltypes::Obj;
use node_parser::{NodeParser, NodeParseResult};
use program_parser::ProgramParser;

/// The output of calling `parse()` on a Parser is one of these Result objects.
pub type Result = std::result::Result<Obj, ParseError>;

#[derive(Debug)]
pub enum ParseError {
    LexerError{msg: String},
    ParserError{msg: String}
}

pub struct Parser<T> where T: Iterator<Item=LexerResult> {
    input: Peekable<T>,
    parsers: Vec<Box<NodeParser>>,
}

impl<T> Parser<T> where T: Iterator<Item=LexerResult> {
    pub fn new(input: T) -> Parser<T> {
        let program_parser = Box::new(ProgramParser::new());
        Parser {
            input: input.peekable(),
            parsers: vec!(program_parser)
        }
    }
}

impl<T> Iterator for Parser<T> where T: Iterator<Item=LexerResult> {
    type Item = Result;

    fn next(&mut self) -> Option<Self::Item> {
        let mut lex = self.input.next();
        loop {
            match lex {
                Some(ref lex) => {
                    match lex {
                        Ok(ref lex) => {
                        }
                        Err(error) => {}
                    }
                },
                None => break
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
