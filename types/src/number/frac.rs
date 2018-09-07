/* types/src/number/frac.rs
 * Eryn Wells <eryn@erynwells.me>
 */

use std::any::Any;
use std::fmt;
use std::ops::{Add, Mul};
use number::arith::GCD;
use number::{Int, Number};
use object::{Obj, Object};

/// A fraction consisting of a numerator and denominator.
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Frac(Int, Int);

impl Frac {
    pub fn new(p: Int, q: Int) -> Result<Frac, ()> {
        if q == Int(0) {
            // TODO: Return a more specific error about dividing by zero.
            Err(())
        } else {
            Ok(Frac(p, q).reduced())
        }
    }

    fn reduced(self) -> Frac {
        let gcd = self.0.gcd(self.1);
        Frac(self.0 / gcd, self.1 / gcd)
    }
}

impl Number for Frac {
    fn as_int(&self) -> Option<Int> {
        if self.1 == Int(1) {
            Some(self.0)
        } else {
            None
        }
    }

    fn as_frac(&self) -> Option<Frac> { Frac::new(self.0, self.1).ok() }
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
        assert_eq!(Frac(Int(3), Int(2)), Frac(Int(3), Int(2)));
        assert_ne!(Frac(Int(12), Int(4)), Frac(Int(9), Int(7)));
    }

    #[test]
    fn fracs_should_reduce_to_ints_where_possible() {
        let rational_as_integer = Frac(Int(3), Int(1)).as_int();
        assert!(rational_as_integer.is_some());
        // Oh my god this line is so dumb.
    }

    #[test]
    fn fracs_should_not_reduce_to_ints_where_impossible() {
        let rational_as_integer = Frac(Int(3), Int(2)).as_int();
        assert!(rational_as_integer.is_none());
    }

    #[test]
    fn fracs_are_exact() {
        assert!(Frac(Int(4), Int(2)).is_exact());
    }
}
