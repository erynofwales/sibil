/* types/src/number/mod.rs
 * Eryn Wells <eryn@erynwells.me>
 */

/// # Numbers
///
/// Scheme numbers are complex, literally.

pub mod real;
mod add;

pub use self::real::Real;

use std::any::Any;
use std::fmt::Debug;
use std::ops::Deref;

use super::value::*;

type Int = i64;
type Flt = f64;

trait Number: Debug + IsBool + IsChar + IsNumber + Value {
    /// Convert a Number to the next lowest type in Scheme's number pyramid, if possible.
    fn convert_down(&self) -> Option<Box<Number>>;
}

trait IsExact {
    /// Should return `true` if this Number is represented exactly. This should be an inverse of
    /// `is_inexact()`.
    fn is_exact(&self) -> bool { false }

    /// Should return `true` if this Number is not represented exactly. This should be an inverse
    /// of `is_exact()`.
    fn is_inexact(&self) -> bool { !self.is_exact() }
}

impl Value for Box<Number> {
    fn as_value(&self) -> &Value { self.deref().as_value() }
}

impl IsBool for Box<Number> { }
impl IsChar for Box<Number> { }
impl IsNumber for Box<Number> { }

impl ValueEq for Box<Number> {
    fn eq(&self, other: &Value) -> bool {
        self.deref().eq(other)
    }
    fn as_any(&self) -> &Any { self }
}


#[derive(Debug, PartialEq)]
struct Complex {
    real: Real,
    imag: Real
}

impl IsNumber for Complex {
    fn is_complex(&self) -> bool { true }
}
