mod lexer;

fn main() {
    let lexer = lexer::Lexer::new(String::from("((abc def + ghi))"));
    for t in lexer {
        println!("token = {}", t);
    }
}
