/* mod.rs
 * Eryn Wells <eryn@erynwells.me>
 */

use std::fmt;

pub use self::number::Number;

pub mod number;

pub type Boolean = bool;
pub type Character = char;

pub trait Value: fmt::Debug { }
impl Value for Boolean { }
impl Value for Character { }
