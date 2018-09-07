/* types/src/number/frac.rs
 * Eryn Wells <eryn@erynwells.me>
 */

use std::any::Any;
use std::fmt;
use number::{Int, Number};
use object::{Obj, Object};

/// A fraction consisting of a numerator and denominator.
#[derive(Debug, Eq, PartialEq)]
pub struct Frac(pub i64, pub u64);

impl Frac {
}

impl Number for Frac {
    fn as_int(&self) -> Option<Int> {
        if self.1 == 1 {
            Some(Int(self.0))
        } else {
            None
        }
    }

    fn as_frac(&self) -> Option<Frac> {
        Some(Frac(self.0, self.1))
    }
}

impl fmt::Display for Frac {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", self.0, self.1)
    }
}

impl Object for Frac {
    fn as_any(&self) -> &Any { self }
    fn as_num(&self) -> Option<&Number> { Some(self) }
}

impl PartialEq<Obj> for Frac {
    fn eq<'a>(&self, rhs: &'a Obj) -> bool {
        match rhs.obj().and_then(Object::as_num) {
            Some(num) => self == num,
            None => false
        }
    }
}

impl<'a> PartialEq<Number + 'a> for Frac {
    fn eq(&self, rhs: &(Number + 'a)) -> bool {
        match rhs.as_frac() {
            Some(rhs) => *self == rhs,
            None => false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equal_fracs_are_equal() {
        assert_eq!(Frac(3, 2), Frac(3, 2));
        assert_ne!(Frac(12, 4), Frac(9, 7));
    }

    #[test]
    fn fracs_should_reduce_to_ints_where_possible() {
        let rational_as_integer = Frac(3, 1).as_int();
        assert!(rational_as_integer.is_some());
        // Oh my god this line is so dumb.
    }

    #[test]
    fn fracs_should_not_reduce_to_ints_where_impossible() {
        let rational_as_integer = Frac(3, 2).as_int();
        assert!(rational_as_integer.is_none());
    }

    #[test]
    fn fracs_are_exact() {
        assert!(Frac(4, 2).is_exact());
    }
}
