/* types/src/number/math.rs
 * Eryn Wells <eryn@erynwells.me>
 */

use number::{Int, Flt};

pub trait GCD {
    /// Find the greatest common divisor of `self` and another number.
    fn gcd(self, other: Self) -> Self;
}

pub trait LCM {
    /// Find the least common multiple of `self` and another number.
    fn lcm(self, other: Self) -> Self;
}

pub trait Rational {
    /// Convert `self` into a rational number -- the quotient of two whole numbers.
    fn to_rational(self) -> (Int, Int);
}

impl GCD for Int {
    fn gcd(self, other: Int) -> Int {
		let (mut a, mut b) = if self > other {
			(self, other)
		} else {
			(other, self)
		};

		while b != 0 {
			let r = a % b;
			a = b;
			b = r;
		}

		a
    }
}

impl LCM for Int {
    fn lcm(self, other: Int) -> Int {
        if self == 0 && other == 0 {
            0
        }
        else {
            self * other / self.gcd(other)
        }
    }
}

impl Rational for Int {
    fn to_rational(self) -> (Int, Int) { (self, 1) }
}

impl Rational for Flt {
    fn to_rational(self) -> (Int, Int) {
        // Convert the float to a fraction by iteratively multiplying by 10 until the fractional part of the float is 0.0.
        let whole_part = self.trunc();
        let mut p = self.fract();
        let mut q = 1.0;
        while p.fract() != 0.0 {
            p *= 10.0;
            q *= 10.0;
        }
        p += whole_part * q;

        // Integers from here down. Reduce the fraction before returning.
        let p = p as Int;
        let q = q as Int;
        let gcd = p.gcd(q);
        (p / gcd, q / gcd)
    }
}

#[cfg(test)]
mod tests {
    use super::{LCM, GCD};

    #[test]
    fn gcd_works() {
        assert_eq!(0, 0.gcd(0));
        assert_eq!(10, 10.gcd(0));
        assert_eq!(10, 0.gcd(10));
        assert_eq!(10, 10.gcd(20));
        assert_eq!(44, 2024.gcd(748));
    }

    #[test]
    fn lcm_works() {
        assert_eq!(0, 0.lcm(0));
        assert_eq!(0, 10.lcm(0));
        assert_eq!(0, 10.lcm(0));
        assert_eq!(42, 21.lcm(6));
    }
}
