/* types/char.rs
 * Eryn Wells <eryn@erynwells.me>
 */

mod names;

use object::Object;
use predicates::IsChar;

pub trait Char {
    fn from_char(c: char) -> Object;
    fn from_char_named(name: &str) -> Option<Object>;
    fn char_name(&self) -> Option<String>;
}

impl IsChar for Object {
    fn is_char(&self) -> bool {
        match *self {
            Object::Char(_) => true,
            _ => false,
        }
    }
}

impl Char for Object {
    fn from_char(c: char) -> Object {
        Object::Char(c)
    }

    fn from_char_named(name: &str) -> Option<Object> {
        names::char_for(name).map(|c| Object::from_char(c))
    }

    fn char_name(&self) -> Option<String> {
        match *self {
            Object::Char(c) => Some(names::name_of(c).map_or(c.to_string(), String::from)),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use object::Object;
    use predicates::{IsBool, IsChar};
    use super::Char;

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
        assert_eq!(Object::from_char_named("newline"), Some(Object::from_char('\n')));
        assert_eq!(Object::from_char_named("asdf"), None);
    }

    #[test]
    fn chars_have_names() {
        assert_eq!(Object::from_char('a').char_name(), Some(String::from("a")));
        assert_eq!(Object::from_char('\n').char_name(), Some(String::from("newline")));
        assert_eq!(Object::Bool(true).char_name(), None);
    }
}
