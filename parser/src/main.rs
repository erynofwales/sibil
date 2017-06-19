/* parser/src/main.rs
 * Eryn Wells <eryn@erynwells.me>
 */

extern crate sibillexer;
extern crate sibilparser;

use sibillexer::Lexer;
use sibilparser::Parser;

fn main() {
    let lexer = Lexer::new("(ab)".chars());
    let parser = Parser::new(lexer);
    for thing in parser { }
}
