/* types/src/number/math.rs
 * Eryn Wells <eryn@erynwells.me>
 */

use number::Int;

pub trait GCD {
    /// Find the greatest common divisor of `self` and another number.
    fn gcd(self, other: Self) -> Self;
}

pub trait LCM {
    /// Find the least common multiple of `self` and another number.
    fn lcm(self, other: Self) -> Self;
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
