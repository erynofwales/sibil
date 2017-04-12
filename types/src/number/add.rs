

use std::ops::Add;
use super::{Int, Flt, Real};

impl Add for Real {
    type Output = Real;

    fn add(self, other: Real) -> Real {
        match (self, other) {
            (Real::Integer(v), Real::Integer(ov)) => Real::Integer(v + ov),
            _ => Real::Integer(0)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use number::Real;

    #[test]
    fn integer_add_works() {
        let result = Real::Integer(3) + Real::Integer(5);
        assert_eq!(result, Real::Integer(8));
    }
}
