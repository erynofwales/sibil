/* types/char.rs
 * Eryn Wells <eryn@erynwells.me>
 */

mod names;

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

impl Object {
    pub fn from_char(c: char) -> Object {
        Object::Char(c)
    }

    pub fn from_char_name(name: &str) -> Option<Object> {
        names::char_for(name).map(|c| Object::from_char(c))
    }
}

#[cfg(test)]
mod tests {
    use object::Object;
    use predicates::{IsBool, IsChar};

    #[test]
    fn chars_are_chars() {
        assert_eq!(Object::from_char('a').is_char(), true);
        assert_eq!(Object::from_char('a').is_bool(), false);
    }

    #[test]
    fn equal_chars_are_equal() {
        assert_eq!(Object::from_char('a'), Object::from_char('a'));
    }

    #[test]
    fn named_chars_are_created() {
        assert_eq!(Object::from_char_name("newline"), Some(Object::from_char('\n')));
        assert_eq!(Object::from_char_name("asdf"), None);
    }
}
