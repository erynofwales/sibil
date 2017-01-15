/* parser/nodes.rs
 * Eryn Wells <eryn@erynwells.me>
 */

use std::fmt;
use std::ops::Deref;

use lexer::Token;
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
    List { left: Token, expr: Vec<Box<Expression>>, right: Token },
}

impl fmt::Debug for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Expression::EOF => write!(f, "EOF"),
            Expression::Atom(ref value) => write!(f, "Atom{{ {:?} }}", value),
            Expression::List{ left: ref lt, ref expr, right: ref rt } => {
                write!(f, "{:?} {:?} {:?}", lt, expr, rt)
            },
        }
    }
}

impl PartialEq for Expression {
    fn eq(&self, other: &Expression) -> bool {
        match *self {
            Expression::EOF => self.eq_eof(other),
            Expression::Atom(ref value) => self.eq_atom(other, value.deref()),
            Expression::List { ref left, ref expr, ref right } => {
                self.eq_list(other, left, expr, right)
            },
        }
    }
}

impl Expression {
    fn eq_eof(&self, other: &Expression) -> bool {
        match *other {
            Expression::EOF => true,
            _ => false,
        }
    }

    fn eq_atom(&self, other: &Expression, value: &types::Value) -> bool {
        match *other {
            Expression::Atom(ref ovalue) => value == ovalue.deref(),
            _ => false,
        }
    }

    fn eq_list(&self, other: &Expression, left: &Token, expr: &Vec<Box<Expression>>, right: &Token) -> bool {
        match *other {
            Expression::List { left: ref olt, expr: ref oexpr, right: ref ort } => {
                let left_eq = left == olt;
                let right_eq = right == ort;
                let expr_eq = expr == oexpr;
                left_eq && expr_eq && right_eq
            },
            _ => false,
        }
    }
}
