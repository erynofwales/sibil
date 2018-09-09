/* types/src/number/frac.rs
 * Eryn Wells <eryn@erynwells.me>
 */

use std::any::Any;
use std::fmt;
use std::ops::{Add, Mul};
use number::arith::GCD;
use number::{Int, Number};
use object::{Obj, Object};

/// A fraction of two integers.
#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Frac {
    /// The numerator.
    p: Int,
    /// The denominator.
    q: Int
}

impl Frac {
    pub fn new(p: Int, q: Int) -> Result<Frac, ()> {
        if q.is_zero() {
            // TODO: Return a more specific error about dividing by zero.
            Err(())
        } else {
            Ok(Frac{p, q}.reduced())
        }
    }

    pub fn from_ints(p: i64, q: i64) -> Result<Frac, ()> {
        Frac::new(Int(p), Int(q))
    }

    fn reduced(self) -> Frac {
        let gcd = self.p.gcd(self.q);
        Frac { p: self.p / gcd, q: self.q / gcd }
    }

    fn _add(self, rhs: Frac) -> Frac {
        let p = self.p * rhs.q + rhs.p * self.q;
        let q = self.q * rhs.q;
        Frac{p,q}.reduced()
    }

    fn _mul(self, rhs: Frac) -> Frac {
        let p = self.p * rhs.p;
        let q = self.q * rhs.q;
        Frac{p,q}.reduced()
    }
}

impl Add for Frac {
    type Output = Frac;
    fn add(self, rhs: Self) -> Self::Output {
        self._add(rhs)
    }
}

impl<'a> Add<Frac> for &'a Frac {
    type Output = Frac;
    fn add(self, rhs: Frac) -> Self::Output {
        self._add(rhs)
    }
}

impl<'a, 'b> Add<&'a Frac> for &'b Frac {
    type Output = Frac;
    fn add(self, rhs: &Frac) -> Self::Output {
        self._add(*rhs)
    }
}

impl fmt::Display for Frac {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", self.p, self.q)
    }
}

impl Mul for Frac {
    type Output = Frac;
    fn mul(self, rhs: Self) -> Self::Output {
        self._mul(rhs)
    }
}

impl<'a> Mul<Frac> for &'a Frac {
    type Output = Frac;
    fn mul(self, rhs: Frac) -> Self::Output {
        self._mul(rhs)
    }
}

impl<'a, 'b> Mul<&'a Frac> for &'b Frac {
    type Output = Frac;
    fn mul(self, rhs: &Frac) -> Self::Output {
        self._mul(*rhs)
    }
}

impl Number for Frac {
    fn as_int(&self) -> Option<Int> {
        if self.q == Int(1) {
            Some(self.p)
        } else {
            None
        }
    }

    fn as_frac(&self) -> Option<Frac> { Frac::new(self.p, self.q).ok() }

    fn is_zero(&self) -> bool { self.p.is_zero() }
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
    use number::Number;
    use super::*;

    #[test]
    fn fracs_with_zero_q_are_invalid() {
        assert!(Frac::from_ints(3, 0).is_err())
    }

    #[test]
    fn equal_fracs_are_equal() {
        assert_eq!(Frac::from_ints(3, 2), Frac::from_ints(3, 2));
        assert_ne!(Frac::from_ints(12, 4), Frac::from_ints(9, 7));
    }

    #[test]
    fn fracs_should_reduce_to_ints_where_possible() {
        let fr = Frac::from_ints(3, 1).unwrap();
        assert_eq!(fr.as_int(), Some(Int(3)));
    }

    #[test]
    fn fracs_should_not_reduce_to_ints_where_impossible() {
        let fr = Frac::from_ints(3, 2).unwrap();
        assert_eq!(fr.as_int(), None);
    }

    #[test]
    fn fracs_are_exact() {
        let fr = Frac::from_ints(4, 2).unwrap();
        assert!(fr.is_exact());
    }

    #[test]
    fn fracs_can_add() {
        let a = Frac::from_ints(5, 6).unwrap();
        let b = Frac::from_ints(2, 3).unwrap();
        let r = Frac::from_ints(3, 2).unwrap();
        assert_eq!(a + b, r);
    }

    #[test]
    fn fracs_can_multiply() {
        let a = Frac::from_ints(4, 3).unwrap();
        let b = Frac::from_ints(3, 8).unwrap();
        let r = Frac::from_ints(1, 2).unwrap();
        assert_eq!(a * b, r);
    }
}
