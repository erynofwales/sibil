/* types/mod.rs
 * Eryn Wells <eryn@erynwells.me>
 */

pub mod bool;
pub mod char;
pub mod number;
pub mod value;

pub use self::bool::Bool;
pub use self::char::Char;
pub use self::number::Number;
pub use self::value::Value;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn booleans_are_equal() {
        assert_eq!(Bool::new(true), Bool::new(true));
        assert_eq!(Bool::new(false), Bool::new(false));
        assert_ne!(Bool::new(true), Bool::new(false));
    }

    #[test]
    fn equal_chars_are_equal() {
        assert_eq!(Char::new('a'), Char::new('a'));
        assert_eq!(Char::new('a').as_value(), Char::new('a').as_value());
    }

    #[test]
    fn booleans_and_chars_are_not_equal() {
        assert_ne!(Bool::new(true).as_value(), Char::new('a').as_value());
    }
}
