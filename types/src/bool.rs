/* types/src/bool.rs
 * Eryn Wells <eryn@erynwells.me>
 */

use std::any::Any;
use std::fmt;
use std::ops::Deref;
use object::{Obj, Object};

/// The Scheme boolean type. It can be `True` or `False`.
#[derive(Debug, PartialEq)]
pub enum Bool { True, False }

impl Object for Bool {
    fn as_any(&self) -> &Any { self }
    fn as_bool(&self) -> Option<&Bool> { Some(self) }
}

impl fmt::Display for Bool {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Bool::True => write!(f, "#t"),
            Bool::False => write!(f, "#f")
        }
    }
}

impl From<bool> for Bool {
    fn from(value: bool) -> Self {
        if value {
            Bool::True
        } else {
            Bool::False
        }
    }
}

impl PartialEq<Obj> for Bool {
    fn eq(&self, rhs: &Obj) -> bool {
        match rhs {
            Obj::Null => false,
            Obj::Ptr(ref inner) => {
                if let Some(rhs_bool) = inner.deref().as_bool() {
                    self == rhs_bool
                } else {
                    false
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Bool;

    #[test]
    fn equal_bools_are_equal() {
        assert_eq!(Bool::True, Bool::True);
        assert_eq!(Bool::False, Bool::False);
        assert_ne!(Bool::True, Bool::False);
    }
}
