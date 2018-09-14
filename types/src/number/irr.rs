/* types/src/number/irr.rs
 * Eryn Wells <eryn@erynwells.me>
 */

use std::any::Any;
use std::fmt;
use number::{Frac, Int, Number};
use object::{Obj, Object};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Irr(pub f64);

impl Irr {
    pub fn zero() -> Irr { Irr(0.0) }
}

impl fmt::Display for Irr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Int> for Irr {
    fn from(i: Int) -> Irr { Irr(i.0 as f64) }
}

impl From<Frac> for Irr {
    fn from(f: Frac) -> Irr {
        Irr(f.quotient())
    }
}

impl Number for Irr {
    fn as_int(&self) -> Option<Int> {
        if self.0.trunc() == self.0 {
            Some(Int(self.0.trunc() as i64))
        } else {
            None
        }
    }

    fn as_frac(&self) -> Option<Frac> {
        if !self.0.is_infinite() && !self.0.is_nan() {
            // TODO
            None
        } else {
            None
        }
    }

    fn is_zero(&self) -> bool { self.0 == 0.0 }
}

impl Object for Irr {
    fn as_any(&self) -> &Any { self }
    fn as_num(&self) -> Option<&Number> { Some(self) }
}

impl PartialEq<Obj> for Irr {
    fn eq<'a>(&self, rhs: &'a Obj) -> bool {
        match rhs.obj().and_then(Object::as_num) {
            Some(num) => self == num,
            None => false
        }
    }
}

impl<'a> PartialEq<Number + 'a> for Irr {
    fn eq(&self, rhs: &(Number + 'a)) -> bool {
        false
    }
}
