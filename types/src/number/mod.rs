/* types/src/number/mod.rs
 * Eryn Wells <eryn@erynwells.me>
 */

/// # Numbers
///
/// Scheme numbers are complex, literally.

use std::fmt;

pub mod real;
mod add;
mod math;

pub use self::real::Real;

type Int = i64;
type Flt = f64;

#[derive(Debug, Eq, PartialEq)]
pub enum Exact { Yes, No }

impl fmt::Display for Exact {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            Exact::Yes => "#e",
            Exact::No => "#i",
        })
    }
}

// TODO: Implement PartialEq myself cause there are some weird nuances to comparing numbers.
#[derive(Debug, PartialEq)]
pub struct Number {
    real: Real,
    imag: Option<Real>,
    exact: Exact,
}

impl Number {
    fn new(real: Real, imag: Option<Real>, exact: Exact) -> Number {
        Number {
            real: real.reduce(),
            imag: imag.map(|n| n.reduce()),
            exact: exact,
        }
    }

    pub fn from_int(value: Int, exact: Exact) -> Number {
        Number::new(Real::Integer(value), None, exact)
    }

    pub fn from_quotient(p: Int, q: Int, exact: Exact) -> Number {
        let real = if exact == Exact::Yes {
            // Make an exact rational an integer if possible.
            Real::Rational(p, q).demote()
        }
        else {
            // Make an inexact rational an irrational.
            Real::Rational(p, q).promote_once()
        };
        Number::new(real, None, exact)
    }

    pub fn from_float(value: Flt, exact: Exact) -> Number {
        let real = if exact == Exact::Yes {
            // Attempt to demote irrationals.
            Real::Irrational(value).demote()
        }
        else {
            Real::Irrational(value)
        };
        Number::new(real, None, exact)
    }

    pub fn is_exact(&self) -> bool {
        match self.exact {
            Exact::Yes => true,
            Exact::No => false,
        }
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.real).and_then(
            |r| self.imag.map(|i| write!(f, "{:+}i", i)).unwrap_or(Ok(r)))
    }
}

#[cfg(test)]
mod tests {
    use super::Exact;
    use super::Number;
    use super::real::Real;

    #[test]
    fn exact_numbers_are_exact() {
        assert!(Number::from_int(3, Exact::Yes).is_exact());
        assert!(!Number::from_int(3, Exact::No).is_exact());
    }

    #[test]
    fn exact_irrationals_are_reduced() {
        let real = Real::Rational(3, 2);
        assert_eq!(Number::from_float(1.5, Exact::Yes), Number::new(real, None, Exact::Yes));
    }
}
