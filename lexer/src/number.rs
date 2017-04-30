/* number.rs
 * Eryn Wells <eryn@erynwells.me>
 */

use sibiltypes::Object;
use sibiltypes::number::{Number, Exact};
use char::FromChar;

#[derive(Debug)]
pub enum Radix { Bin, Oct, Dec, Hex }

#[derive(Eq, PartialEq, Debug)]
pub enum Sign { Pos, Neg }

#[derive(Debug)]
pub struct NumberBuilder {
    exact: Exact,
    radix: Radix,
    sign: Sign,
    value: f64,
    point: u32,
}

impl NumberBuilder {
    pub fn new() -> NumberBuilder {
        NumberBuilder {
            exact: Exact::Yes,
            radix: Radix::Dec,
            sign: Sign::Pos,
            value: 0.0,
            point: 0,
        }
    }

    pub fn exact<'a>(&'a mut self, ex: Exact) -> &'a mut NumberBuilder {
        self.exact = ex;
        self
    }

    pub fn radix<'a>(&'a mut self, r: Radix) -> &'a mut NumberBuilder {
        self.radix = r;
        self
    }

    pub fn sign<'a>(&'a mut self, s: Sign) -> &'a mut NumberBuilder {
        self.sign = s;
        self
    }

    pub fn extend_value<'a>(&'a mut self, digit: char) -> &'a mut Self {
        if let Some(place) = NumberBuilder::place_value(digit) {
            self.value = self.radix.float_value() * self.value + place;
        }
        else {
            // TODO: Indicate an error.
        }
        self
    }

    pub fn extend_decimal_value<'a>(&'a mut self, digit: char) -> &'a mut Self {
        self.extend_value(digit);
        self.point += 1;
        self
    }

    pub fn resolve(&self) -> Number {
        // TODO: Convert fields to Number type.
        let value = if self.point > 0 { self.value / 10u32.pow(self.point) as f64 } else { self.value };
        let value = if self.sign == Sign::Neg { value * -1.0 } else { value };
        // TODO: Use an integer if we can.
        Number::from_float(value, self.exact)
    }

    pub fn radix_value(&self) -> u32 {
        self.radix.value()
    }

    fn place_value(digit: char) -> Option<f64> {
        match digit {
            '0' ... '9' => Some((digit as u32 - '0' as u32) as f64),
            'a' ... 'f' => Some((digit as u32 - 'a' as u32 + 10) as f64),
            'A' ... 'F' => Some((digit as u32 - 'A' as u32 + 10) as f64),
            _ => None,
        }
    }
}

impl Radix {
    pub fn value(&self) -> u32 {
        match *self {
            Radix::Bin => 2,
            Radix::Oct => 8,
            Radix::Dec => 10,
            Radix::Hex => 16,
        }
    }

    pub fn float_value(&self) -> f64 {
        self.value() as f64
    }
}

impl FromChar for Radix {
    fn from_char(c: char) -> Option<Radix> {
        match c {
            'b' => Some(Radix::Bin),
            'o' => Some(Radix::Oct),
            'd' => Some(Radix::Dec),
            'h' => Some(Radix::Hex),
            _ => None,
        }
    }
}

impl FromChar for Sign {
    fn from_char(c: char) -> Option<Sign> {
        match c {
            '+' => Some(Sign::Pos),
            '-' => Some(Sign::Neg),
            _ => None,
        }
    }
}

impl FromChar for Exact {
    fn from_char(c: char) -> Option<Exact> {
        match c {
            'i' => Some(Exact::No),
            'e' => Some(Exact::Yes),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use sibiltypes::Number;
    use super::*;

    #[test]
    fn builds_integers() {
        let mut b = NumberBuilder::new();
        b.extend_value('3');
        assert_eq!(b.resolve(), Number::from_int(3, true));
        b.extend_value('4');
        assert_eq!(b.resolve(), Number::from_int(34, true));
    }

    #[test]
    fn builds_negative_integers() {
        let num = NumberBuilder::new().sign(Sign::Neg).extend_value('3').resolve();
        assert_eq!(num, Number::from_int(-3, true));
    }

    #[test]
    fn builds_pointy_numbers() {
        let mut b = NumberBuilder::new();
        b.extend_value('5');
        assert_eq!(b.resolve(), Number::from_int(5, true));
        b.extend_decimal_value('3');
        assert_eq!(b.resolve(), Number::from_float(5.3, false));
        b.extend_decimal_value('4');
        assert_eq!(b.resolve(), Number::from_float(5.34, false));
    }

    #[test]
    fn builds_hex() {
        let mut b = NumberBuilder::new();
        b.radix(Radix::Hex).extend_value('4');
        assert_eq!(b.resolve(), Number::from_int(0x4, true));
        b.extend_value('A');
        assert_eq!(b.resolve(), Number::from_int(0x4A, true));
        b.extend_value('6');
        assert_eq!(b.resolve(), Number::from_int(0x4A6, true));
    }
}
