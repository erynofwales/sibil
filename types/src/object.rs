/* types/value.rs
 * Eryn Wells <eryn@erynwells.me>
 */

use std::fmt;
use std::any::Any;

type ObjectPtr = Option<Box<Object>>;

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
    Pair(ObjectPtr, ObjectPtr),
    //Procedure/*( TODO: Something )*/,
    //Record/*( TODO: Something )*/,
    String(String),
    Symbol(String),
    Vector(Vec<ObjectPtr>),
}

pub trait IsNull {
    fn is_null(&self) -> bool { false }
    fn is_eof(&self) -> bool { false }
}

pub trait IsBool {
    /// Should return `true` if this Value is a Bool.
    fn is_bool(&self) -> bool { false }
}

pub trait IsChar {
    /// Should return `true` if this Value is a Char.
    fn is_char(&self) -> bool { false }
}

pub trait IsNumber {
    /// Should return `true` if this Value is any kind of number.
    fn is_number(&self) -> bool { self.is_complex() || self.is_real() || self.is_rational() || self.is_integer() }
    /// Should return `true` if this Value is a complex number type.
    fn is_complex(&self) -> bool { self.is_real() }
    /// Should return `true` if this Value is a real number type.
    fn is_real(&self) -> bool { self.is_rational() }
    /// Should return `true` if this Value is a rational number type.
    fn is_rational(&self) -> bool { self.is_integer() }
    /// Should return `true` if this Value is a integer number type.
    fn is_integer(&self) -> bool { false }
}
