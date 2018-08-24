/* parser/src/sym_parser.rs
 * Eryn Wells <eryn@erynwells.me>
 */

use sibillexer::{Lex, Token};
use sibiltypes::{Obj, Sym};
use node_parser::{NodeParser, NodeParseResult};

#[derive(Debug)]
pub struct SymParser;

impl NodeParser for SymParser {
    fn parse(&mut self, lex: Lex) -> NodeParseResult {
        match lex.token() {
            Token::Id => {
                let value = String::from(lex.value());
                // Initializing with Sym(value) caused E0423. So use this isntead.
                let obj = Obj::new(Sym::new(value));
                NodeParseResult::Complete { obj: obj }
            }
            _ => {
                let msg = format!("Invalid token: {:?}", lex);
                NodeParseResult::error(msg)
            }
        }
    }
}
