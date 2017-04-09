/* types/src/number/rational.rs
 * Eryn Wells <eryn@erynwells.me>
 */

use std::any::Any;
use value::*;
use super::*;

#[derive(Debug, Eq, PartialEq)]
pub struct Rational(pub Int, pub Int);

impl Number for Rational {
    fn convert_down(&self) -> Option<Box<Number>> {
        if self.1 == 1 {
            Some(Box::new(Integer(self.0)))
        }
        else {
            None
        }
    }
}

impl Value for Rational {
    fn as_value(&self) -> &Value { self }
}

impl IsBool for Rational { }
impl IsChar for Rational { }

impl IsNumber for Rational {
    fn is_rational(&self) -> bool { true }
}

impl ValueEq for Rational {
    fn eq(&self, other: &Value) -> bool {
        other.as_any().downcast_ref::<Self>().map_or(false, |x| x == self)
    }

    fn as_any(&self) -> &Any { self }
}

#[cfg(test)]
mod tests {
    use std::ops::Deref;
    use number::*;
    use value::*;

    #[test]
    fn equal_rationals_are_equal() {
        assert_eq!(Rational(3, 2), Rational(3, 2));
        assert_ne!(Rational(12, 4), Rational(9, 7));
        assert_eq!(Rational(4, 5).as_value(), Rational(4, 5).as_value());
        assert_ne!(Rational(5, 6).as_value(), Rational(7, 6).as_value());
    }

    #[test]
    fn rationals_are_rationals() {
        assert_eq!(Rational(4, 3).is_rational(), true);
        assert_eq!(Rational(4, 3).is_integer(), false);
        assert_eq!(Rational(4, 3).is_number(), true);
        assert_eq!(Rational(6, 8).is_char(), false);
        assert_eq!(Rational(6, 9).is_bool(), false);
    }

    #[test]
    fn rationals_should_reduce_to_integers_where_possible() {
        let rational_as_integer = Rational(3, 1).convert_down();
        assert!(rational_as_integer.is_some());
        // Oh my god this line is so dumb.
        let rational_as_integer = rational_as_integer.unwrap();
        let rational_as_integer = rational_as_integer.as_value();
        assert_eq!(rational_as_integer.deref(), Integer(3).as_value());
    }

    #[test]
    fn rationals_should_not_reduce_to_integers_where_impossible() {
        let rational_as_integer = Rational(3, 2).convert_down();
        assert!(rational_as_integer.is_none());
    }
}
