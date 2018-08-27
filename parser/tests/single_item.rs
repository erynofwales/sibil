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
use sibiltypes::{Bool, Obj, Pair, Sym};

#[test]
fn single_sym() {
    let lex: LexerResult = Ok(Lex::new(Token::Id, "abc", 0, 0));
    let tokens = vec![lex].into_iter();
    let mut parser = Parser::new(tokens);
    assert_eq!(parser.next(), Some(Ok(Obj::new(Sym::with_str("abc")))));
    assert_eq!(parser.next(), None);
}

#[test]
fn single_pair() {
    let tokens = vec![Ok(Lex::new(Token::LeftParen, "(", 0, 0)),
                      Ok(Lex::new(Token::RightParen, ")", 0, 0))].into_iter();
    let mut parser = Parser::new(tokens);
    assert_eq!(parser.next(), Some(Ok(Obj::Null)));
    assert_eq!(parser.next(), None);
}

#[test]
fn single_bool() {
    let lex: LexerResult = Ok(Lex::new(Token::Bool(true), "#t", 0, 0));
    let tokens = vec![lex].into_iter();
    let mut parser = Parser::new(tokens);
    assert_eq!(parser.next(), Some(Ok(Obj::new(Bool::True))));
    assert_eq!(parser.next(), None);
}
