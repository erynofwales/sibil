/* parser/src/lib.rs
 * Eryn Wells <eryn@erynwells.me>
 */

extern crate sibillexer;
extern crate sibiltypes;

mod list;
mod program;

use sibillexer::Lexer;

struct ParseError { }

type Result<T> = std::result::Result<T, ParseError>;

trait Parsable: Sized {
    fn parse(lexer: &Lexer) -> Result<Self>;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
