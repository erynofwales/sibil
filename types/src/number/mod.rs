/* types/src/number/mod.rs
 * Eryn Wells <eryn@erynwells.me>
 */

/// # Numbers
///
/// Scheme numbers are complex, literally.

pub mod real;
//pub mod complex;
//mod add;
mod math;

pub use self::real::Real;
//pub use self::complex::Complex;

type Int = i64;
type Flt = f64;

// TODO: Implement PartialEq myself cause there are some weird nuances to comparing numbers.
#[derive(Debug, PartialEq)]
pub struct Number {
    real: Real,
    imag: Option<Real>,
    exact: bool,
}

impl Number {
    fn new(real: Real, imag: Option<Real>, exact: bool) -> Number {
        Number {
            real: real.reduce(),
            imag: imag.map(|n| n.reduce()),
            exact: exact,
        }
    }

    pub fn from_int(value: Int, exact: bool) -> Number {
        Number::new(Real::Integer(value), None, exact)
    }

    pub fn from_quotient(p: Int, q: Int, exact: bool) -> Number {
        let real = if exact {
            // Make an exact rational an integer if possible.
            Real::Rational(p, q).demote()
        }
        else {
            // Make an inexact rational an irrational.
            Real::Rational(p, q).promote_once()
        };
        Number::new(real, None, exact)
    }

    pub fn from_float(value: Flt, exact: bool) -> Number {
        let real = if exact {
            // Attempt to demote irrationals.
            Real::Irrational(value).demote()
        }
        else {
            Real::Irrational(value)
        };
        Number::new(real, None, exact)
    }

    pub fn is_exact(&self) -> bool { self.exact }
}

#[cfg(test)]
mod tests {
    use super::Number;
    use super::real::Real;

    #[test]
    fn exact_numbers_are_exact() {
        assert!(Number::from_int(3, true).is_exact());
        assert!(!Number::from_int(3, false).is_exact());
    }

    #[test]
    fn exact_irrationals_are_reduced() {
        let real = Real::Rational(3, 2);
        assert_eq!(Number::from_float(1.5, true), Number::new(real, None, true));
    }
}
