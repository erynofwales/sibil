/* types/char.rs
 * Eryn Wells <eryn@erynwells.me>
 */

use std::any::Any;
use super::value::{Value, ValueEq};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Char(char);

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

