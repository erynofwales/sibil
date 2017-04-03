/* types/mod.rs
 * Eryn Wells <eryn@erynwells.me>
 */

pub use self::bool::Bool;
pub use self::char::Char;
pub use self::number::Number;
use self::value::Value;

pub mod bool;
pub mod char;
pub mod number;
mod value;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn booleans_are_equal() {
        assert_eq!(Bool(true), Bool(true));
        assert_eq!(Bool(false), Bool(false));
        assert_ne!(Bool(true), Bool(false));
    }

    #[test]
    fn equal_chars_are_equal() {
        assert_eq!(Char('a'), Char('a'));
        assert_eq!(Char('a').as_value(), Char('a').as_value());
    }

    #[test]
    fn booleans_and_chars_are_not_equal() {
        assert_ne!(Bool(true).as_value(), Char('a').as_value());
    }
}
