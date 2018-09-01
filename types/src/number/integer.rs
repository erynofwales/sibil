/* types/src/number/integer.rs
 * Eryn Wells <eryn@erynwells.me>
 */

use std::any::Any;
use number::Number;
use object::{Obj, Object};

pub type Int = i64;

impl Object for Int {
    fn as_any(&self) -> &Any { self }
}

impl Number for Int {
    fn as_int(&self) -> Option<&Int> { Some(self) }
}

impl PartialEq<Obj> for Int {
    fn eq<'a>(&self, rhs: &'a Obj) -> bool {
        match rhs.obj().and_then(Object::as_num) {
            Some(num) => self == num,
            None => false
        }
    }
}

impl<'a> PartialEq<Number + 'a> for Int {
    fn eq(&self, rhs: &(Number + 'a)) -> bool {
        match rhs.as_int() {
            Some(rhs) => *self == *rhs,
            None => false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Int;
    use number::*;
    use value::*;

    #[test]
    fn equal_integers_are_equal() {
        assert_eq!(Integer(3), Integer(3));
        assert_ne!(Integer(12), Integer(9));
        assert_eq!(Integer(4).as_value(), Integer(4).as_value());
        assert_ne!(Integer(5).as_value(), Integer(7).as_value());
    }

    #[test]
    fn integers_are_integers() {
        assert!(Integer(4).is_complex());
        assert!(Integer(4).is_real());
        assert!(Integer(4).is_rational());
        assert!(Integer(4).is_integer());
        assert!(Integer(4).is_number());
        assert!(!Integer(6).is_char());
        assert!(!Integer(6).is_bool());
    }

    #[test]
    fn integers_are_exact() {
        assert!(Integer(4).is_exact());
        assert!(!Integer(4).is_inexact());
    }
}
