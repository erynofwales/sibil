/* parser/src/parsers/list.rs
 * Eryn Wells <eryn@erynwells.me>
 */

use sibillexer::{Lex, Token};
use sibiltypes::{Obj, Pair};
use parsers::{NodeParser, NodeParseResult};
use parsers::sym::SymParser;

#[derive(Debug)]
pub struct ListParser {
    list: Option<Pair>
}

impl ListParser {
    pub fn new() -> ListParser {
        ListParser {
            list: None
        }
    }
}

impl NodeParser for ListParser {
    fn parse(&mut self, lex: &Lex) -> NodeParseResult {
        match lex.token() {
            Token::LeftParen => {
                match self.list {
                    None => {
                        // Create our empty pair and proceed parsing this list.
                        self.list = Some(Pair::empty());
                        NodeParseResult::Continue
                    },
                    Some(_) => {
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
                match self.list {
                    None => {
                        let msg = format!("Found right paren without matching left paren");
                        NodeParseResult::error(msg)
                    },
                    Some(_) => {
                        let taken = self.list.take().unwrap();
                        // TODO: If the cdr is Null, fill it in with an empty pair.
                        NodeParseResult::Complete { obj: Obj::new(taken) }
                    }
                }
            }
        }
    }

    fn none(&mut self) -> NodeParseResult {
        let msg = format!("Unmatched paren, found EOF");
        NodeParseResult::error(msg)
    }

    fn subparser_completed(&mut self, obj: Obj) -> NodeParseResult {
        if let Some(ref mut list) = self.list {
            match list.car {
                Obj::Null => {
                    list.car = obj;
                },
                Obj::Ptr(_) => {
                    let pair = Pair::with_car(obj);
                    list.cdr = Obj::new(pair);
                }
            }
            NodeParseResult::Continue
        } else {
            let msg = format!("what happened here???");
            NodeParseResult::error(msg)
        }
    }
}
