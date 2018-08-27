/* parser/src/parsers/mod.rs
 * Eryn Wells <eryn@erynwells.me>
 */

mod bool;
mod list;
mod program;
mod sym;

pub use self::program::ProgramParser;

use std::fmt::Debug;
use sibillexer::Lex;
use sibiltypes::Obj;

#[derive(Debug)]
pub enum NodeParseResult {
    /// Continue parsing with this NodeParser. The passed in Lex was consumed.
    Continue,
    /// This NodeParser has completed its work and has produced the given Object as a result.
    Complete { obj: Obj },
    /// Push a new NodeParser onto the parsing stack and let that parser proceed with the current Lex.
    Push { next: Box<NodeParser> },
    /// There was an error parsing with the current Lex.
    Error { msg: String },
}

impl NodeParseResult {
    pub fn error(msg: String) -> NodeParseResult {
        NodeParseResult::Error { msg: msg }
    }
}

/// A `NodeParser` is responsible for parsing one particular thing in the Scheme parse tree.
/// Roughly, there should be one NodeParser for each type of object in `sibiltypes`. As the
/// top-level `Parser` object progresses through the stream of tokens, new NodeParsers are created
/// to handle the nodes it encounters.
pub trait NodeParser: Debug {
    /// Called on a NodeParser when a Lex is encountered in the input.
    fn parse(&mut self, lex: &Lex) -> NodeParseResult;

    /// Called on a NodeParser when None is encountered in the input.
    fn none(&mut self) -> NodeParseResult;

    /// Called on a NodeParser when a NodeParser created by this one returns an object via
    /// `NodeParseResult::Complete`.
    fn subparser_completed(&mut self, obj: Obj) -> NodeParseResult;
}
