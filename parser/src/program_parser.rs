/* parser/src/program_parser.rs
 * Eryn Wells <eryn@erynwells.me>
 */

use sibillexer::{Lex, Token};
use list_parser::ListParser;
use node_parser::{NodeParser, NodeParseResult};
use sym_parser::SymParser;

#[derive(Debug)]
pub struct ProgramParser;

impl ProgramParser {
    pub fn new() -> ProgramParser {
        ProgramParser { }
    }
}

impl NodeParser for ProgramParser {
    fn parse(&mut self, lex: Lex) -> NodeParseResult {
        match lex.token() {
            Token::LeftParen => {
                let parser = ListParser::new();
                let parser = Box::new(parser);
                NodeParseResult::Push { next: parser }
            },
            Token::RightParen => {
                let msg = format!("Expected symbol found {:?}", lex);
                NodeParseResult::error(msg)
            },
            Token::Id => {
                let parser = SymParser{};
                let parser = Box::new(parser);
                NodeParseResult::Push { next: parser }
            }
        }
    }
}

