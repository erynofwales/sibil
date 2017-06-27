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

#[derive(Debug, Eq, PartialEq)]
pub enum Token { LeftParen, RightParen, Id, }

impl Lex {
    pub fn new(token: Token, value: &str, line: usize, offset: usize) -> Lex {
        Lex {
            token: token,
            value: String::from(value),
            line: line,
            offset: offset,
        }
    }
}
