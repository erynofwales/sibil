/* types/object.rs
 * Eryn Wells <eryn@erynwells.me>
 */

/// `Object` is the top-level type in Scheme's world. It is abstract -- no value is of `Object`
/// type -- but all types must implement it.
pub trait Object: IsBool + IsChar { }

pub trait IsBool {
    fn is_bool() -> bool { false }
}

pub trait isChar {
    fn is_char() -> bool { false }
}
