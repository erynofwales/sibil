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
    fn parse(&mut self, lex: &Lex) -> NodeParseResult {
        match lex.token() {
            Token::LeftParen => {
                match self.list {
                    Obj::Null => {
                        // Create our empty pair and proceed parsing this list.
                        self.list = Obj::new(Pair::empty());
                        NodeParseResult::Continue
                    },
                    Obj::Ptr(_) => {
                        // This is an embedded list. Create a new parser for it.
                        let parser = ListParser::new();
                        NodeParseResult::Push { next: Box::new(parser) }
                    }
                }
            },
            Token::Id => {
                let parser = SymParser{};
                NodeParseResult::Push { next: Box::new(parser) }
            },
            Token::RightParen => {
                NodeParseResult::Complete { obj: self.list.take() }
            }
        }
    }

    fn none(&mut self) -> NodeParseResult {
        let msg = format!("Unmatched paren, found EOF");
        NodeParseResult::Error { msg }
    }
}
