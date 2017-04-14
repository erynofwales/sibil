/* types/src/number/mod.rs
 * Eryn Wells <eryn@erynwells.me>
 */

/// # Numbers
///
/// Scheme numbers are complex, literally.

pub mod real;
pub mod complex;
mod add;
mod math;

pub use self::real::Real;
pub use self::complex::Complex;

use std::any::Any;
use std::fmt::Debug;
use std::ops::Deref;

use super::value::*;

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

    pub fn is_exact(&self) -> bool { self.exact }
}

impl Value for Number {
    fn as_value(&self) -> &Value { self }
}

impl ValueEq for Number {
    fn eq(&self, other: &Value) -> bool {
        other.as_any().downcast_ref::<Self>().map_or(false, |x| x == self)
    }
    fn as_any(&self) -> &Any { self }
}

impl IsBool for Number { }
impl IsChar for Number { }
impl IsNumber for Number { }
