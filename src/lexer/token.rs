/* token.rs
 * Eryn Wells <eryn@erynwells.me>
 */

use lexer::number::Number;

#[derive(PartialEq, Debug)]
pub enum Token {
    Boolean(bool),
    Comment(String),
    Dot,
    Identifier(String),
    LeftParen(String),
    LeftVectorParen,
    Number(Number),
    RightParen(String),
    String(String),
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
