/* number.rs
 * Eryn Wells <eryn@erynwells.me>
 */

/// # Numbers
///
/// Scheme numbers are complex, literally.

#[derive(PartialEq, Debug)]
pub struct Number {
    pub value: f64
}

impl Number {
    pub fn from_int(v: i64) -> Number {
        Number { value: v as f64 }
    }

    pub fn from_float(v: f64) -> Number {
        Number { value: v }
    }
}

/*
pub trait Number {
    fn new() -> Number;
    fn from_int(v: i64);
    fn from_float(v: f64);
}

pub trait Exact {
    fn exact() -> bool;
}

type Integer = i64;

impl Exact for Integer {
    fn exact() -> bool { true }
}

#[derive(PartialEq, Debug)]
pub struct Rational { numer: i64, denom: i64 }

impl Exact for Rational {
    fn exact() -> bool { true }
}

type Real = f64;

impl Exact for Real {
    fn exact() -> bool { false }
}

#[derive(PartialEq, Debug)]
pub struct Complex { real: f64, imag: f64 }

impl Exact for Complex {
    fn exact() -> bool { false }
}
*/
