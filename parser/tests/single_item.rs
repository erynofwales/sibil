/* parser/tests/single_item.rs
 * Eryn Wells <eryn@erynwells.me>
 */

//! Tests that the parser can handle inputs of single tokens.

extern crate sibillexer;
extern crate sibilparser;
extern crate sibiltypes;

use sibillexer::{Lex, Token};
use sibillexer::Result as LexerResult;
use sibilparser::Parser;
use sibiltypes::{Obj, Sym};

#[test]
fn single_sym() {
    let lex: LexerResult = Ok(Lex::new(Token::Id, "abc", 0, 0));
    let tokens = vec![lex].into_iter();
    let mut parser = Parser::new(tokens);
    assert_eq!(parser.next(), Some(Ok(Obj::new(Sym::with_str("abc")))));
    assert_eq!(parser.next(), None);
}
