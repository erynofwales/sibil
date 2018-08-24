/* types/src/pair.rs
 * Eryn Wells <eryn@erynwells.me>
 */

use std::any::Any;
use std::fmt;
use super::*;
use object::Object;

pub struct Pair {
    car: Obj,
    cdr: Obj
}

impl Pair {
    fn fmt_pair(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let r = write!(f, "{}", self.car);
        r.and_then(|r| match self.cdr {
            Obj::Null => Ok(r),  // Don't write anything.
            Obj::Ptr(ref next) => {
                match next.as_pair() {
                    Some(next_pair) => write!(f, " ").and_then(|_| next_pair.fmt_pair(f)),
                    None => write!(f, " . {}", next)
                }
            }
        })
    }
}

impl Object for Pair {
    fn as_any(&self) -> &Any { self }
    fn as_pair(&self) -> Option<&Pair> { Some(self) }
    fn as_sym(&self) -> Option<&Sym> { None }
}

impl fmt::Display for Pair {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(").and_then(|_| self.fmt_pair(f))
                      .and_then(|_| write!(f, ")"))
    }
}
