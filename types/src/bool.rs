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
    use super::Bool;
    use value::*;

    #[test]
    fn equal_bools_are_equal() {
        assert_eq!(Bool(true), Bool(true));
        assert_eq!(Bool(false), Bool(false));
        assert_ne!(Bool(true), Bool(false));

        assert_eq!(Bool(true).as_value(), Bool(true).as_value());
        assert_ne!(Bool(true).as_value(), Bool(false).as_value());
    }

    #[test]
    fn bools_are_bools() {
        assert_eq!(Bool(false).is_bool(), true);
        assert_eq!(Bool(false).is_char(), false);
        assert_eq!(Bool(false).is_number(), false);
    }
}
