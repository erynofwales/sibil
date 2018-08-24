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
        let mut out: Option<Self::Item> = None;
        let mut result: Option<NodeParseResult> = None;
        loop {
            match self.input.next() {
                Some(Ok(ref lex)) => {
                    // TODO: Valid Lex from our input. Hand it off to the
                    // current parser and process the result.
                },
                Some(Err(ref error)) => {
                    // TODO: Lexer error. Throw it up and out.
                    out = Some(Err(ParseError::LexerError { msg: error.msg().to_string() }));
                },
                None => {
                    // TODO: We didn't get a Lex from the input. If there's any
                    // parse result waiting around, clean it up and return it or
                    // return an error.
                    break;
                }
            }
        }
        assert_eq!(self.parsers.len(), 0);
        out
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
