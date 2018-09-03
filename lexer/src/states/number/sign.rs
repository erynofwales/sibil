/* lexer/src/states/number/sign.rs
 * Eryn Wells <eryn@erynwells.me>
 */

use error::Error;
use states::{State, StateResult};
use states::number::Builder;
use states::number::Sign as Sgn;
use token::Token;

#[derive(Debug)] pub struct Sign(Builder);

impl Sign {
    pub fn new(b: Builder) -> Sign {
        Sign(b)
    }

    pub fn with_char(b: Builder, c: char) -> Option<Sign> {
        if !b.seen_sign() {
            match c {
                '+' => {
                    let mut b = b.clone();
                    b.push_sign(Sgn::Pos);
                    Some(Sign::new(b))
                },
                '-' => {
                    let mut b = b.clone();
                    b.push_sign(Sgn::Neg);
                    Some(Sign::new(b))
                },
                _ => None
            }
        } else {
            None
        }
    }
}

impl State for Sign {
    fn lex(&mut self, c: char) -> StateResult {
        StateResult::fail(Error::invalid_char(c))
    }

    fn none(&mut self) -> Result<Option<Token>, Error> {
        Err(Error::unexpected_eof())
    }
}
