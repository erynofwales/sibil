/* types/src/number/mod.rs
 * Eryn Wells <eryn@erynwells.me>
 */

//! # Numbers
//!
//! Scheme numbers are complex, literally. The model it uses is a hierarchy of types called the
//! Number Tower. It consists of four types, in order: Integers, Rationals (or Fractionals),
//! Irrationals (or Reals), and Complex Numbers. Each type going down the tower can be
//! unequivocally cast to the type below it, but the reverse is not necessarily true. So, an
//! Integer can be cast as a Rational (by putting its value over 1), but a Rational like 1/2 cannot
//! be represented as an Integer.

use object::Object;

mod arith;
mod frac;
mod integer;
mod irr;

pub use self::frac::Frac;
pub use self::integer::Int;
pub use self::irr::Irr;

pub trait Number: 
    Object 
{
    /// Cast this Number to an Int if possible.
    fn as_int(&self) -> Option<Int> { None }
    /// Cast this Number to a Frac if possible.
    fn as_frac(&self) -> Option<Frac> { None }
    /// Return `true` if this Number is an exact representation of its value.
    fn is_exact(&self) -> bool { true }
    /// Return `true` if this Number is equal to 0.
    fn is_zero(&self) -> bool;
}
