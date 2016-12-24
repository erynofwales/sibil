//! # Lexer

use std::fmt;

use characters;
use characters::RelativeIndexable;

pub enum Kind {
    LeftParen,
    RightParen,
    Identifier,
}

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

pub struct Token {
    kind: Kind,
    value: String,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, \"{}\")", self.kind, self.value)
    }
}

enum State {
    Initial,
    Identifier,
}

pub struct Lexer {
    input: String,
    begin: usize,
    forward: usize,
    state: State,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        Lexer {
            input: input,
            begin: 0,
            forward: 0,
            state: State::Initial,
        }
    }
}

impl Lexer {
    fn begin_lexing(&mut self) {
        self.forward = self.begin;
        self.state = State::Initial;
    }

    /// Advance the forward pointer to the next character.
    fn advance(&mut self) {
        if let Some(next) = self.input.index_after(&self.forward) {
            self.forward = next;
        }
    }

    /// Retract the forward pointer to the previous character.
    fn retract(&mut self) {
        if let Some(prev) = self.input.index_before(&self.forward) {
            self.forward = prev;
        }
    }
}

impl Lexer {
    fn state_initial(&mut self) {
        println!("Initial!");
    }

    fn state_identifier(&mut self) {
        println!("Identifier!")
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        self.begin_lexing();
        let mut emit = false;
        println!("Lexing '{}'", self.input);
        while !emit {
            match self.state {
                State::Initial => self.state_initial(),
                State::Identifier => self.state_identifier(),
            }
            emit = true;
        }
        None
    }
}

pub fn hello(person: &str) {
    println!("Hello, {}!", person);
    for (idx, c) in person.char_indices() {
        println!("  {}, {} -> {}", c, idx, characters::identifier_initials().contains(&c));
    }
}
