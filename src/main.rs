mod lexer;
mod parser;
mod types;

use lexer::lex;
use parser::Parser;

fn main() {
    let lexer = lex("(#f (abc def + ghi #f))");
    let mut parser = Parser::new(lexer);
    let program = parser.parse();
}
