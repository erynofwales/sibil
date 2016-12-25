/* token.rs
 * Eryn Wells <eryn@erynwells.me>
 */

#[derive(Debug)]
pub enum Token {
    LeftParen(String),
    RightParen(String),
    Identifier(String),
    Boolean(bool),
    Comment(String),
}

/// A Lex is a Token extracted from a specific position in an input. It contains useful information about the token's
/// place.
#[derive(Debug)]
pub struct Lex {
    token: Token,
}

impl Lex {
    pub fn new(token: Token) -> Lex {
        Lex { token: token }
    }
}
