/* lexer/src/states/number/mod.rs
 * Eryn Wells <eryn@erynwells.me>
 */

mod prefix;
mod sign;

pub use self::prefix::Prefix;

trait NumberLexable {
    /// Returns the value of this character interpreted as the indicator for a
    /// base. In Scheme, you indicate the base of a number by prefixing it with
    /// #[bodx].
    fn base_value(&self) -> Option<Radix>;
    /// Returns the value of the character interpreted as a numerical digit.
    fn digit_value(&self) -> Option<u8>;
    fn sign_value(&self) -> Option<Sign>;
    fn is_dot(&self) -> bool;
    fn is_hash(&self) -> bool;
    fn is_sign(&self) -> bool;
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Radix { Bin = 2, Oct = 8, Dec = 10, Hex = 16 }

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Sign { Neg = -1, Pos = 1 }

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Exact { Yes, No }

#[derive(Copy, Clone, Debug)]
pub struct Builder {
    radix: Option<Radix>,
    sign: Option<Sign>,
    exact: Option<Exact>,
    value: i64,
}

impl Radix {
    pub fn from(c: char) -> Option<Radix> {
        match c {
            'b'|'B' => Some(Radix::Bin),
            'o'|'O' => Some(Radix::Oct),
            'd'|'D' => Some(Radix::Dec),
            'x'|'X' => Some(Radix::Hex),
            _ => None
        }
    }
}

impl Exact {
    pub fn from(c: char) -> Option<Exact> {
        match c {
            'i'|'I' => Some(Exact::No),
            'e'|'E' => Some(Exact::Yes),
            _ => None
        }
    }
}

impl Builder {
    pub fn new() -> Builder {
        Builder {
             radix: None,
             sign: None,
             exact: None,
             value: 0
        }
    }

    fn push_digit(&mut self, digit: u8) {
        //self.value = self.value * self.base_value() as i64 + digit as i64;
    }

    fn push_exact(&mut self, ex: Exact) {
        self.exact = Some(ex);
    }

    fn push_radix(&mut self, radix: Radix) {
        self.radix = Some(radix);
    }

    fn push_sign(&mut self, sign: Sign) {
        self.sign = Some(sign);
    }

    fn resolve(&self) -> i64 {
        //let sign_factor: i64 = if let Some(sign) = self.sign { sign as i64 } else { 1 };
        //self.value * sign_factor
        0
    }

    fn seen_exact(&self) -> bool { self.exact.is_some() }
    fn seen_radix(&self) -> bool { self.radix.is_some() }
    fn seen_sign(&self) -> bool { self.sign.is_some() }
}
