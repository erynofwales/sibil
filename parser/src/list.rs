/* parser/src/list.rs
 * Eryn Wells <eryn@erynwells.me>
 */

extern crate sibillexer;
extern crate sibiltypes;

use sibillexer::{Lexer, Token};
use sibiltypes::{Bool, Char, Number, Value};
use super::{Parsable, ParseError, Result};

pub enum SExpression {
    Atom(Box<Value>),
    List(Vec<SExpression>),
}

impl Parsable for SExpression {
    fn parse(lexer: &Lexer) -> Result<SExpression> {
        Ok(SExpression::Atom(Box::new(Number::from_int(3, true))))
    }
}
