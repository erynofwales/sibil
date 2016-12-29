/* parser/nodes/program.rs
 * Eryn Wells <eyrn@erynwells.me>
 */

use std::fmt;

use super::TreeDebug;
use super::Expression;

pub struct Program {
    forms: Vec<Box<Expression>>,
}

impl fmt::Debug for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.tree_fmt(f, 0)
    }
}

impl TreeDebug for Program {
    fn tree_fmt(&self, f: &mut fmt::Formatter, indent: u8) -> fmt::Result {
        let spaces = self.tree_indent(indent);
        let mut result = write!(f, "{}Program", spaces);
        for form in self.forms.iter() {
            if result.is_err() {
                break;
            }
            result = form.tree_fmt(f, indent + 2);
        }
        result
    }
}
