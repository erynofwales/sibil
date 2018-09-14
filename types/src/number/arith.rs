/* types/src/number/arith.rs
 * Eryn Wells <eryn@erynwells.me>
 */

use std::ops::{Add, Div, Mul, Sub, Rem};
use number::{Int, Irr, Number};

pub trait GCD {
    /// Find the greatest common divisor of `self` and another number.
    fn gcd(self, other: Self) -> Self;
}

pub trait LCM {
    /// Find the least common multiple of `self` and another number.
    fn lcm(self, other: Self) -> Self;
}

macro_rules! impl_newtype_arith_op {
    ($id:ident, $opt:ident, $opm:ident, $op:tt) => {
        impl<T> $opt<T> for $id where T: Number + Into<$id> {
            type Output = $id;
            #[inline]
            fn $opm(self, rhs: T) -> Self::Output {
                let rhs: $id = rhs.into();
                $id(self.0 $op rhs.0)
            }
        }
        impl<'a, T> $opt<T> for &'a $id where T: Number + Into<$id> {
            type Output = $id;
            #[inline]
            fn $opm(self, rhs: T) -> Self::Output {
                let rhs: $id = rhs.into();
                $id(self.0 $op rhs.0)
            }
        }
    }
}

macro_rules! impl_newtype_arith {
    ($($id:ident)*) => ($(
        impl_newtype_arith_op!{$id, Add, add, +}
        impl_newtype_arith_op!{$id, Div, div, /}
        impl_newtype_arith_op!{$id, Mul, mul, *}
        impl_newtype_arith_op!{$id, Sub, sub, -}
    )*)
}

impl_newtype_arith!{ Int Irr }
impl_newtype_arith_op!{Int, Rem, rem, %}
impl_newtype_arith_op!{Irr, Rem, rem, %}
