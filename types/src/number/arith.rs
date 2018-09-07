/* types/src/number/arith.rs
 * Eryn Wells <eryn@erynwells.me>
 */

pub trait GCD {
    /// Find the greatest common divisor of `self` and another number.
    fn gcd(self, other: Self) -> Self;
}

pub trait LCM {
    /// Find the least common multiple of `self` and another number.
    fn lcm(self, other: Self) -> Self;
}

//impl Rational for Int {
//    fn to_rational(self) -> (Int, Int) { (self, 1) }
//}
//
//impl Rational for Flt {
//    fn to_rational(self) -> (Int, Int) {
//        // Convert the float to a fraction by iteratively multiplying by 10 until the fractional part of the float is 0.0.
//        let whole_part = self.trunc();
//        let mut p = self.fract();
//        let mut q = 1.0;
//        while p.fract() != 0.0 {
//            p *= 10.0;
//            q *= 10.0;
//        }
//        p += whole_part * q;
//
//        // Integers from here down. Reduce the fraction before returning.
//        let p = p as Int;
//        let q = q as Int;
//        let gcd = p.gcd(q);
//        (p / gcd, q / gcd)
//    }
//}
