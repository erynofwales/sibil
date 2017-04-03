mod lexer;
mod types;

use lexer::lex;

fn main() {
    let lexer = lex("(#f (abc def + ghi #f))");
    for lex in lexer {
        println!("{:?}", lex);
    }
}
