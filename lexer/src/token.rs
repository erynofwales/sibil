/* token.rs
 * Eryn Wells <eryn@erynwells.me>
 */

use sibiltypes::{Bool, Char, Number};

#[derive(Debug, PartialEq)]
pub enum Token {
    Boolean(Bool),
    Character(Char),
    Comment(String),
    Dot,
    Id(String),
    LeftParen,
    LeftVectorParen,
    Number(Number),
    Quote,
    RightParen,
    String(String),
}

/// A Lex is a Token extracted from a specific position in an input stream. It
/// contains useful information about the token's place.
#[derive(Debug)]
pub struct Lex {
    pub token: Token,
    pub line: usize,
    pub offset: usize,
}

impl Lex {
    pub fn new(token: Token, line: usize, offset: usize) -> Lex {
        Lex { token: token, line: line, offset: offset }
    }
}
