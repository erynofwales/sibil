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

pub trait Value: Debug + 'static { }
impl Value for Boolean { }
impl Value for Character { }

impl<'a,'b> PartialEq<&'a Value> for &'b Value {
    fn eq(&self, other: &&Value) -> bool {
        other.as_any().downcast_ref::<Self>().map_or(false, |x| x == self)
    }
}

impl Value {
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
}
