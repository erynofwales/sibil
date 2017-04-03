/* types/char.rs
 * Eryn Wells <eryn@erynwells.me>
 */

use std::any::Any;
use super::value::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Char(pub char);

impl Char {
    pub fn new(v: char) -> Char { Char(v) }
}

impl Value for Char {
    fn as_value(&self) -> &Value { self }
}

impl ValueEq for Char {
    fn eq(&self, other: &Value) -> bool {
        other.as_any().downcast_ref::<Self>().map_or(false, |x| x == self)
    }

    fn as_any(&self) -> &Any { self }
}

impl IsBool for Char {
    fn is_bool(&self) -> bool { false }
}

impl IsChar for Char {
    fn is_char(&self) -> bool { true }
}

#[cfg(test)]
mod tests {
    use super::Char;
    use types::value::{IsBool, IsChar};

    #[test]
    fn chars_are_chars() {
        assert_eq!(Char('a').is_char(), true);
    }

    #[test]
    fn chars_are_not_bools() {
        assert_eq!(Char('a').is_bool(), false);
    }
}
