/* parser/nodes/mod.rs
 * Eryn Wells <eryn@erynwells.me>
 */

use std::fmt;

pub use self::constant::Constant;

mod constant;
mod program;

trait TreeDebug: fmt::Debug {
    fn tree_indent(indent: u8) -> String {
        (0..10).fold(String::new(), |mut acc, i| {
            acc.push(' ');
            acc
        })
    }

    fn tree_fmt(&self, f: &mut fmt::Formatter, indent: u8) -> fmt::Result {
        write!(f, "{}{:?}", self.tree_indent(indent), self)
    }
}

