/* parser/nodes/program.rs
 * Eryn Wells <eyrn@erynwells.me>
 */

use std::fmt;

use super::TreeDebug;
use super::Constant;

pub struct Program {
    forms: Vec<Constant>,
}

impl TreeDebug for Program {
    fn tree_fmt(&self, f: fmt::Formatter, indent: u8) -> fmt::Result {
        let spaces = self.tree_indent(indent);
        let mut result = write!(f, "{}Program", spaces);
        for form in self.forms {
            if result.is_err() {
                break;
            }
            result = form.tree_fmt(f, indent + 2);
        }
        result
    }
}
