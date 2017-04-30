pub mod number;
pub mod char;

mod bool;
mod object;
mod predicates;

pub use object::Object;
pub use object::ObjectPtr;
pub use predicates::*;

#[cfg(test)]
mod tests {
    use super::Object;

    #[test]
    fn booleans_and_chars_are_not_equal() {
        assert_ne!(Object::Bool(true), Object::Char('a'));
    }
}
