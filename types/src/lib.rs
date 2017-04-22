//pub mod number;
mod bool;
//mod char;
mod number;
mod object;
mod predicates;

pub use object::Object;
pub use predicates::*;

#[cfg(test)]
mod tests {
    use bool::Bool;
    use char::Char;
    use value::*;

    #[test]
    fn booleans_and_chars_are_not_equal() {
        assert_ne!(Bool(true).as_value(), Char('a').as_value());
    }
}
