/* lexer/src/states/number.rs
 * Eryn Wells <eryn@erynwells.me>
 */

use std::collections::HashSet;
use chars::Lexable;
use super::{Resume, State, StateResult, Token};

trait NumberLexable {
    /// Returns the value of this character interpreted as the indicator for a
    /// base. In Scheme, you indicate the base of a number by prefixing it with
    /// #[bodx].
    fn base_value(&self) -> Option<Base>;
    /// Returns the value of the character interpreted as a numerical digit.
    fn digit_value(&self) -> Option<u8>;
    fn sign_value(&self) -> Option<Sign>;
    fn is_dot(&self) -> bool;
    fn is_hash(&self) -> bool;
    fn is_sign(&self) -> bool;
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Base { Bin = 2, Oct = 8, Dec = 10, Hex = 16 }

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Sign { Neg = -1, Pos = 1 }

#[derive(Copy, Clone, Debug)]
pub struct Builder {
    base: Option<Base>,
    sign: Option<Sign>,
    value: i64
}

#[derive(Debug)] pub struct BeginState(Builder);
#[derive(Debug)] pub struct DigitState(Builder);
#[derive(Debug)] pub struct HashState(Builder);
#[derive(Debug)] pub struct SignState(Builder);

impl Base {
    pub fn contains(&self, digit: u8) -> bool {
        digit < (*self as u8)
    }
}

impl Builder {
    pub fn new() -> Builder {
        Builder {
             base: None,
             sign: None,
             value: 0
        }
    }

    fn base(&self) -> Base {
        match self.base {
            Some(b) => b,
            None => Base::Dec
        }
    }

    fn sign(&self) -> Sign {
        match self.sign {
            Some(s) => s,
            None => Sign::Pos
        }
    }

    fn push_base(&mut self, base: Base) {
        self.base = Some(base);
    }

    fn push_sign(&mut self, sign: Sign) {
        self.sign = Some(sign);
    }

    fn push_digit(&mut self, digit: u8) {
        self.value = self.value * self.base_value() as i64 + digit as i64;
    }

    fn resolve(&self) -> i64 {
        let sign_factor: i64 = if let Some(sign) = self.sign { sign as i64 } else { 1 };
        self.value * sign_factor
    }

    fn seen_base(&self) -> bool { self.base.is_some() }

    fn base_value(&self) -> u8 { self.base() as u8 }
}

impl BeginState {
    pub fn new() -> BeginState  {
        BeginState (Builder::new())
    }
}

impl State for BeginState  {
    fn lex(&mut self, c: char) -> StateResult {
        match c {
            c if c.is_hash() => StateResult::advance(Box::new(HashState(self.0))),
            c if c.is_sign() => {
                self.0.push_sign(c.sign_value().unwrap());
                StateResult::advance(Box::new(SignState(self.0)))
            },
            c if c.is_digit(self.0.base_value() as u32) => {
                let value = c.digit_value().unwrap();
                if self.0.base().contains(value) {
                    self.0.push_digit(value);
                    StateResult::advance(Box::new(DigitState(self.0)))
                } else {
                    StateResult::fail(format!("invalid digit for current base: {}", c).as_str())
                }
            },
            _ => StateResult::fail(format!("invalid char: {}", c).as_str())
        }
    }

    fn none(&mut self) -> Result<Option<Token>, String> {
        // TODO: Implement.
        Err("blah".to_string())
    }
}

impl State for HashState {
    fn lex(&mut self, c: char) -> StateResult {
        if let Some(base) = c.base_value() {
            if !self.0.seen_base() {
                self.0.push_base(base);
                StateResult::advance(Box::new(BeginState (self.0)))
            } else {
                StateResult::fail("got base again, despite already having one")
            }
        } else {
            StateResult::fail(format!("invalid char: {}", c).as_str())
        }
    }

    fn none(&mut self) -> Result<Option<Token>, String> {
        // TODO: Implement.
        Err("blah".to_string())
    }
}

impl SignState {
    pub fn initials() -> HashSet<char> {
        let mut inits = HashSet::new();
        inits.insert('+');
        inits.insert('-');
        inits
    }
}

impl State for SignState {
    fn lex(&mut self, c: char) -> StateResult {
        if let Some(digit) = c.digit_value() {
            if self.0.base().contains(digit) {
                self.0.push_digit(digit);
                StateResult::advance(Box::new(DigitState(self.0)))
            } else {
                StateResult::fail(format!("invalid digit for current base: {}", c).as_str())
            }
        } else {
            StateResult::fail(format!("invalid char: {}", c).as_str())
        }
    }

    fn none(&mut self) -> Result<Option<Token>, String> {
        // TODO: Implement.
        Err("blah".to_string())
    }
}

impl DigitState {
    pub fn initials() -> HashSet<char> {
        let foldp = |acc: HashSet<char>, x: u8| {
            let c = char::from(x);
            acc.insert(c);
            acc
        };
        '0'..='9'.chain(('a' as u8)..=('f' as u8)).fold(HashSet::new(), foldp)
    }
}

impl State for DigitState {
    fn lex(&mut self, c: char) -> StateResult {
        if let Some(digit) = c.digit_value() {
            if self.0.base().contains(digit) {
                self.0.push_digit(digit);
                StateResult::Continue
            } else {
                StateResult::fail(format!("invalid digit for current base: {}", c).as_str())
            }
        } else if c.is_identifier_delimiter() {
            StateResult::emit(Token::Num(self.0.resolve()), Resume::Here)
        } else {
            StateResult::fail(format!("invalid char: {}", c).as_str())
        }
    }

    fn none(&mut self) -> Result<Option<Token>, String> {
        // TODO: Implement.
        Err("blah".to_string())
    }
}

impl NumberLexable for char {
    fn base_value(&self) -> Option<Base> {
        match *self {
            'b' => Some(Base::Bin),
            'o' => Some(Base::Oct),
            'd' => Some(Base::Dec),
            'x' => Some(Base::Hex),
            _ => None
        }
    }

    fn digit_value(&self) -> Option<u8> {
        let ascii_value = *self as u32;
        match *self {
            '0'...'9' => Some((ascii_value - '0' as u32) as u8),
            'a'...'f' => Some((ascii_value - 'a' as u32 + 10) as u8),
            'A'...'F' => Some((ascii_value - 'A' as u32 + 10) as u8),
            _ => None
        }
    }

    fn sign_value(&self) -> Option<Sign> {
        match *self {
            '+' => Some(Sign::Pos),
            '-' => Some(Sign::Neg),
            _ => None
        }
    }

    fn is_dot(&self) -> bool { *self == '.' }
    fn is_hash(&self) -> bool { *self == '#' }
    fn is_sign(&self) -> bool { self.sign_value().is_some() }
}
