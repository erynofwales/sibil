//! # Lexer

mod characters {
    use std::collections::HashSet;
    use std::iter::FromIterator;
    
    pub type CharSet = HashSet<char>;

    pub fn ascii_letters() -> CharSet {
        let letters = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars();
        return CharSet::from_iter(letters);
    }

    pub fn ascii_digits() -> CharSet {
        let digits = "1234567890".chars();
        return CharSet::from_iter(digits);
    }

    pub fn identifier_initials() -> CharSet {
        let letters = ascii_letters();
        let extras = CharSet::from_iter("!$%&*/:<=>?~_^".chars());
        let mut initials = CharSet::new();
        initials.extend(letters.iter());
        initials.extend(extras.iter());
        return initials;
    }

    pub fn identifier_subsequents() -> CharSet {
        let initials = identifier_initials();
        let digits = ascii_digits();
        let extras = CharSet::from_iter(".+-".chars());
        let mut subsequents = CharSet::new();
        subsequents.extend(initials.iter());
        subsequents.extend(digits.iter());
        subsequents.extend(extras.iter());
        return subsequents;
    }
}

enum Kind {
    LeftParen,
    RightParen,
    Identifier,
}

struct Token {
    kind: Kind,
    value: String,
}

pub fn hello(person: &str) {
    println!("Hello, {}!", person);
    for (idx, c) in person.char_indices() {
        println!("  {}, {} -> {}", c, idx, characters::ascii_letters().contains(&c));
    }
}
