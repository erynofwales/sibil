/* parser/nodes/constant.rs
 * Eryn Wells <eryn@erynwells.me>
 */

use std::fmt;
use types::{Boolean, Number};
use super::TreeDebug;

pub trait ConstantValue {}
impl ConstantValue for Boolean {}
impl ConstantValue for Number {}

#[derive(Debug)]
pub struct Constant<V: ConstantValue> {
    value: V
}

impl<V: ConstantValue + fmt::Debug> TreeDebug for Constant<V> {}

impl<V: ConstantValue> Constant<V> {
    pub fn new(value: V) -> Constant<V> {
        Constant { value: value }
    }
}
