/* parser/src/program_parser.rs
 * Eryn Wells <eryn@erynwells.me>
 */

use sibillexer::{Lex, Token};
use sibiltypes::Obj;
use parsers::{NodeParser, NodeParseResult};
use parsers::bool::BoolParser;
use parsers::list::ListParser;
use parsers::sym::SymParser;

#[derive(Debug)]
pub struct ProgramParser;

impl ProgramParser {
    pub fn new() -> ProgramParser {
        ProgramParser { }
    }
}

impl NodeParser for ProgramParser {
    fn parse(&mut self, lex: &Lex) -> NodeParseResult {
        match lex.token() {
            Token::Bool(_) => {
                let next = Box::new(BoolParser{});
                NodeParseResult::Push { next }
            },
            Token::LeftParen => {
                let next = Box::new(ListParser::new());
                NodeParseResult::Push { next }
            },
            Token::RightParen => {
                let msg = format!("Expected symbol found {:?}", lex);
                NodeParseResult::error(msg)
            },
            Token::Id => {
                let parser = SymParser{};
                let parser = Box::new(parser);
                NodeParseResult::Push { next: parser }
            },
            _ => {
                panic!("unhandled symbol");
            },
        }
    }

    fn none(&mut self) -> NodeParseResult {
        NodeParseResult::Complete { obj: Obj::Null }
    }

    fn subparser_completed(&mut self, obj: Obj) -> NodeParseResult {
        NodeParseResult::Complete { obj: obj }
    }
}

