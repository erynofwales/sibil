/* parser/src/list.rs
 * Eryn Wells <eryn@erynwells.me>
 */

extern crate sibillexer;
extern crate sibiltypes;

use sibillexer::Lexer;
use sibillexer::Token;
use sibiltypes::Value;
use super::Parsable;
use super::ParseError;
use super::Result;

pub enum SExpression {
    Value(Box<Value>),
    List(Vec<SExpression>),
}

impl Parsable for SExpression {
    fn parse(lexer: &Lexer) -> Result<SExpression> {
        Err(ParseError{ })
    }
}
