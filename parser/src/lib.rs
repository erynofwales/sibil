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
use sibillexer::Lex;
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

    fn parse_lex(&mut self, lex: &Lex) -> NodeParseResult {
        let parser = self.parsers.last_mut().expect("couldn't get a parser -- this is unexpected");
        parser.parse(lex)
    }
}

impl<T> Iterator for Parser<T> where T: Iterator<Item=LexerResult> {
    type Item = Result;

    fn next(&mut self) -> Option<Self::Item> {
        let mut out: Option<Self::Item> = None;
        let mut result: Option<NodeParseResult> = None;
        let mut input_lex: Option<T::Item> = None;
        loop {
            input_lex = match result {
                None => self.input.next(),  // Starting condition
                Some(NodeParseResult::Continue) => self.input.next(),
                Some(NodeParseResult::Complete{ obj }) => {
                    self.parsers.pop();
                    // TODO: Handle obj
                    self.input.next()
                },
                Some(NodeParseResult::Push{ next }) => {
                    self.parsers.push(next);
                    input_lex
                },
                Some(NodeParseResult::Error{ msg }) => {
                    out = Some(Err(ParseError::ParserError{ msg: msg }));
                    break;
                }
            };
            match input_lex {
                Some(Ok(ref lex)) => {
                    // TODO: Valid Lex from our input. Hand it off to the
                    // current parser and process the result.
                    result = Some(self.parse_lex(lex));
                },
                Some(Err(ref error)) => {
                    // TODO: Lexer error. Throw it up and out.
                    let msg = error.msg().to_string();
                    out = Some(Err(ParseError::LexerError { msg: msg }));
                    break;
                },
                None => {
                    // TODO: We didn't get a Lex from the input, which means the
                    // input is done. If there's any parse result waiting
                    // around, clean it up and return it or return an error.
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
