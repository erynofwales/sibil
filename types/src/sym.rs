/* types/src/symbol.rs
 * Eryn Wells <eryn@erynwells.me>
 */

use std::any::Any;
use std::ops::Deref;
use std::fmt;
use object::Object;
use super::*;

#[derive(Debug, PartialEq)]
pub struct Sym(String);

impl Sym {
    /// Creates a Sym with the given String.
    pub fn new(value: String) -> Sym {
        Sym(value)
    }

    /// Makes a copy of the input `&str` and creates a Sym with it.
    pub fn with_str(value: &str) -> Sym {
        Sym(value.to_string())
    }
}

impl Object for Sym {
    fn as_any(&self) -> &Any { self }
    fn as_sym(&self) -> Option<&Sym> { Some(self) }
}

impl fmt::Display for Sym {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl PartialEq<Obj> for Sym {
    fn eq(&self, rhs: &Obj) -> bool {
        match rhs {
            Obj::Null => false,
            Obj::Ptr(ref inner) => {
                if let Some(rhs_sym) = inner.deref().as_sym() {
                    self.0 == rhs_sym.0
                } else {
                    false
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Sym;

    #[test]
    fn syms_with_the_same_name_are_equal() {
        let a = Sym::with_str("abc");
        let b = Sym::with_str("abc");
        assert_eq!(a, b);
    }
}
