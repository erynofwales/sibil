//! # Lexer

use characters;

pub enum Kind {
    LeftParen,
    RightParen,
    Identifier,
}

pub struct Token {
    kind: Kind,
    value: String,
}

pub struct Lexer {
    input: String,
    index: usize,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        Lexer { input: input, index: 0 }
    }
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
        println!("  {}, {} -> {}", c, idx, characters::identifier_initials().contains(&c));
    }
}
