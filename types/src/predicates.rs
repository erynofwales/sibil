/* types/src/predicates.rs
 * Eryn Wells <eryn@erynwells.me>
 */

pub trait IsNull {
    /// Is this thing null?
    fn is_null(&self) -> bool { false }
}

pub trait IsBool {
    /// Is this thing a boolean?
    fn is_bool(&self) -> bool { false }
}

pub trait IsChar {
    /// Is this thing a char?
    fn is_char(&self) -> bool { false }
}

pub trait IsNumber {
    /// Is this thing a number?
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
