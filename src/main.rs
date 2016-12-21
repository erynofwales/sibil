mod characters;
mod lexer;

fn main() {
    lexer::hello("Eryn");
    lexer::hello("Emily");
    let s = "Jonas".to_string();
    lexer::hello(&s);

    let lexer = lexer::Lexer::new(String::from("((abc))"));
    for t in lexer { }
}
