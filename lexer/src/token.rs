/* token.rs
 * Eryn Wells <eryn@erynwells.me>
 */

use sibiltypes::Object;

#[derive(Debug, PartialEq)]
pub enum Token {
    Boolean(Object),
    Character(Object),
    Comment(Object),
    Dot,
    Id(Object),
    LeftParen,
    LeftVectorParen,
    Number(Object),
    Quote,
    RightParen,
    String(Object),
}

/// A Lex is a Token extracted from a specific position in an input string. It contains useful
/// information about the token's place in that input.
#[derive(Debug)]
pub struct Lex {
    token: Token,
    line: usize,
    offset: usize,
}

impl Lex {
    pub fn new(token: Token, line: usize, offset: usize) -> Lex {
        Lex { token: token, line: line, offset: offset }
    }

    pub fn token(&self) -> &Token { &self.token }
    pub fn line(&self) -> usize { self.line }
    pub fn offset(&self) -> usize { self.offset }
}
