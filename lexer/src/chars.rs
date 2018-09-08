/* lexer/src/chars.rs
 * Eryn Wells <eryn@erynwells.me>
 */

pub trait Lexable {
    fn is_dot(&self) -> bool;
    fn is_exactness(&self) -> bool;
    fn is_hash(&self) -> bool;
    fn is_identifier_delimiter(&self) -> bool;
    fn is_identifier_initial(&self) -> bool;
    fn is_identifier_subsequent(&self) -> bool;
    fn is_left_paren(&self) -> bool;
    fn is_quote(&self) -> bool;
    fn is_radix(&self) -> bool;
    fn is_right_paren(&self) -> bool;
}

impl Lexable for char {
    fn is_dot(&self) -> bool {
        *self == '.'
    }

    fn is_exactness(&self) -> bool {
        *self == 'i' || *self == 'e'
    }

    fn is_left_paren(&self) -> bool {
        *self == '('
    }

    fn is_right_paren(&self) -> bool {
        *self == ')'
    }

    fn is_identifier_initial(&self) -> bool {
        self.is_alphabetic() || self.is_special_initial()
    }

    fn is_identifier_subsequent(&self) -> bool {
        self.is_identifier_initial() || self.is_numeric() || self.is_special_subsequent()
    }

    fn is_identifier_delimiter(&self) -> bool {
        self.is_whitespace() || self.is_left_paren() || self.is_right_paren()
    }

    fn is_quote(&self) -> bool {
        *self == '\''
    }

    fn is_radix(&self) -> bool {
        let radishes = &['b', 'd', 'o', 'x'];
        radishes.contains(self)
    }

    fn is_hash(&self) -> bool {
        *self == '#'
    }
}

trait LexableSpecial {
    fn is_special_initial(&self) -> bool;
    fn is_special_subsequent(&self) -> bool;
    fn is_explicit_sign(&self) -> bool;
}

impl LexableSpecial for char {
    fn is_special_initial(&self) -> bool {
        "!$%&*/:<=>?~_^".contains(*self)
    }

    fn is_special_subsequent(&self) -> bool {
        self.is_explicit_sign() || ".@".contains(*self)
    }

    fn is_explicit_sign(&self) -> bool {
        *self == '+' || *self == '-'
    }
}
