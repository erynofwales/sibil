/// # Token

use std::fmt;

pub enum Kind {
    LeftParen,
    RightParen,
    Identifier,
    Boolean,
}

pub struct Token {
    kind: Kind,
    value: String,
}

impl Token {
    pub fn new(kind: Kind, value: String) -> Token {
        Token { kind: kind, value: value, }
    }
}

//
// Display
//

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
            Kind::LeftParen => "LeftParen",
            Kind::RightParen => "RightParen",
            Kind::Identifier => "Identifier",
            Kind::Boolean => "Boolean",
        };
        write!(f, "{}", s)
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, \"{}\")", self.kind, self.value)
    }
}

