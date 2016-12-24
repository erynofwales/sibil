/// # Token

use std::fmt;

pub enum Kind {
    LeftParen,
    RightParen,
    Identifier,
}

pub struct Token {
    kind: Kind,
    value: String,
}

impl Token {
    fn new(kind: Kind, value: String) -> Token {
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
        };
        write!(f, "{}", s)
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, \"{}\")", self.kind, self.value)
    }
}

