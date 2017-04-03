/* types/value.rs
 * Eryn Wells <eryn@erynwells.me>
 */

use std::fmt::Debug;
use std::any::Any;

pub trait Value: Debug + ValueEq + IsBool + IsChar {
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

pub trait IsBool {
    fn is_bool(&self) -> bool { false }
}

pub trait IsChar {
    fn is_char(&self) -> bool { false }
}
