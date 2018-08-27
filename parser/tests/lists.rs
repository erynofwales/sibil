/* parser/tests/lists.rs
 * Eryn Wells <eryn@erynwells.me>
 */

//! Tests of lists of various sorts.

extern crate sibillexer;
extern crate sibilparser;
extern crate sibiltypes;

use sibillexer::{Lex, Token};
use sibillexer::Result as LexerResult;
use sibilparser::Parser;
use sibiltypes::{Obj, Pair, Sym};

#[test]
fn list_of_four_tokens() {
    let tokens = vec![Ok(Lex::new(Token::LeftParen, "(", 0, 0)),
                      Ok(Lex::new(Token::Id, "ab", 0, 0)),
                      Ok(Lex::new(Token::Id, "cd", 0, 0)),
                      Ok(Lex::new(Token::Id, "ef", 0, 0)),
                      Ok(Lex::new(Token::Id, "gh", 0, 0)),
                      Ok(Lex::new(Token::RightParen, ")", 0, 0))].into_iter();
    let mut parser = Parser::new(tokens);

    let ex_list = Obj::new(
        Pair::new(Obj::new(Sym::with_str("ab")), Obj::new(
                Pair::new(Obj::new(Sym::with_str("cd")), Obj::new(
                        Pair::new(Obj::new(Sym::with_str("ef")), Obj::new(
                                Pair::new(Obj::new(Sym::with_str("gh")), Obj::Null))))))));
    
    assert_eq!(parser.next(), Some(Ok(ex_list)));
    assert_eq!(parser.next(), None);
}
