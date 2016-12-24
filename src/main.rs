mod characters;
mod lexer;

fn main() {
    let lexer = lexer::Lexer::new(String::from("((abc))"));
    for t in lexer {
        println!("token = {}", t);
    }
}
