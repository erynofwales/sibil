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
