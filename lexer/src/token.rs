/* lexer/src/token.rs
 * Eryn Wells <eryn@erynwells.me>
 */

#[derive(Debug, Eq, PartialEq)]
pub struct Lex {
    token: Token,
    value: String,
    line: usize,
    offset: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Token {
    Bool(bool),
    Num(i64),
    LeftParen,
    RightParen,
    Id
}

impl Lex {
    pub fn new(token: Token, value: &str, line: usize, offset: usize) -> Lex {
        Lex {
            token: token,
            value: String::from(value),
            line: line,
            offset: offset,
        }
    }

    pub fn token(&self) -> Token { self.token }
    pub fn value(&self) -> &str { self.value.as_str() }
}
