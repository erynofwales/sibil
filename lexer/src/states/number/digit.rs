/* lexer/src/states/number/digit.rs
 * Eryn Wells <eryn@erynwells.me>
 */

use chars::Lexable;
use error::Error;
use states::{State, StateResult, Resume};
use states::number::{Builder, Radix};
use token::Token;

#[derive(Debug)] pub struct Digit(Builder);

impl Digit {
    pub fn new(b: Builder) -> Digit {
        Digit(b)
    }

    pub fn with_char(b: Builder, c: char) -> Option<Digit> {
        let mut b = b.clone();
        if !b.seen_radix() {
            b.push_radix(Radix::Dec);
        }
        match b.push_digit(c) {
            Ok(_) => Some(Digit::new(b)),
            // TODO: Deal with this error properly. Don't just ignore it.
            Err(_) => None,
        }
    }
}

impl State for Digit {
    fn lex(&mut self, c: char) -> StateResult {
        if self.0.push_digit(c).is_ok() {
            StateResult::Continue
        } else if c.is_identifier_delimiter() {
            StateResult::emit(Token::Num(self.0.resolve()), Resume::Here)
        } else {
            StateResult::fail(Error::invalid_char(c))
        }
    }

    fn none(&mut self) -> Result<Option<Token>, Error> {
        Ok(Some(Token::Num(self.0.resolve())))
    }
}
