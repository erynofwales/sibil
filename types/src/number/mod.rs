/* types/src/number/mod.rs
 * Eryn Wells <eryn@erynwells.me>
 */

/// # Numbers
///
/// Scheme numbers are complex, literally.

pub mod integer;
pub mod rational;

pub use self::integer::Integer;
pub use self::rational::Rational;

use std::any::Any;
use std::fmt::Debug;
use std::ops::Deref;

use super::value::*;

type Int = i64;
type Flt = f64;

trait Number: Debug + IsBool + IsChar + IsNumber + Value {
    fn convert_down(&self) -> Option<Box<Number>>;
}

impl Value for Box<Number> {
    fn as_value(&self) -> &Value { self.deref().as_value() }
}

impl IsBool for Box<Number> { }
impl IsChar for Box<Number> { }
impl IsNumber for Box<Number> { }

impl ValueEq for Box<Number> {
    fn eq(&self, other: &Value) -> bool {
        self.deref().eq(other)
    }
    fn as_any(&self) -> &Any { self }
}

struct Real(Flt);
struct Complex<'a>(&'a Number, &'a Number);

