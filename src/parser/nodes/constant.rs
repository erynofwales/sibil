/* parser/nodes/constant.rs
 * Eryn Wells <eryn@erynwells.me>
 */

use std::fmt;
use types::{Boolean, Number};
use super::TreeDebug;
use super::expression::Expression;

pub trait ConstantValue {}
impl ConstantValue for Boolean {}
impl ConstantValue for Number {}

pub struct Constant<'a, V: ConstantValue> {
    parent: Option<&'a Expression>,
    value: V
}

impl<'a, V: ConstantValue> Constant<'a, V> {
    pub fn new(value: V) -> Constant<'a, V> {
        Constant { parent: None, value: value }
    }
}

impl<'a, V: ConstantValue + fmt::Debug> fmt::Debug for Constant<'a, V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Constant {{ {:?} }}", self.value)
    }
}
