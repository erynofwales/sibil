/* types/src/number/mod.rs
 * Eryn Wells <eryn@erynwells.me>
 */

//! # Numbers
//!
//! Scheme numbers are complex, literally. The model it uses is a hierarchy of types called the
//! Number Tower. It consists of four types, in order: Integers, Rationals (or Fractionals),
//! Irrationals (or Reals), and Complex Numbers. Each type going down the tower can be
//! unequivocally cast to the type below it, but the reverse is not necessarily true. So, an
//! Integer can be cast as a Rational (by putting its value over 1), but a Rational like 1/2 cannot
//! be represented as an Integer.

mod arith;
mod integer;
mod frac;

use object::Object;

pub use self::integer::Int;
pub use self::frac::Frac;

pub trait Number: 
    Object 
{
    /// Cast this Number to an Int if possible.
    fn as_int(&self) -> Option<Int> { None }
    /// Cast this Number to a Frac if possible.
    fn as_frac(&self) -> Option<Frac> { None }
    /// Return `true` if this Number is an exact representation of its value.
    fn is_exact(&self) -> bool { true }
    fn is_zero(&self) -> bool;
}

// TODO: Implement PartialEq myself cause there are some weird nuances to comparing numbers.
//#[derive(Debug, PartialEq)]
//pub struct Number {
//    real: Real,
//    imag: Option<Real>,
//    exact: Exact,
//}

//impl Number {
//    fn new(real: Real, imag: Option<Real>, exact: Exact) -> Number {
//        Number {
//            real: real.reduce(),
//            imag: imag.map(|n| n.reduce()),
//            exact: exact,
//        }
//    }
//
//    pub fn from_int(value: Int, exact: Exact) -> Number {
//        Number::new(Real::Integer(value), None, exact)
//    }
//
//    pub fn from_quotient(p: Int, q: Int, exact: Exact) -> Number {
//        let real = if exact == Exact::Yes {
//            // Make an exact rational an integer if possible.
//            Real::Rational(p, q).demote()
//        }
//        else {
//            // Make an inexact rational an irrational.
//            Real::Rational(p, q).promote_once()
//        };
//        Number::new(real, None, exact)
//    }
//
//    pub fn from_float(value: Flt, exact: Exact) -> Number {
//        let real = if exact == Exact::Yes {
//            // Attempt to demote irrationals.
//            Real::Irrational(value).demote()
//        }
//        else {
//            Real::Irrational(value)
//        };
//        Number::new(real, None, exact)
//    }
//
//    pub fn is_exact(&self) -> bool {
//        match self.exact {
//            Exact::Yes => true,
//            Exact::No => false,
//        }
//    }
//}
//
//impl fmt::Display for Number {
//    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//        write!(f, "{}", self.real).and_then(
//            |r| self.imag.map(|i| write!(f, "{:+}i", i)).unwrap_or(Ok(r)))
//    }
//}
//
//#[cfg(test)]
//mod tests {
//    use super::Exact;
//    use super::Number;
//    use super::real::Real;
//
//    #[test]
//    fn exact_numbers_are_exact() {
//        assert!(Number::from_int(3, Exact::Yes).is_exact());
//        assert!(!Number::from_int(3, Exact::No).is_exact());
//    }
//
//    #[test]
//    fn exact_irrationals_are_reduced() {
//        let real = Real::Rational(3, 2);
//        assert_eq!(Number::from_float(1.5, Exact::Yes), Number::new(real, None, Exact::Yes));
//    }
//}
