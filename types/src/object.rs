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

#[derive(Debug)]
pub enum ObjectPtr {
    /// Absence of a value. A null pointer.
    Null,
    /// A pointer to an object.
    Ptr(Box<Object>),
}

#[derive(Debug)]
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
                write!(f, "{}", n)
            }

            Object::Pair(ref car, ref cdr) => {
                // TODO: There are rules for printing pairs...
                // Print a dot before the cdr iff it's anything but Null or another Pair.
                write!(f, "({}", car).and_then(|_| match cdr {
                    &ObjectPtr::Null => write!(f, ")"),
                    &ObjectPtr::Ptr(ref ptr) => match ptr.deref() {
                        &Object::Pair(_, _) => write!(f, "{}", ptr),
                        _ => write!(f, " . {})", ptr)
                    }
                })
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
