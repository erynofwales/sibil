mod lexer;
mod parser;
mod types;

use lexer::lex;

fn main() {
    let lexer = lex("((abc def + ghi #f))");
    for t in lexer {
        println!("token = {:?}", t);
    }
}
