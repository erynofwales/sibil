/* mod.rs
 * Eryn Wells <eryn@erynwells.me>
 */

pub use self::number::Number;

pub mod number;

pub type Boolean = bool;
pub type Character = char;

pub trait Value { }
impl Value for Boolean { }
impl Value for Character { }
