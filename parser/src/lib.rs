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
        Parser {
            input: input.peekable(),
            parsers: vec![]
        }
    }

    fn prepare(&mut self) {
        assert_eq!(self.parsers.len(), 0);
        let program_parser = Box::new(ProgramParser::new());
        self.push_parser(program_parser);
    }

    fn parse_lex(&mut self, lex: &Lex) -> NodeParseResult {
        let parser = self.parsers.last_mut().expect("couldn't get a parser -- this is unexpected");
        parser.parse(lex)
    }

    fn parse_none(&mut self) -> NodeParseResult {
        let parser = self.parsers.last_mut().expect("couldn't get a parser -- this is unexpected");
        parser.none()
    }

    fn pop_parser(&mut self) {
        let popped = self.parsers.pop();
        println!("popped parser stack --> {:?}", self.parsers);
    }

    fn push_parser(&mut self, next: Box<NodeParser>) {
        self.parsers.push(next);
        println!("pushed onto parser stack -> {:?}", self.parsers);
    }

    fn next_lex(&mut self) -> Option<T::Item> {
        let next = self.input.next();
        println!("next lex: {:?}", next);
        next
    }
}

impl<T> Iterator for Parser<T> where T: Iterator<Item=LexerResult> {
    type Item = Result;

    fn next(&mut self) -> Option<Self::Item> {
        let out: Option<Self::Item>;
        let mut result: Option<NodeParseResult> = None;
        let mut input_lex: Option<T::Item> = None;
        loop {
            input_lex = match result {
                None => {
                    let next_lex = self.next_lex();
                    if next_lex.is_none() {
                        // First run through the loop and our input has nothing to give.
                        out = None;
                        break;
                    } else {
                        // Prepare for parsing!
                        self.prepare();
                    }
                    next_lex
                }
                Some(NodeParseResult::Continue) => self.next_lex(),
                Some(NodeParseResult::Complete{ obj }) => {
                    println!("{:?} completed with {:?}", self.parsers.last().unwrap(), obj);
                    self.pop_parser();
                    if self.parsers.len() == 0 && input_lex.is_none() {
                        // We are done.
                        println!("we are done");
                        out = Some(Ok(obj));
                        break;
                    } else {
                        let prev_parser = self.parsers.last_mut().unwrap();
                        prev_parser.subparser_completed(obj);
                        // TODO: Handle the result from above.
                    }
                    println!("parsers {:?}", self.parsers);
                    self.next_lex()
                },
                Some(NodeParseResult::Push{ next }) => {
                    // Push the next parser on and give it a shot at the current token.
                    self.push_parser(next);
                    input_lex
                },
                Some(NodeParseResult::Error{ msg }) => {
                    out = Some(Err(ParseError::ParserError{ msg }));
                    break;
                }
            };
            match input_lex {
                Some(Ok(ref lex)) => {
                    // TODO: Valid Lex from our input. Hand it off to the current parser and process the result.
                    result = Some(self.parse_lex(lex));
                },
                Some(Err(ref error)) => {
                    // Lexer error. Throw it up and out.
                    let msg = error.msg().to_string();
                    out = Some(Err(ParseError::LexerError { msg }));
                    break;
                },
                None => {
                    // We didn't get a Lex from the input, which means the input
                    // is done. If there's any parse result waiting around,
                    // clean it up and return it or return an error.
                    result = Some(self.parse_none());
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
