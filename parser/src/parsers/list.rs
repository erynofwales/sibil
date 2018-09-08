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
    pairs: Option<Vec<Pair>>,
    waiting_for_final: bool,
}

impl ListParser {
    pub fn new() -> ListParser {
        ListParser {
            pairs: None,
            waiting_for_final: false,
        }
    }

    fn assemble(&mut self) -> Result<Obj, String> {
        match self.pairs.take() {
            Some(mut pairs) => {
                let last = pairs.last_mut().and_then(|p| Some(p.cdr.take())).unwrap_or(Obj::Null);
                let obj = pairs.into_iter().rfold(last, |acc, mut pair| {
                    pair.cdr = acc;
                    Obj::new(pair)
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
            },
            Token::Dot => {
                self.waiting_for_final = true;
                NodeParseResult::Continue
            },
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
            Token::Num(n) => {
                panic!("TODO: Handle numbrs.");
            },
            Token::Quote => {
                panic!("TODO: Handle quotes.");
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
            },
        }
    }

    fn none(&mut self) -> NodeParseResult {
        let msg = format!("Unmatched paren, found EOF");
        NodeParseResult::error(msg)
    }

    fn subparser_completed(&mut self, obj: Obj) -> NodeParseResult {
        match self.pairs {
            Some(ref mut pairs) if self.waiting_for_final => match pairs.last_mut() {
                Some(ref mut last) => {
                    last.cdr = obj;
                    // Waiting for RightParen to close list.
                    NodeParseResult::Continue
                },
                None => {
                    let msg = "Found dot before any pairs parsed".to_string();
                    NodeParseResult::error(msg)
                },
            },
            Some(ref mut pairs) => {
                pairs.push(Pair::with_car(obj));
                NodeParseResult::Continue
            },
            None => {
                let msg = "While attempting to parse list, found token before opening paren".to_string();
                NodeParseResult::error(msg)
            },
        }
    }
}
