/* parser/src/lib.rs
 * Eryn Wells <eryn@erynwells.me>
 */

extern crate sibillexer;
extern crate sibiltypes;

mod program;

use sibillexer::Lexer;
use sibiltypes::Object;

struct ParseError { }

type Result = std::result::Result<Object, ParseError>;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
