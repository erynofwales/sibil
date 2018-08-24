/* types/src/bool.rs
 * Eryn Wells <eryn@erynwells.me>
 */

use std::fmt;
use object::Obj;
use preds;

/// The Scheme boolean type. It can be `True` or `False`.
pub enum Bool { True, False }

impl Obj for Bool { }

impl fmt::Display for Bool {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Bool::True => write!(f, "#t"),
            Bool::False => write!(f, "#f")
        }
    }
}

impl preds::IsBool for Bool {
    fn is_bool(&self) -> bool { true }
}

#[cfg(test)]
mod tests {
    use object::Object;
    use predicates::{IsBool, IsChar};

    #[test]
    fn bools_are_bools() {
        assert_eq!(Object::Bool(false).is_bool(), true);
        assert_eq!(Object::Bool(false).is_char(), false);
    }

    #[test]
    fn equal_bools_are_equal() {
        assert_eq!(Object::Bool(true), Object::Bool(true));
        assert_eq!(Object::Bool(false), Object::Bool(false));
        assert_ne!(Object::Bool(true), Object::Bool(false));
    }
}
