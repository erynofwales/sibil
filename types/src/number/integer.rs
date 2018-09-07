/* types/src/number/integer.rs
 * Eryn Wells <eryn@erynwells.me>
 */

use std::any::Any;
use std::fmt;
use std::ops::{Add, Mul};
use number::{Frac, Number};
use object::{Obj, Object};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl Object for Int {
    fn as_any(&self) -> &Any { self }
    fn as_num(&self) -> Option<&Number> { Some(self) }
}

impl Number for Int {
    fn as_int(&self) -> Option<Int> { Some(*self) }
    fn as_frac(&self) -> Option<Frac> { Some(Frac(self.0, 1)) }
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
}
