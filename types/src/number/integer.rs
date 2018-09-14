/* types/src/number/integer.rs
 * Eryn Wells <eryn@erynwells.me>
 */

use std::any::Any;
use std::fmt;
use number::arith::{GCD, LCM};
use object::{Obj, Object};
use super::{Frac, Number};

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Int(pub i64);

impl Int {
    pub fn zero() -> Int { Int(0) }
}

impl fmt::Display for Int {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl GCD for Int {
    fn gcd(self, other: Int) -> Int {
		let (mut a, mut b) = if self > other {
			(self, other)
		} else {
			(other, self)
		};
		while !b.is_zero() {
			let r = a % b;
			a = b;
			b = r;
		}
		a
    }
}

impl LCM for Int {
    fn lcm(self, other: Int) -> Int {
        if self.0 == 0 && other.0 == 0 {
            Int::zero()
        } else {
            self * other / self.gcd(other)
        }
    }
}

impl Object for Int {
    fn as_any(&self) -> &Any { self }
    fn as_num(&self) -> Option<&Number> { Some(self) }
}

impl Number for Int {
    fn as_int(&self) -> Option<Int> { Some(*self) }
    fn as_frac(&self) -> Option<Frac> { Frac::new(*self, Int(1)).ok() }
    fn is_zero(&self) -> bool { self.0 == 0 }
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
            Some(rhs) => *self == rhs,
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

    #[test]
    fn integers_add() {
        assert_eq!(Int(4) + Int(8), Int(12));
    }

    #[test]
    fn integers_multiply() {
        assert_eq!(Int(4) * Int(5), Int(20));
    }

    #[test]
    fn integer_modulo_divide() {
        assert_eq!(Int(20) % Int(5), Int(0));
        assert_eq!(Int(20) % Int(6), Int(2));
    }

    #[test]
    fn finding_int_gcd() {
        assert_eq!(Int(0), Int(0).gcd(Int(0)));
        assert_eq!(Int(10), Int(10).gcd(Int(0)));
        assert_eq!(Int(10), Int(0).gcd(Int(10)));
        assert_eq!(Int(10), Int(10).gcd(Int(20)));
        assert_eq!(Int(44), Int(2024).gcd(Int(748)));
    }

    #[test]
    fn finding_int_lcm() {
        assert_eq!(Int(0), Int(0).lcm(Int(0)));
        assert_eq!(Int(0), Int(10).lcm(Int(0)));
        assert_eq!(Int(0), Int(10).lcm(Int(0)));
        assert_eq!(Int(42), Int(21).lcm(Int(6)));
    }
}
