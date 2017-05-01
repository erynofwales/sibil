/* lexer/src/main.rs
 * Eryn Wells <eryn@erynwells.me>
 */

extern crate sibillexer;

use sibillexer::Lexer;

fn main() {
    let lexer = Lexer::new("(ab (cd) ef)".chars());
    for tok in lexer {
        println!("found {:?}", tok.unwrap());
    }
}
