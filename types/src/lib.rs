mod object;
mod pair;
mod sym;

pub use object::Obj;
pub use pair::Pair;
pub use sym::Sym;

#[cfg(test)]
mod tests {
    use super::Object;

    #[test]
    fn booleans_and_chars_are_not_equal() {
        assert_ne!(Object::Bool(true), Object::Char('a'));
    }
}
