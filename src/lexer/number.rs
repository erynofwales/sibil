/* number.rs
 * Eryn Wells <eryn@erynwells.me>
 */

use types::number::Number;

#[derive(Debug)]
pub enum Radix { Bin, Oct, Dec, Hex }

#[derive(PartialEq, Debug)]
pub enum Sign { Pos, Neg }

#[derive(PartialEq, Debug)]
pub enum Exactness { Exact, Inexact }

#[derive(Debug)]
pub struct NumberBuilder {
    exact: Exactness,
    radix: Radix,
    sign: Sign,
    value: f64,
    point: u32,
}

impl NumberBuilder {
    pub fn new() -> NumberBuilder {
        NumberBuilder {
            exact: Exactness::Inexact,
            radix: Radix::Dec,
            sign: Sign::Pos,
            value: 0.0,
            point: 0,
        }
    }

    pub fn exact<'a>(&'a mut self, ex: Exactness) -> &'a mut NumberBuilder {
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
        println!("value = {}, point = {}", self.value, self.point);
        self
    }

    pub fn resolve(&self) -> Number {
        // TODO: Convert fields to Number type.
        let value = if self.point > 0 { self.value / 10u32.pow(self.point) as f64 } else { self.value };
        let value = if self.sign == Sign::Neg { value * -1.0 } else { value };
        Number::from_float(value)
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
    pub fn from_char(c: char) -> Option<Radix> {
        match c {
            'b' => Some(Radix::Bin),
            'o' => Some(Radix::Oct),
            'd' => Some(Radix::Dec),
            'h' => Some(Radix::Hex),
            _ => None,
        }
    }

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

impl Sign {
    pub fn from_char(c: char) -> Option<Sign> {
        match c {
            '+' => Some(Sign::Pos),
            '-' => Some(Sign::Neg),
            _ => None,
        }
    }
}

impl Exactness {
    pub fn from_char(c: char) -> Option<Exactness> {
        match c {
            'i' => Some(Exactness::Inexact),
            'e' => Some(Exactness::Exact),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_integers() {
        let mut b = NumberBuilder::new();
        b.extend_value('3');
        assert_eq!(b.resolve().value, 3.0);
        b.extend_value('4');
        assert_eq!(b.resolve().value, 34.0);
    }

    #[test]
    fn builds_negative_integers() {
        let num = NumberBuilder::new().sign(Sign::Neg).extend_value('3').resolve();
        assert_eq!(num.value, -3.0);
    }

    #[test]
    fn builds_pointy_numbers() {
        let mut b = NumberBuilder::new();
        b.extend_value('5');
        assert_eq!(b.resolve().value, 5.0);
        b.extend_decimal_value('3');
        assert_eq!(b.resolve().value, 5.3);
        b.extend_decimal_value('4');
        assert_eq!(b.resolve().value, 5.34);
    }

    #[test]
    fn builds_hex() {
        let mut b = NumberBuilder::new();
        b.radix(Radix::Hex).extend_value('4');
        assert_eq!(b.resolve().value, 0x4 as f64);
        b.extend_value('A');
        assert_eq!(b.resolve().value, 0x4A as f64);
        b.extend_value('6');
        assert_eq!(b.resolve().value, 0x4A6 as f64);
    }
}
