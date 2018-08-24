/* types/src/symbol.rs
 * Eryn Wells <eryn@erynwells.me>
 */

use std::any::Any;
use std::fmt;
use object::Object;
use super::*;

pub struct Sym(String);

impl Sym {
    pub fn new(value: String) -> Sym {
        Sym(value)
    }
}

impl Object for Sym {
    fn as_any(&self) -> &Any { self }
    fn as_pair(&self) -> Option<&Pair> { None }
    fn as_sym(&self) -> Option<&Sym> { Some(self) }
}

impl fmt::Display for Sym {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Debug for Sym {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}
