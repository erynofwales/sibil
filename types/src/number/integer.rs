/* types/src/number/integer.rs
 * Eryn Wells <eryn@erynwells.me>
 */

use std::any::Any;
use value::*;
use super::*;

#[derive(Debug, Eq, PartialEq)]
pub struct Integer(pub Int);

impl Number for Integer {
    fn convert_down(&self) -> Option<Box<Number>> { None }

    fn is_exact(&self) -> bool { true }
}

impl Value for Integer {
    fn as_value(&self) -> &Value { self }
}

impl IsBool for Integer { }
impl IsChar for Integer { }

impl IsNumber for Integer {
    fn is_integer(&self) -> bool { true }
}

impl ValueEq for Integer {
    fn eq(&self, other: &Value) -> bool {
        other.as_any().downcast_ref::<Self>().map_or(false, |x| x == self)
    }

    fn as_any(&self) -> &Any { self }
}

#[cfg(test)]
mod tests {
    use super::Integer;
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
