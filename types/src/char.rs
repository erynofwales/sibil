/* types/char.rs
 * Eryn Wells <eryn@erynwells.me>
 */

use object::Object;
use predicates::IsChar;

impl IsChar for Object {
    fn is_char(&self) -> bool {
        match *self {
            Object::Char(_) => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use object::Object;
    use predicates::{IsBool, IsChar};

    #[test]
    fn chars_are_chars() {
        assert_eq!(Object::Char('a').is_char(), true);
        assert_eq!(Object::Char('a').is_bool(), false);
    }

    #[test]
    fn equal_chars_are_equal() {
        assert_eq!(Object::Char('a'), Object::Char('a'));
    }
}
