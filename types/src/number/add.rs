

use std::ops::Add;
use super::{Int, Flt, Real};

impl Add for Real {
    type Output = Real;

    fn add(self, other: Real) -> Real {
        match other {
            Real::Integer(v) => self.add_integer(v),
            _ => Real::Integer(0)
        }
    }
}

impl Real {
    fn add_integer(self, v_other: Int) -> Real {
        match self {
            Real::Integer(v) => Real::Integer(v + v_other),
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
