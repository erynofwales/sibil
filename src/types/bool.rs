/* types/bool.rs
 * Eryn Wells <eryn@erynwells.me>
 */

use std::any::Any;
use super::value::{Value, ValueEq};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Bool(bool);

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
