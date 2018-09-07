/* types/src/number/integer.rs
 * Eryn Wells <eryn@erynwells.me>
 */

use std::any::Any;
use std::fmt;
use std::ops::{Add, Div, Mul, Rem};
use number::arith::{GCD, LCM};
use number::{Frac, Number};
use object::{Obj, Object};

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Int(pub i64);

impl Add for Int {
    type Output = Int;
    fn add(self, rhs: Self) -> Self::Output {
        Int(self.0 + rhs.0)
    }
}

impl<'a> Add<Int> for &'a Int {
    type Output = Int;
    fn add(self, rhs: Int) -> Self::Output {
        Int(self.0 + rhs.0)
    }
}

impl<'a, 'b> Add<&'a Int> for &'b Int {
    type Output = Int;
    fn add(self, rhs: &Int) -> Self::Output {
        Int(self.0 + rhs.0)
    }
}

impl fmt::Display for Int {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Div for Int {
    type Output = Int;
    fn div(self, rhs: Self) -> Self::Output {
        Int(self.0 / rhs.0)
    }
}

impl<'a> Div<Int> for &'a Int {
    type Output = Int;
    fn div(self, rhs: Int) -> Self::Output {
        Int(self.0 / rhs.0)
    }
}

impl<'a, 'b> Div<&'a Int> for &'b Int {
    type Output = Int;
    fn div(self, rhs: &Int) -> Self::Output {
        Int(self.0 / rhs.0)
    }
}

impl GCD for Int {
    fn gcd(self, other: Int) -> Int {
		let (mut a, mut b) = if self.0 > other.0 {
			(self.0, other.0)
		} else {
			(other.0, self.0)
		};
		while b != 0 {
			let r = a % b;
			a = b;
			b = r;
		}
		Int(a)
    }
}

impl LCM for Int {
    fn lcm(self, other: Int) -> Int {
        if self.0 == 0 && other.0 == 0 {
            Int(0)
        } else {
            Int(self.0 * other.0 / self.gcd(other).0)
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
}

impl Mul for Int {
    type Output = Int;
    fn mul(self, rhs: Self) -> Self::Output {
        Int(self.0 * rhs.0)
    }
}

impl<'a> Mul<Int> for &'a Int {
    type Output = Int;
    fn mul(self, rhs: Int) -> Self::Output {
        Int(self.0 * rhs.0)
    }
}

impl<'a, 'b> Mul<&'a Int> for &'b Int {
    type Output = Int;
    fn mul(self, rhs: &Int) -> Self::Output {
        Int(self.0 * rhs.0)
    }
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

impl Rem for Int {
    type Output = Int;
    fn rem(self, rhs: Self) -> Self::Output {
        Int(self.0 % rhs.0)
    }
}

impl<'a> Rem<Int> for &'a Int {
    type Output = Int;
    fn rem(self, rhs: Int) -> Self::Output {
        Int(self.0 % rhs.0)
    }
}

impl<'a, 'b> Rem<&'a Int> for &'b Int {
    type Output = Int;
    fn rem(self, rhs: &Int) -> Self::Output {
        Int(self.0 % rhs.0)
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
