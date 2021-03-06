/* parser/src/sym_parser.rs
 * Eryn Wells <eryn@erynwells.me>
 */

use sibillexer::{Lex, Token};
use sibiltypes::{Obj, Sym};
use parsers::{NodeParser, NodeParseResult};

#[derive(Debug)] pub struct SymParser;

impl NodeParser for SymParser {
    fn parse(&mut self, lex: &Lex) -> NodeParseResult {
        match lex.token() {
            Token::Id => {
                let value = String::from(lex.value());
                // Initializing with Sym(value) caused E0423. So use this isntead.
                let obj = Obj::new(Sym::new(value));
                NodeParseResult::Complete { obj: obj }
            }
            _ => {
                let msg = format!("Expected symbol, found {:?}", lex);
                NodeParseResult::error(msg)
            }
        }
    }

    fn none(&mut self) -> NodeParseResult {
        let msg = format!("Expected symbol, found EOF");
        NodeParseResult::error(msg)
    }

    fn subparser_completed(&mut self, obj: Obj) -> NodeParseResult {
        let msg = format!("Unexpected subparser result: {}", obj);
        NodeParseResult::error(msg)
    }
}
