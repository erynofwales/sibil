/* token.rs
 * Eryn Wells <eryn@erynwells.me>
 */

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Token {
    LeftParen(String),
    LeftVectorParen,
    RightParen(String),
    Dot,
    Identifier(String),
    Boolean(bool),
    String(String),
    Comment(String),
}

/// A Lex is a Token extracted from a specific position in an input. It contains useful information about the token's
/// place.
#[derive(Debug)]
pub struct Lex {
    pub token: Token,
}

impl Lex {
    pub fn new(token: Token) -> Lex {
        Lex { token: token }
    }
}
