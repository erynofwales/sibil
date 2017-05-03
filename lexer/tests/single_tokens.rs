/* lexer/tests/single_token.rs
 * Eryn Wells <eryn@erynwells.me>
 */

//! Tests that single tokens are matches by the lexer.

extern crate sibillexer;

use sibillexer::{Lexer, Token};

#[test]
fn lexer_finds_left_paren() {
    let mut lex = Lexer::new("(".chars());
    assert_eq!(lex.next(), Some(Ok(Token::LeftParen)));
    assert_eq!(lex.next(), None);
}

#[test]
fn lexer_finds_right_paren() {
    let mut lex = Lexer::new(")".chars());
    assert_eq!(lex.next(), Some(Ok(Token::RightParen)));
    assert_eq!(lex.next(), None);
}

#[test]
fn lexer_finds_id() {
    let mut lex = Lexer::new("abc".chars());
    assert_eq!(lex.next(), Some(Ok(Token::Id("abc".to_string()))));
    assert_eq!(lex.next(), None);
}
