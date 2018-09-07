/* lexer/tests/single_token.rs
 * Eryn Wells <eryn@erynwells.me>
 */

//! Tests that single tokens are matches by the lexer.

extern crate sibillexer;

use sibillexer::{Lexer, Lex, Token};

#[test]
fn lexer_finds_left_paren() {
    let expected_lex = Lex::new(Token::LeftParen, "(", 0, 0);
    let mut lex = Lexer::new("(".chars());
    assert_eq!(lex.next(), Some(Ok(expected_lex)));
    assert_eq!(lex.next(), None);
}

#[test]
fn lexer_finds_right_paren() {
    let expected_lex = Lex::new(Token::RightParen, ")", 0, 0);
    let mut lex = Lexer::new(")".chars());
    assert_eq!(lex.next(), Some(Ok(expected_lex)));
    assert_eq!(lex.next(), None);
}

#[test]
fn lexer_finds_id() {
    let expected_lex = Lex::new(Token::Id, "abc", 0, 0);
    let mut lex = Lexer::new("abc".chars());
    assert_eq!(lex.next(), Some(Ok(expected_lex)));
    assert_eq!(lex.next(), None);
}

#[test]
fn bool_short_true() {
    let expected_lex = Lex::new(Token::Bool(true), "#t", 0, 0);
    let mut lex = Lexer::new("#t".chars());
    assert_eq!(lex.next(), Some(Ok(expected_lex)));
    assert_eq!(lex.next(), None);
}

#[test]
fn bool_short_false() {
    let expected_lex = Lex::new(Token::Bool(false), "#f", 0, 0);
    let mut lex = Lexer::new("#f".chars());
    assert_eq!(lex.next(), Some(Ok(expected_lex)));
    assert_eq!(lex.next(), None);
}

#[test]
fn bool_long_true() {
    let expected_lex = Lex::new(Token::Bool(true), "#true", 0, 0);
    let mut lex = Lexer::new("#true".chars());
    assert_eq!(lex.next(), Some(Ok(expected_lex)));
    assert_eq!(lex.next(), None);
}

#[test]
fn bool_long_false() {
    let expected_lex = Lex::new(Token::Bool(false), "#false", 0, 0);
    let mut lex = Lexer::new("#false".chars());
    assert_eq!(lex.next(), Some(Ok(expected_lex)));
    assert_eq!(lex.next(), None);
}

#[test]
fn bool_with_spaces() {
    // See issue #12
    let expected_lex = Lex::new(Token::Bool(false), "#f", 0, 2);
    let mut lex = Lexer::new("  #f  ".chars());
    assert_eq!(lex.next(), Some(Ok(expected_lex)));
    assert_eq!(lex.next(), None);
}

#[test]
fn simple_integers() {
    let mut lex = Lexer::new("23 42".chars());
    assert_eq!(lex.next(), Some(Ok(Lex::new(Token::Num(23), "23", 0, 0))));
    assert_eq!(lex.next(), Some(Ok(Lex::new(Token::Num(42), "42", 0, 3))));
    assert_eq!(lex.next(), None);
}

#[test]
fn integers_in_alternative_bases() {
    let mut lex = Lexer::new("#x2A #b11001".chars());
    assert_eq!(lex.next(), Some(Ok(Lex::new(Token::Num(0x2A), "#x2A", 0, 0))));
    assert_eq!(lex.next(), Some(Ok(Lex::new(Token::Num(0b11001), "#b11001", 0, 5))));
    assert_eq!(lex.next(), None);
}

#[test]
fn dot() {
    let mut lex = Lexer::new(".".chars());
    assert_eq!(lex.next(), Some(Ok(Lex::new(Token::Dot, ".", 0, 0))));
    assert_eq!(lex.next(), None);
}
