/* parser/nodes.rs
 * Eryn Wells <eryn@erynwells.me>
 */

use std::fmt;

use lexer;
use types;

pub struct Program {
    forms: Vec<Expression>
}

impl Program {
    pub fn new(forms: Vec<Expression>) -> Program {
        Program { forms: forms }
    }
}

pub enum Expression {
    EOF,
    Atom(Box<types::Value>),
    List { left: lexer::Token, expr: Vec<Box<Expression>>, right: lexer::Token },
}

impl fmt::Debug for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Expression::EOF => write!(f, "EOF"),
            Expression::Atom(ref value) => write!(f, "Atom{{ {:?} }}", value),
            Expression::List{ left: ref lt, expr: ref expr, right: ref rt } => {
                write!(f, "{:?} {:?} {:?}", lt, expr, rt)
            },
        }
    }
}
