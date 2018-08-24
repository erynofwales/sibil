/* node_parser.rs
 * Eryn Wells <eryn@erynwells.me>
 */

use std::fmt::Debug;
use sibillexer::{Lex, Token};
use sibiltypes::{Object, ObjectPtr};

#[derive(Debug)]
pub enum NodeParseResult {
    /// Continue parsing with this NodeParser. The passed in Lex was consumed.
    Continue,
    /// This NodeParser has completed its work and has produced the given Object
    /// as a result.
    Complete { obj: ObjectPtr },
    /// Push a new NodeParser onto the parsing stack and let that parser proceed
    /// with the current Lex.
    Push { next: Box<NodeParser> },
    /// There was an error parsing with the current Lex.
    Error { msg: String },
}

/// A `NodeParser` is responsible for parsing one particular thing in the Scheme
/// parse tree. Roughly, there should be one `XParser` for each variant of the
/// `sibiltypes::Object` enum. As the top-level `Parser` object progresses
/// through the stream of tokens, new NodeParsers are created to handle the
/// nodes it encounters.
pub trait NodeParser: Debug {
    fn parse(&mut self, lex: Lex) -> NodeParseResult;
}

#[derive(Debug)]
pub struct ProgramParser {
}

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

#[derive(Debug)]
pub struct IdParser {
}

impl IdParser {
    pub fn new() -> IdParser {
        IdParser { }
    }
}

impl NodeParser for IdParser {
    fn parse(&mut self, lex: Lex) -> NodeParseResult {
        match lex.token() {
            Token::Id => {
                let value = String::from(lex.value());
                let obj = ObjectPtr::new(Object::Symbol(value));
                NodeParseResult::Complete { obj: obj }
            }
            _ => {
                let msg = String::from(format!("Invalid token: {:?}", lex));
                NodeParseResult::Error { msg: msg }
            }
        }
    }
}

#[derive(Debug)]
pub struct ListParser {
    list: ObjectPtr
}

impl ListParser {
    pub fn new() -> ListParser {
        ListParser {
            list: ObjectPtr::Null
        }
    }
}

impl NodeParser for ListParser {
    fn parse(&mut self, lex: Lex) -> NodeParseResult {
        NodeParseResult::Error { msg: "womp".to_string() }
    }
}
