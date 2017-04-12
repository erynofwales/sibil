/* types/src/number/add.rs
 * Eryn Wells <eryn@erynwells.me>
 */

use std::ops::Add;
use super::Real;

impl Add for Real {
    type Output = Real;

    fn add(self, other: Real) -> Real {
        match (self, other) {
            (Real::Integer(v), Real::Integer(ov)) => Real::Integer(v + ov),
            (Real::Irrational(v), Real::Irrational(ov)) => Real::Irrational(v + ov),
            // TODO: The rest.
            _ => Real::Integer(0)
        }
    }
}

#[cfg(test)]
mod tests {
    use number::Real;

    #[test]
    fn integer_addition() {
        let r = Real::Integer(3) + Real::Integer(5);
        assert_eq!(r, Real::Integer(8));
    }

    #[test]
    fn rational_addition() {
        let r = Real::Rational(1, 4) + Real::Rational(1, 4);
        assert_eq!(r, Real::Rational(1, 2));
    }

    #[test]
    fn irrational_addition() {
        let r = Real::Irrational(3.2) + Real::Irrational(3.2);
        assert_eq!(r, Real::Irrational(6.4));
    }
}
