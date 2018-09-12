/* lexer/tests/numbers.rs
 * Eryn Wells <eryn@erynwells.me>
 */

//! Tests for lexing numbers.

extern crate sibillexer;

use sibillexer::{Lexer, Lex, Token};

#[test]
fn ints_simple() {
    let mut lex = Lexer::new("23 42 0".chars());
    assert_eq!(lex.next(), Some(Ok(Lex::new(Token::Num(23), "23", 0, 0))));
    assert_eq!(lex.next(), Some(Ok(Lex::new(Token::Num(42), "42", 0, 3))));
    assert_eq!(lex.next(), Some(Ok(Lex::new(Token::Num(0), "0", 0, 6))));
    assert_eq!(lex.next(), None);
}

#[test]
fn ints_negative() {
    let mut lex = Lexer::new("-56".chars());
    assert_eq!(lex.next(), Some(Ok(Lex::new(Token::Num(-56), "-56", 0, 0))));
    assert_eq!(lex.next(), None);
}

#[test]
fn ints_alternative_bases() {
    let mut lex = Lexer::new("#x2A #b11001 #o56 #d78".chars());
    assert_eq!(lex.next(), Some(Ok(Lex::new(Token::Num(0x2A), "#x2A", 0, 0))));
    assert_eq!(lex.next(), Some(Ok(Lex::new(Token::Num(0b11001), "#b11001", 0, 5))));
    assert_eq!(lex.next(), Some(Ok(Lex::new(Token::Num(0o56), "#o56", 0, 13))));
    assert_eq!(lex.next(), Some(Ok(Lex::new(Token::Num(78), "#d78", 0, 18))));
    assert_eq!(lex.next(), None);
}
