/* lexer/src/states/number/mod.rs
 * Eryn Wells <eryn@erynwells.me>
 */

use error::Error;

mod digit;
mod prefix;
mod sign;

pub use self::prefix::Prefix;
pub use self::digit::Digit;

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
             value: 0,
        }
    }

    fn push_digit(&mut self, c: char) -> Result<(), Error> {
        let rx = self.radix_value();
        match c.to_digit(rx as u32) {
            Some(d) => {
                self.value = self.value * rx as i64 + d as i64;
                Ok(())
            },
            None => Err(Error::invalid_char(c))
        }
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
        let sign_factor: i64 = if let Some(sign) = self.sign {
            sign as i64
        } else {
            1
        };
        self.value * sign_factor
    }

    fn seen_exact(&self) -> bool { self.exact.is_some() }
    fn seen_radix(&self) -> bool { self.radix.is_some() }
    fn seen_sign(&self) -> bool { self.sign.is_some() }

    fn radix_value(&self) -> u8 {
        let rx = match self.radix {
            Some(r) => r,
            None => Radix::Dec,
        };
        rx as u8
    }
}
