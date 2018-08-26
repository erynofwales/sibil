/* parser/src/main.rs
 * Eryn Wells <eryn@erynwells.me>
 */

extern crate sibillexer;
extern crate sibilparser;

use std::io::prelude::*;
use std::io;
use sibillexer::Lexer;
use sibilparser::Parser;

fn main() {
    loop {
        // Print a prompt.
        print!("> ");
        io::stdout().flush().ok().expect("couldn't flush");

        // Read a line from stdin.
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        // Remove the trailing newline.
        input.pop();

        // Create a lexer and parser and process the input.
        let lexer = Lexer::new(input.chars());
        let parser = Parser::new(lexer);

        // Print the parser's output.
        for thing in parser {
            println!("{:?}", thing);
        }
        println!();
    }
}
