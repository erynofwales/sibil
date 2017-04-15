/* types/src/number/real.rs
 * Eryn Wells <eryn@erynwells.me>
 */

use super::*;
use number::math::*;

#[derive(Clone, Copy, Debug)]
pub enum Real {
    Integer(Int),
    Rational(Int, Int),
    Irrational(Flt)
}

impl Real {
    /// Reduce a rational.
    pub fn reduce(self) -> Real {
        match self {
            Real::Rational(p, q) => {
                let gcd = p.gcd(q);
                Real::Rational(p / gcd, q / gcd)
            },
            _ => self
        }
    }

    /// Promote a Real to the next highest type.
    pub fn promote_once(self) -> Real {
        match self {
            Real::Integer(v) => Real::Rational(v, 1),
            Real::Rational(p, q) => Real::Irrational(p as Flt / q as Flt),
            Real::Irrational(_) => self
        }
    }

    /// Demote a Real as far down the tower as possible.
    ///
    /// # Examples
    ///
    /// ```
    /// use number::real::Real;
    ///
    /// assert_eq!(Real::Integer(3).demote(), Real::Integer(3));
    /// assert_eq!(Real::Rational(3, 1).demote(), Real::Integer(3));
    /// assert_eq!(Real::Irrational(3.2).demote(), Real::Rational(32, 100));
    /// assert_eq!(Real::Irrational(3.0).demote(), Real::Integer(3));
    /// ```
    pub fn demote(self) -> Real {
        match self.demote_once() {
            Some(demoted) => demoted.demote(),
            None => self,
        }
    }

    /// Demote a Real to the next lowest type, if possible.
    pub fn demote_once(self) -> Option<Real> {
        match self {
            Real::Integer(_) => None,
            Real::Rational(p, q) => if q == 1 {
                Some(Real::Integer(p))
            }
            else {
                None
            },
            // TODO: Convert an irrational into a fraction.
            Real::Irrational(v) => {
                let whole_part = v.trunc();
                let mut p = v.fract();
                let mut q = 1.0;
                while p.fract() != 0.0 {
                    p *= 10.0;
                    q *= 10.0;
                }
                p += whole_part * q;
                Some(Real::Rational(p as Int, q as Int).reduce())
            }
        }
    }
}

impl PartialEq for Real {
    fn eq(&self, other: &Real) -> bool {
        // TODO: Make comparing different variants possible.
        match (self, other) {
            (&Real::Integer(v), &Real::Integer(ov)) => v == ov,
            (&Real::Rational(p, q), &Real::Rational(op, oq)) => p == op && q == oq,
            (&Real::Irrational(v), &Real::Irrational(ov)) => v == ov,
            _ => false
        }
    }
}

#[cfg(test)]
mod tests {
    mod integers {
        use number::real::*;

        #[test]
        fn are_equal() {
            assert_eq!(Real::Integer(3), Real::Integer(3));
            assert_ne!(Real::Integer(12), Real::Integer(9));
        }

        #[test]
        fn reduce_to_themselves() {
            assert_eq!(Real::Integer(4).reduce(), Real::Integer(4));
        }

        #[test]
        fn promote_to_rationals() {
            assert_eq!(Real::Integer(6).promote_once(), Real::Rational(6, 1));
        }

        #[test]
        fn demote_to_themselves() {
            assert_eq!(Real::Integer(6).demote(), Real::Integer(6));
        }
    }

    mod rationals {
        use number::real::*;

        #[test]
        fn are_equal() {
            assert_eq!(Real::Rational(3, 2), Real::Rational(3, 2));
            assert_ne!(Real::Rational(12, 4), Real::Rational(9, 7));
        }

        #[test]
        fn reduce_correctly() {
            assert_eq!(Real::Rational(2, 4).reduce(), Real::Rational(1, 2));
        }

        #[test]
        fn promote_to_irrationals() {
            assert_eq!(Real::Rational(3, 2).promote_once(), Real::Irrational(1.5));
        }

        #[test]
        fn demote_to_integers_if_possible() {
            assert_eq!(Real::Rational(3, 2).demote(), Real::Rational(3, 2));
            assert_eq!(Real::Rational(4, 1).demote(), Real::Integer(4));
        }
    }

    mod irrationals {
        use number::real::*;

        #[test]
        fn are_equal() {
            assert_eq!(Real::Irrational(3.2), Real::Irrational(3.2));
            assert_ne!(Real::Irrational(12.0), Real::Irrational(9.0));
        }

        #[test]
        fn reduce_to_themselves() {
            assert_eq!(Real::Irrational(3.2).reduce(), Real::Irrational(3.2));
        }

        #[test]
        fn promote_to_themselves() {
            assert_eq!(Real::Irrational(3.2).promote_once(), Real::Irrational(3.2));
        }

        #[test]
        fn demote_to_rationals() {
            assert_eq!(Real::Irrational(3.2).demote(), Real::Rational(16, 5));
            assert_eq!(Real::Irrational(3.5).demote(), Real::Rational(7, 2));
        }
    }
}
