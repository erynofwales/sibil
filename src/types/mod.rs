/* mod.rs
 * Eryn Wells <eryn@erynwells.me>
 */

use std::fmt::Debug;
use std::any::Any;

pub use self::number::Number;

pub mod number;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Boolean(bool);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Character(char);

pub trait Value: Debug + ValueEq {
    fn as_value(&self) -> &Value;
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

impl Value for Boolean {
    fn as_value(&self) -> &Value { self }
}

impl ValueEq for Boolean {
    fn eq(&self, other: &Value) -> bool {
        other.as_any().downcast_ref::<Self>().map_or(false, |x| x == self)
    }

    fn as_any(&self) -> &Any { self }
}

impl Value for Character {
    fn as_value(&self) -> &Value { self }
}

impl ValueEq for Character {
    fn eq(&self, other: &Value) -> bool {
        other.as_any().downcast_ref::<Self>().map_or(false, |x| x == self)
    }

    fn as_any(&self) -> &Any { self }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn booleans_are_equal() {
        assert_eq!(Boolean(true), Boolean(true));
        assert_eq!(Boolean(false), Boolean(false));
        assert_ne!(Boolean(true), Boolean(false));
    }

    #[test]
    fn equal_chars_are_equal() {
        assert_eq!(Character('a'), Character('a'));
        assert_eq!(Character('a').as_value(), Character('a').as_value());
    }

    #[test]
    fn booleans_and_chars_are_not_equal() {
        assert_ne!(Boolean(true).as_value(), Character('a').as_value());
    }
}
