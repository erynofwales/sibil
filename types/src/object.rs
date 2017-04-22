/* types/src/object.rs
 * Eryn Wells <eryn@erynwells.me>
 */

//! # Objects
//!
//! All scheme types are represented by the `Object` enum defined in this
//! module. Most references to objects are going to be through an `ObjectPtr`.
//!
//! ## Type Predicates
//!
//! Objects satisfy one (and only one) of several predicates which define the
//! available types in Scheme. These predicates are implemented as `is_*`
//! methods in a bunch of `Is*` traits defined below.

use std::fmt;
use std::ops::Deref;
use number::Number;

#[derive(Debug, PartialEq)]
pub enum ObjectPtr {
    /// Absence of a value. A null pointer.
    Null,
    /// A pointer to an object.
    Ptr(Box<Object>),
}

#[derive(Debug, PartialEq)]
pub enum Object {
    Bool(bool),
    ByteVector(Vec<u8>),
    Char(char),
    Number(Number),
    Pair(ObjectPtr, ObjectPtr),
    //Procedure/*( TODO: Something )*/,
    //Record/*( TODO: Something )*/,
    String(String),
    Symbol(String),
    Vector(Vec<ObjectPtr>),
}

impl ObjectPtr {
    fn new(obj: Object) -> ObjectPtr { ObjectPtr::Ptr(Box::new(obj)) }
}

impl fmt::Display for ObjectPtr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ObjectPtr::Null => write!(f, "()"),
            ObjectPtr::Ptr(ref bx) => write!(f, "{}", bx.deref()),
        }
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Object::Bool(ref v) => {
                let out = if *v { "#t" } else { "#f" };
                write!(f, "{}", out)
            },

            Object::ByteVector(ref vec) => {
                // TODO: Actually write the vector values.
                write!(f, "#u8(").and_then(|_| write!(f, ")"))
            },

            Object::Char(ref c) => {
                // TODO: This is not correct for all cases. See section 6.6 of the spec.
                write!(f, "#\\{}", c)
            },

            Object::Number(ref n) => {
                // TODO: Implement Display for Number
                write!(f, "{:?}", n)
            }

            Object::Pair(ref car, ref cdr) => {
                write!(f, "(").and_then(|_| Object::fmt_pair(car, cdr, f))
                              .and_then(|_| write!(f, ")"))
            },

            Object::String(ref st) => {
                write!(f, "\"{}\"", st)
            },

            Object::Symbol(ref sym) => {
                write!(f, "{}", sym)
            },

            Object::Vector(ref vec) => {
                // TODO: Actually write the vector values.
                write!(f, "#(").and_then(|_| write!(f, ")"))
            }
        }
    }
}

impl Object {
    fn fmt_pair(car: &ObjectPtr, cdr: &ObjectPtr, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", car).and_then(|r| match cdr {
            &ObjectPtr::Null => Ok(r),  // Don't write anything.
            &ObjectPtr::Ptr(ref ptr) => match ptr.deref() {
                &Object::Pair(ref next_car, ref next_cdr) => {
                    write!(f, " ").and_then(|_| Object::fmt_pair(next_car, next_cdr, f))
                },
                _ => write!(f, " . {}", ptr)
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Object;
    use super::ObjectPtr;

    #[test]
    fn display_bools() {
        assert_eq!(format!("{}", Object::Bool(true)), "#t");
        assert_eq!(format!("{}", Object::Bool(false)), "#f");
    }

    #[test]
    fn display_simple_pair() {
        let pair = Object::Pair(ObjectPtr::new(Object::Bool(true)), ObjectPtr::new(Object::Bool(false)));
        assert_eq!(format!("{}", pair), "(#t . #f)");
    }

    #[test]
    fn display_single_item_pair() {
        let pair = Object::Pair(ObjectPtr::new(Object::Bool(true)), ObjectPtr::Null);
        assert_eq!(format!("{}", pair), "(#t)");
    }

    #[test]
    fn display_recursive_pair() {
        let p1 = Object::Pair(ObjectPtr::new(Object::Bool(true)), ObjectPtr::Null);
        let p2 = Object::Pair(ObjectPtr::new(Object::Bool(true)), ObjectPtr::new(p1));
        assert_eq!(format!("{}", p2), "(#t #t)");
    }

    #[test]
    fn display_improper_recursive_pair() {
        let p1 = Object::Pair(ObjectPtr::new(Object::Bool(true)), ObjectPtr::new(Object::Bool(false)));
        let p2 = Object::Pair(ObjectPtr::new(Object::Bool(true)), ObjectPtr::new(p1));
        assert_eq!(format!("{}", p2), "(#t #t . #f)");
    }

    #[test]
    fn display_string() {
        assert_eq!(format!("{}", Object::String(String::from("Hello!"))), "\"Hello!\"");
    }
}
