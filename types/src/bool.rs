/* types/src/bool.rs
 * Eryn Wells <eryn@erynwells.me>
 */

use object::Object;
use predicates::IsBool;

impl IsBool for Object {
    fn is_bool(&self) -> bool {
        match *self {
            Object::Bool(_) => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use object::Object;
    use predicates::{IsBool, IsChar};

    #[test]
    fn bools_are_bools() {
        assert_eq!(Object::Bool(false).is_bool(), true);
        assert_eq!(Object::Bool(false).is_char(), false);
    }

    #[test]
    fn equal_bools_are_equal() {
        assert_eq!(Object::Bool(true), Object::Bool(true));
        assert_eq!(Object::Bool(false), Object::Bool(false));
        assert_ne!(Object::Bool(true), Object::Bool(false));
    }
}
