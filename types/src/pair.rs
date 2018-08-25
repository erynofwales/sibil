/* types/src/pair.rs
 * Eryn Wells <eryn@erynwells.me>
 */

use std::any::Any;
use std::fmt;
use super::*;
use object::Object;

#[derive(Debug, PartialEq)]
pub struct Pair {
    pub car: Obj,
    pub cdr: Obj
}

impl Pair {
    pub fn new(car: Obj, cdr: Obj) -> Pair {
        Pair { car, cdr }
    }

    pub fn empty() -> Pair {
        Pair { car: Obj::Null, cdr: Obj::Null }
    }

    pub fn with_car(car: Obj) -> Pair {
        Pair { car: car, cdr: Obj::Null }
    }

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
}

impl fmt::Display for Pair {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(").and_then(|_| self.fmt_pair(f))
                      .and_then(|_| write!(f, ")"))
    }
}

impl PartialEq<Obj> for Pair {
    fn eq(&self, rhs: &Obj) -> bool {
        match rhs {
            Obj::Null => false,
            Obj::Ptr(ref rhs) => {
                if let Some(rhs_pair) = rhs.as_pair() {
                    let car_eq = self.car == rhs_pair.car;
                    let cdr_eq = self.cdr == rhs_pair.cdr;
                    car_eq && cdr_eq
                } else {
                    false
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Pair;

    #[test]
    fn empty_pairs_are_equal() {
        let a = Pair::empty();
        let b = Pair::empty();
        assert_eq!(a, b);
    }
}
