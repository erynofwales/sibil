/* types/src/number/integer.rs
 * Eryn Wells <eryn@erynwells.me>
 */

use std::any::Any;
use std::fmt;
use number::Number;
use object::{Obj, Object};

#[derive(Debug, Eq, PartialEq)]
pub struct Int(i64);

impl fmt::Display for Int {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Object for Int {
    fn as_any(&self) -> &Any { self }
    fn as_num(&self) -> Option<&Number> { Some(self) }
}

impl Number for Int {
    fn as_int(&self) -> Option<&Int> { Some(self) }
}

impl PartialEq<Obj> for Int {
    fn eq<'a>(&self, rhs: &'a Obj) -> bool {
        match rhs.obj().and_then(Object::as_num) {
            Some(num) => self == num,
            None => false
        }
    }
}

impl<'a> PartialEq<Number + 'a> for Int {
    fn eq(&self, rhs: &(Number + 'a)) -> bool {
        match rhs.as_int() {
            Some(rhs) => *self == *rhs,
            None => false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equal_integers_are_equal() {
        assert_eq!(Int(3), Int(3));
        assert_ne!(Int(12), Int(9));
        assert_eq!(Obj::new(Int(3)), Obj::new(Int(3)));
        assert_ne!(Obj::new(Int(3)), Obj::new(Int(4)));
    }

    #[test]
    fn integers_are_integers() {
        assert_eq!(Int(4).as_bool(), None);
    }

    #[test]
    fn integers_are_exact() {
        assert!(Int(4).is_exact());
    }
}
