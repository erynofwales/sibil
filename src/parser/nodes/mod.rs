/* parser/nodes/mod.rs
 * Eryn Wells <eryn@erynwells.me>
 */

use std::fmt;

pub use self::constant::Constant;
pub use self::program::Program;

mod constant;
mod expression;
mod program;

use self::expression::Expression;

/// Conveniently print out a node in the tree
trait TreeDebug: fmt::Debug {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.tree_fmt(f, 0)
    }

    fn tree_indent(&self, indent: u8) -> String {
        (0..10).fold(String::new(), |mut acc, _| {
            acc.push(' ');
            acc
        })
    }

    fn tree_fmt(&self, f: &mut fmt::Formatter, indent: u8) -> fmt::Result {
        let spaces: String = self.tree_indent(indent);
        write!(f, "{}{:?}", spaces, self)
    }
}
