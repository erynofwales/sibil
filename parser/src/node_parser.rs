/* node_parser.rs
 * Eryn Wells <eryn@erynwells.me>
 */

use std::fmt::Debug;
use sibillexer::{Lex, Token};
use sibiltypes::Obj;

#[derive(Debug)]
pub enum NodeParseResult {
    /// Continue parsing with this NodeParser. The passed in Lex was consumed.
    Continue,
    /// This NodeParser has completed its work and has produced the given Object
    /// as a result.
    Complete { obj: Obj },
    /// Push a new NodeParser onto the parsing stack and let that parser proceed
    /// with the current Lex.
    Push { next: Box<NodeParser> },
    /// There was an error parsing with the current Lex.
    Error { msg: String },
}

impl NodeParseResult {
    pub fn error(msg: String) -> NodeParseResult {
        NodeParseResult::Error { msg: msg }
    }
}

/// A `NodeParser` is responsible for parsing one particular thing in the Scheme
/// parse tree. Roughly, there should be one `XParser` for each variant of the
/// `sibiltypes::Object` enum. As the top-level `Parser` object progresses
/// through the stream of tokens, new NodeParsers are created to handle the
/// nodes it encounters.
pub trait NodeParser: Debug {
    fn parse(&mut self, lex: &Lex) -> NodeParseResult;
    /// Called on a NodeParser when None is encountered in the input.
    fn none(&mut self) -> NodeParseResult;
}
