//! # Lexer

use characters;

enum Kind {
    LeftParen,
    RightParen,
    Identifier,
}

struct Token {
    kind: Kind,
    value: String,
}

struct Lexer {
    input: str,
}

impl Lexer {
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        None
    }
}

pub fn hello(person: &str) {
    println!("Hello, {}!", person);
    for (idx, c) in person.char_indices() {
        println!("  {}, {} -> {}", c, idx, characters::ascii_letters().contains(&c));
    }
}
