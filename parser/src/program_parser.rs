/* parser/src/program_parser.rs
 * Eryn Wells <eryn@erynwells.me>
 */

use std::fmt::Debug;
use sibillexer::{Lex, Token};
use node_parser::{NodeParser, NodeParseResult};

#[derive(Debug)]
pub struct ProgramParser;

impl ProgramParser {
    pub fn new() -> ProgramParser {
        ProgramParser { }
    }
}

impl NodeParser for ProgramParser {
    fn parse(&mut self, lex: Lex) -> NodeParseResult {
        NodeParseResult::Error { msg: "womp".to_string() }
    }
}

