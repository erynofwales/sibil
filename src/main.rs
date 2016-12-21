mod characters;
mod lexer;

fn main() {
    lexer::hello("Eryn");
    lexer::hello("Emily");
    let s = "Jonas".to_string();
    lexer::hello(&s);
}
