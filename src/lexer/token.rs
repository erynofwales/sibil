/// # Token

#[derive(Debug)]
pub enum Kind {
    LeftParen,
    RightParen,
    Identifier,
    Boolean,
}

#[derive(Debug)]
pub struct Token {
    kind: Kind,
    value: String,
}

impl Token {
    pub fn new(kind: Kind, value: String) -> Token {
        Token { kind: kind, value: value, }
    }
}
