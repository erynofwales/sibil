/* parser/src/parsers/bool.rs
 * Eryn Wells <eryn@erynwells.me>
 */

use sibillexer::{Lex, Token};
use sibiltypes::{Bool, Obj};
use parsers::{NodeParser, NodeParseResult};

#[derive(Debug)] pub struct BoolParser;

impl NodeParser for BoolParser {
    fn parse(&mut self, lex: &Lex) -> NodeParseResult {
        match lex.token() {
            Token::Bool(value) => {
                NodeParseResult::Complete { obj: Obj::new(Bool::from(value)) }
            }
            _ => {
                let msg = format!("Expected bool, found {:?}", lex);
                NodeParseResult::error(msg)
            }
        }
    }

    fn none(&mut self) -> NodeParseResult {
        let msg = format!("Expected bool, found EOF");
        NodeParseResult::error(msg)
    }

    fn subparser_completed(&mut self, obj: Obj) -> NodeParseResult {
        let msg = format!("Unexpected subparser result: {}", obj);
        NodeParseResult::error(msg)
    }
}
