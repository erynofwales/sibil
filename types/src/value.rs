/* types/value.rs
 * Eryn Wells <eryn@erynwells.me>
 */

use std::fmt::Debug;
use std::any::Any;

pub trait Value: Debug + IsBool + IsChar + IsNumber + ValueEq {
    fn as_value(&self) -> &Value;
}

pub trait IsBool {
    /// Should return `true` if this Value is a Bool.
    fn is_bool(&self) -> bool { false }
}

pub trait IsChar {
    /// Should return `true` if this Value is a Char.
    fn is_char(&self) -> bool { false }
}

pub trait IsNumber {
    /// Should return `true` if this Value is a number type.
    fn is_number(&self) -> bool { self.is_complex() || self.is_real() || self.is_rational() || self.is_integer() }
    /// Should return `true` if this Value is a complex number type.
    fn is_complex(&self) -> bool { self.is_real() }
    /// Should return `true` if this Value is a real number type.
    fn is_real(&self) -> bool { self.is_rational() }
    /// Should return `true` if this Value is a rational number type.
    fn is_rational(&self) -> bool { self.is_integer() }
    /// Should return `true` if this Value is a integer number type.
    fn is_integer(&self) -> bool { false }
}

/// A trait on value types that makes it easier to compare values of disparate types. The methods
/// provided by this trait are used by the PartialEq implementation on Values.
pub trait ValueEq {
    fn eq(&self, other: &Value) -> bool;
    fn as_any(&self) -> &Any;
}

impl<'lhs,'rhs> PartialEq<Value+'rhs> for Value+'lhs {
    fn eq(&self, other: &(Value+'rhs)) -> bool {
        ValueEq::eq(self, other)
    }
}
