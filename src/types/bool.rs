/* types/bool.rs
 * Eryn Wells <eryn@erynwells.me>
 */

use std::any::Any;
use super::value::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Bool(pub bool);

impl Bool {
    pub fn new(v: bool) -> Bool { Bool(v) }
}

impl Value for Bool {
    fn as_value(&self) -> &Value { self }
}

impl ValueEq for Bool {
    fn eq(&self, other: &Value) -> bool {
        other.as_any().downcast_ref::<Self>().map_or(false, |x| x == self)
    }

    fn as_any(&self) -> &Any { self }
}

impl IsBool for Bool {
    fn is_bool(&self) -> bool { true }
}

impl IsChar for Bool {
    fn is_char(&self) -> bool { false }
}

#[cfg(test)]
mod tests {
    use super::Bool;
    use types::value::{IsBool, IsChar, Value};

    #[test]
    fn equal_bools_are_equal() {
        assert_eq!(Bool(true), Bool(true));
        assert_eq!(Bool(false), Bool(false));
        assert_ne!(Bool(true), Bool(false));

        assert_eq!(Bool(true).as_value(), Bool(true).as_value());
        assert_ne!(Bool(true).as_value(), Bool(false).as_value());
    }

    #[test]
    fn bools_are_bools() {
        assert_eq!(Bool(false).is_bool(), true);
    }

    #[test]
    fn bools_are_not_chars() {
        assert_eq!(Bool(false).is_char(), false);
    }
}