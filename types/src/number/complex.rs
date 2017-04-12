/* types/src/number/complex.rs
 * Eryn Wells <eryn@erynwells.me>
 */

use number::Real;
use value::*;

#[derive(Debug, PartialEq)]
pub struct Complex {
    real: Real,
    imag: Real
}

impl IsBool for Complex { }
impl IsChar for Complex { }

impl IsNumber for Complex {
    fn is_complex(&self) -> bool { true }
}
