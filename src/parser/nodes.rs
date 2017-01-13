/* parser/nodes.rs
 * Eryn Wells <eryn@erynwells.me>
 */

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
    List { left: lexer::Token, expression: Vec<Box<Expression>>, right: lexer::Token },
}
