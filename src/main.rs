mod lexer;

fn main() {
    let lexer = lexer::Lexer::new(String::from("((abc def + ghi #f))"));
    for t in lexer {
        println!("token = {:?}", t);
    }
}
