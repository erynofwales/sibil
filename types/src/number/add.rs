/* types/src/number/add.rs
 * Eryn Wells <eryn@erynwells.me>
 */

use std::ops::Add;
use number::Real;
use number::math::*;

impl Add for Real {
    type Output = Real;

    fn add(self, other: Real) -> Real {
        match (self, other) {
            (Real::Integer(v), Real::Integer(ov)) => Real::Integer(v + ov),
            (Real::Rational(p, q), Real::Rational(op, oq)) => {
                if q == oq {
                    Real::Rational(p + op, q)
                }
                else {
                    let lcm = q.lcm(oq);
                    let numer = (p * (lcm / q)) + (op * (lcm / oq));
                    let denom = lcm;
                    println!("lcm = {}, numer = {}, denom = {}", lcm, numer, denom);
                    Real::Rational(numer, denom)
                }.reduce()
            },
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
    fn rational_addition_with_like_denominators() {
        let r = Real::Rational(1, 4) + Real::Rational(1, 4);
        assert_eq!(r, Real::Rational(1, 2));
    }

    #[test]
    fn rational_addition_with_unlike_denominators() {
        let r = Real::Rational(4, 7) + Real::Rational(14, 3);
        assert_eq!(r, Real::Rational(110, 21));
    }

    #[test]
    fn irrational_addition() {
        let r = Real::Irrational(3.2) + Real::Irrational(3.2);
        assert_eq!(r, Real::Irrational(6.4));
    }
}
