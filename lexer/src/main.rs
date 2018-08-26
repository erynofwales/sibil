/* lexer/src/main.rs
 * Eryn Wells <eryn@erynwells.me>
 */

extern crate sibillexer;

use std::io::prelude::*;
use std::io;
use sibillexer::Lexer;

fn main() {
    loop {
        // Print a prompt.
        print!("> ");
        io::stdout().flush().ok().expect("couldn't flush");

        // Read a line from stdin.
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        // Create a lexer and parser and process the input.
        let lexer = Lexer::new(input.chars());

        // Print the parser's output.
        for thing in lexer {
            println!("{:?}", thing);
        }
        println!();
    }
}
