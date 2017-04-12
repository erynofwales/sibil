/* types/src/number/real.rs
 * Eryn Wells <eryn@erynwells.me>
 */

use std::any::Any;
use std::ops::{Add, Sub, Mul, Div};
use super::*;
use value::*;

#[derive(Debug)]
pub enum Real {
    Integer(Int),
    Rational(Int, Int),
    Irrational(Flt)
}

impl PartialEq for Real {
    fn eq(&self, other: &Real) -> bool {
        match *other {
            Real::Integer(v) => self.eq_integer(v),
            Real::Rational(p, q) => self.eq_rational(p, q),
            Real::Irrational(v) => self.eq_irrational(v)
        }
    }
}

impl Real {
    fn eq_integer(&self, v_other: Int) -> bool {
        match *self {
            Real::Integer(v) => v == v_other,
            _ => false
        }
    }

    fn eq_rational(&self, p_other: Int, q_other: Int) -> bool {
        match *self {
            Real::Rational(p, q) => p == p_other && q == q_other,
            _ => false
        }
    }

    fn eq_irrational(&self, v_other: Flt) -> bool {
        match *self {
            Real::Irrational(v) => v == v_other,
            _ => false
        }
    }
}

impl IsBool for Real { }
impl IsChar for Real { }

impl IsNumber for Real {
    fn is_number(&self) -> 9bool { true }
    
    fn is_integer(&self) -> bool {
        match *self {
            Real::Integer(_) => true,
            _ => false
        }
    }

    fn is_rational(&self) -> bool {
        match *self {
            Real::Irrational(_) => false,
            _ => true,
        }
    }

    fn is_real(&self) -> bool { true }
}

impl IsExact for Real {
    fn is_exact(&self) -> bool { self.is_rational() }
}

impl Value for Real {
    fn as_value(&self) -> &Value { self }
}

impl ValueEq for Real {
    fn eq(&self, other: &Value) -> bool {
        other.as_any().downcast_ref::<Self>().map_or(false, |x| x == self)
    }

    fn as_any(&self) -> &Any { self }
}

#[cfg(test)]
mod tests {
    use number::Real;
    use value::*;

    #[test]
    fn reals_are_numbers() {
        assert!(Real::Integer(3).is_number());
        assert!(Real::Integer(3).as_value().is_number());
    }

    #[test]
    fn reals_are_not_other_values() {
        assert!(!Real::Integer(3).is_bool());
        assert!(!Real::Integer(3).is_char());
        assert!(!Real::Integer(3).as_value().is_bool());
        assert!(!Real::Integer(3).as_value().is_char());
    }

    mod integers {
        use number::*;
        use value::*;

        #[test]
        fn are_equal() {
            assert_eq!(Real::Integer(3), Real::Integer(3));
            assert_ne!(Real::Integer(12), Real::Integer(9));
            assert_eq!(Real::Integer(4).as_value(), Real::Integer(4).as_value());
            assert_ne!(Real::Integer(5).as_value(), Real::Integer(7).as_value());
        }

        #[test]
        fn are_correctly_placed_in_the_number_pyramid() {
            assert!(Real::Integer(4).is_complex());
            assert!(Real::Integer(4).is_real());
            assert!(Real::Integer(4).is_rational());
            assert!(Real::Integer(4).is_integer());
            assert!(Real::Integer(4).is_number());
        }
        
        #[test]
        fn are_exact() {
            assert!(Real::Integer(3).is_exact());
            assert!(!Real::Integer(3).is_inexact());
        }
    }

    mod rationals {
        use number::*;
        use value::*;

        #[test]
        fn are_equal() {
            assert_eq!(Real::Rational(3, 2), Real::Rational(3, 2));
            assert_ne!(Real::Rational(12, 4), Real::Rational(9, 7));
            assert_eq!(Real::Rational(4, 5).as_value(), Real::Rational(4, 5).as_value());
            assert_ne!(Real::Rational(5, 6).as_value(), Real::Rational(7, 6).as_value());
        }

        #[test]
        fn are_correctly_placed_in_the_number_pyramid() {
            assert!(Real::Rational(4, 3).is_complex());
            assert!(Real::Rational(4, 3).is_real());
            assert!(Real::Rational(4, 3).is_rational());
            assert!(!Real::Rational(4, 3).is_integer());
        }

        #[test]
        fn are_exact() {
            assert!(Real::Rational(3, 5).is_exact());
            assert!(!Real::Rational(3, 5).is_inexact());
        }
    }

    mod irrationals {
        use number::*;
        use value::*;

        #[test]
        fn are_equal() {
            assert_eq!(Real::Irrational(3.2), Real::Irrational(3.2));
            assert_ne!(Real::Irrational(12.0), Real::Irrational(9.0));
            assert_eq!(Real::Irrational(4.0).as_value(), Real::Irrational(4.0).as_value());
            assert_ne!(Real::Irrational(5.0).as_value(), Real::Irrational(7.0).as_value());
        }

        #[test]
        fn are_correctly_placed_in_the_number_pyramid() {
            assert!(Real::Irrational(4.0).is_complex());
            assert!(Real::Irrational(4.0).is_real());
            assert!(!Real::Irrational(4.0).is_rational());
            assert!(!Real::Irrational(4.0).is_integer());
        }

        #[test]
        fn are_inexact() {
            assert!(Real::Irrational(3.0).is_inexact());
            assert!(!Real::Irrational(3.0).is_exact());
        }
    }
}
