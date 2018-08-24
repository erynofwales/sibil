/* parser/src/list_parser.rs
 * Eryn Wells <eryn@erynwells.me>
 */

use sibillexer::{Lex, Token};
use sibiltypes::{Obj, Pair};
use node_parser::{NodeParser, NodeParseResult};
use sym_parser::SymParser;

#[derive(Debug)]
pub struct ListParser {
    list: Obj
}

impl ListParser {
    pub fn new() -> ListParser {
        ListParser {
            list: Obj::Null
        }
    }
}

impl NodeParser for ListParser {
    fn parse(&mut self, lex: Lex) -> NodeParseResult {
        match lex.token() {
            Token::LeftParen => {
                self.list = Obj::new(Pair::empty());
                NodeParseResult::Continue
            },
            Token::Id => {
                let parser = SymParser{};
                NodeParseResult::Push { next: Box::new(parser) }
            }
            _ => NodeParseResult::Error { msg: "womp".to_string() }
        }
    }
}
