/* lexer/tests/expressions.rs
 * Eryn Wells <eryn@erynwells.me>
 */

extern crate sibillexer;

use sibillexer::{Lexer, Lex, Token};

#[test]
fn addition() {
    let mut lex = Lexer::new("(+ 3 4)".chars());
    assert_eq!(lex.next(), Some(Ok(Lex::new(Token::LeftParen, "(", 0, 0))));
    assert_eq!(lex.next(), Some(Ok(Lex::new(Token::Id, "+", 0, 1))));
    assert_eq!(lex.next(), Some(Ok(Lex::new(Token::Num(3), "3", 0, 3))));
    assert_eq!(lex.next(), Some(Ok(Lex::new(Token::Num(4), "4", 0, 5))));
    assert_eq!(lex.next(), Some(Ok(Lex::new(Token::RightParen, ")", 0, 6))));
    assert_eq!(lex.next(), None);
}

#[test]
fn subtraction() {
    let mut lex = Lexer::new("(- 3 4)".chars());
    assert_eq!(lex.next(), Some(Ok(Lex::new(Token::LeftParen, "(", 0, 0))));
    assert_eq!(lex.next(), Some(Ok(Lex::new(Token::Id, "-", 0, 1))));
    assert_eq!(lex.next(), Some(Ok(Lex::new(Token::Num(3), "3", 0, 3))));
    assert_eq!(lex.next(), Some(Ok(Lex::new(Token::Num(4), "4", 0, 5))));
    assert_eq!(lex.next(), Some(Ok(Lex::new(Token::RightParen, ")", 0, 6))));
    assert_eq!(lex.next(), None);
}
