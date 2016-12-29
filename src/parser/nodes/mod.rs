/* node.rs
 * Eryn Wells <eryn@erynwells.me>
 */

use types::{Boolean, Number};

trait ConstantValue {}
impl ConstantValue for Boolean {}
impl ConstantValue for Number {}

struct Constant<V: ConstantValue> {
    value: V
}

impl<V: ConstantValue> Constant<V> {
    fn new(value: V) -> Constant<V> {
        Constant { value: value }
    }
}
