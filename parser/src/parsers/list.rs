/* parser/src/parsers/list.rs
 * Eryn Wells <eryn@erynwells.me>
 */

use sibillexer::{Lex, Token};
use sibiltypes::{Obj, Pair};
use parsers::{NodeParser, NodeParseResult};
use parsers::bool::BoolParser;
use parsers::sym::SymParser;

#[derive(Debug)]
pub struct ListParser {
    pairs: Option<Vec<Pair>>
}

impl ListParser {
    pub fn new() -> ListParser {
        ListParser { pairs: None }
    }

    fn assemble(&mut self) -> Result<Obj, String> {
        match self.pairs.take() {
            Some(pairs) => {
                let obj = pairs.into_iter().rfold(Obj::Null, |acc, mut pair| {
                    pair.cdr = acc;
                    Obj::Ptr(Box::new(pair))
                });
                Ok(obj)
            },
            None => Err("bad".to_string())
        }
    }
}

impl NodeParser for ListParser {
    fn parse(&mut self, lex: &Lex) -> NodeParseResult {
        match lex.token() {
            Token::Bool(_) => {
                let parser = BoolParser{};
                NodeParseResult::Push { next: Box::new(parser) }
            }
            Token::LeftParen => {
                match self.pairs {
                    None => {
                        // Create our empty pair and proceed parsing this list.
                        self.pairs = Some(Vec::new());
                        NodeParseResult::Continue
                    },
                    Some(_) => {
                        // This is an embedded list. Create a new parser for it.
                        let next = Box::new(ListParser::new());
                        NodeParseResult::Push { next }
                    }
                }
            },
            Token::Id => {
                let next = Box::new(SymParser{});
                NodeParseResult::Push { next }
            },
            Token::RightParen => {
                match self.pairs {
                    None => {
                        let msg = format!("Found right paren without matching left paren");
                        NodeParseResult::error(msg)
                    },
                    Some(_) => {
                        let list = self.assemble();
                        match list {
                            Ok(list) => NodeParseResult::Complete { obj: list },
                            Err(msg) => NodeParseResult::error(msg)
                        }
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
        if let Some(ref mut pairs) = self.pairs {
            pairs.push(Pair::with_car(obj));
            NodeParseResult::Continue
        } else {
            let msg = format!("what happened here???");
            NodeParseResult::error(msg)
        }
    }
}
