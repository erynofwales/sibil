/* types/src/object.rs
 * Eryn Wells <eryn@erynwells.me>
 */

//! # Objects
//!
//! All Scheme types implement the `Object` trait defined in this module. Most
//! references to objects are going to be through an `ObjectPtr`.
//!
//! ## Type Predicates
//!
//! Objects satisfy one (and only one) of several predicates which define the
//! available types in Scheme. These predicates are implemented as `is_*`
//! methods in a bunch of `Is*` traits defined below.

use std::ops::Deref;
use std::mem;
use std::any::Any;
use std::fmt;
use super::*;
use number::Number;

#[derive(Debug)]
pub enum Obj {
    Null,
    Ptr(Box<Object>)
}

pub trait Object:
    fmt::Debug +
    fmt::Display +
    PartialEq<Obj>
{
    /// Cast this Object to an Any.
    fn as_any(&self) -> &Any;
    /// Cast this Object to a Bool if possible.
    fn as_bool(&self) -> Option<&Bool> { None }
    /// Cast this Object to a Pair if possible.
    fn as_pair(&self) -> Option<&Pair> { None }
    /// Cast this Object to a Sym if possible.
    fn as_sym(&self) -> Option<&Sym> { None }
    /// Cast this Object to a Number if possible.
    fn as_num(&self) -> Option<&Number> { None }
}

impl Obj {
    pub fn new<T: Object + 'static>(obj: T) -> Obj {
        Obj::Ptr(Box::new(obj))
    }

    pub fn obj<'s, 'r: 's>(&'s self) -> Option<&'r (Object + 's)> {
        match self {
            Obj::Ptr(obj) => Some(obj.deref()),
            Obj::Null => None
        }
    }

    pub fn take(&mut self) -> Obj {
        // Stole Option's implementation of this. Handy. :D
        mem::replace(self, Obj::Null)
    }

    pub fn unbox_as<T: Object + 'static>(&self) -> Option<&T> {
        match self {
            Obj::Null => None,
            Obj::Ptr(obj) => obj.as_any().downcast_ref::<T>()
        }
    }

    pub fn is_null(&self) -> bool {
        match self {
            Obj::Null => true,
            _ => false
        }
    }
}

impl fmt::Display for Obj {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Obj::Null => write!(f, "()"),
            Obj::Ptr(obj) => write!(f, "{}", obj)
        }
    }
}

impl PartialEq for Obj {
    fn eq(&self, rhs: &Self) -> bool {
        match self {
            Obj::Null => rhs.is_null(),
            Obj::Ptr(ref inner) => inner.deref() == rhs
        }
    }
}

//#[derive(Debug, PartialEq)]
//pub enum Object {
//    ByteVector(Vec<u8>),
//    Char(char),
//    Number(Number),
//    Pair(ObjectPtr, ObjectPtr),
//    //Procedure/*( TODO: Something )*/,
//    //Record/*( TODO: Something )*/,
//    String(String),
//    Symbol(String),
//    Vector(Vec<ObjectPtr>),
//}
//
//impl ObjectPtr {
//    pub fn new(obj: Object) -> ObjectPtr {
//        ObjectPtr::Ptr(Box::new(obj))
//    }
//
//    pub fn new_pair() -> ObjectPtr {
//        ObjectPtr::new(Object::Pair(ObjectPtr::Null, ObjectPtr::Null))
//    }
//}
//
//impl fmt::Display for ObjectPtr {
//    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//        match *self {
//            ObjectPtr::Null => write!(f, "()"),
//            ObjectPtr::Ptr(ref bx) => write!(f, "{}", bx.deref()),
//        }
//    }
//}
//
//impl fmt::Display for Object {
//    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//        match *self {
//            Object::Bool(ref v) => {
//                let out = if *v { "#t" } else { "#f" };
//                write!(f, "{}", out)
//            },
//
//            Object::ByteVector(ref vec) => {
//                // TODO: Actually write the vector values.
//                write!(f, "#u8(").and_then(|_| write!(f, ")"))
//            },
//
//            Object::Char(ref c) => {
//                // TODO: This is not correct for all cases. See section 6.6 of the spec.
//                write!(f, "#\\{}", c)
//            },
//
//            Object::Number(ref n) => {
//                // TODO: Implement Display for Number
//                write!(f, "{}", n)
//            }
//
//            Object::Pair(ref car, ref cdr) => {
//                write!(f, "(").and_then(|_| Object::fmt_pair(car, cdr, f))
//                              .and_then(|_| write!(f, ")"))
//            },
//
//            Object::String(ref st) => {
//                write!(f, "\"{}\"", st)
//            },
//
//            Object::Symbol(ref sym) => {
//                write!(f, "{}", sym)
//            },
//
//            Object::Vector(ref vec) => {
//                // TODO: Actually write the vector values.
//                vec.iter().enumerate().fold(write!(f, "#("), |acc, (i, obj)| {
//                    let space = if i == (vec.len() - 1) { " " } else { "" };
//                    acc.and(write!(f, "{}{}", obj, space))
//                }).and(write!(f, ")"))
//            }
//        }
//    }
//}
//
//impl Object {
//    fn fmt_pair(car: &ObjectPtr, cdr: &ObjectPtr, f: &mut fmt::Formatter) -> fmt::Result {
//        write!(f, "{}", car).and_then(|r| match cdr {
//            &ObjectPtr::Null => Ok(r),  // Don't write anything.
//            &ObjectPtr::Ptr(ref ptr) => match ptr.deref() {
//                &Object::Pair(ref next_car, ref next_cdr) => {
//                    write!(f, " ").and_then(|_| Object::fmt_pair(next_car, next_cdr, f))
//                },
//                _ => write!(f, " . {}", ptr)
//            }
//        })
//    }
//}

#[cfg(test)]
mod tests {
    use super::Obj;

//    #[test]
//    fn display_bools() {
//        assert_eq!(format!("{}", Object::Bool(true)), "#t");
//        assert_eq!(format!("{}", Object::Bool(false)), "#f");
//    }
//
//    #[test]
//    fn display_simple_pair() {
//        let pair = Object::Pair(ObjectPtr::new(Object::Bool(true)), ObjectPtr::new(Object::Bool(false)));
//        assert_eq!(format!("{}", pair), "(#t . #f)");
//    }
//
//    #[test]
//    fn display_single_item_pair() {
//        let pair = Object::Pair(ObjectPtr::new(Object::Bool(true)), ObjectPtr::Null);
//        assert_eq!(format!("{}", pair), "(#t)");
//    }
//
//    #[test]
//    fn display_recursive_pair() {
//        let p1 = Object::Pair(ObjectPtr::new(Object::Bool(true)), ObjectPtr::Null);
//        let p2 = Object::Pair(ObjectPtr::new(Object::Bool(true)), ObjectPtr::new(p1));
//        assert_eq!(format!("{}", p2), "(#t #t)");
//    }
//
//    #[test]
//    fn display_improper_recursive_pair() {
//        let p1 = Object::Pair(ObjectPtr::new(Object::Bool(true)), ObjectPtr::new(Object::Bool(false)));
//        let p2 = Object::Pair(ObjectPtr::new(Object::Bool(true)), ObjectPtr::new(p1));
//        assert_eq!(format!("{}", p2), "(#t #t . #f)");
//    }
//
//    #[test]
//    fn display_string() {
//        assert_eq!(format!("{}", Object::String(String::from("Hello!"))), "\"Hello!\"");
//    }
}
