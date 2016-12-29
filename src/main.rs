mod lexer;
mod parser;

fn main() {
    let lexer = lexer::Lexer::new("((abc def + ghi #f))");
    for t in lexer {
        println!("token = {:?}", t);
    }
}
