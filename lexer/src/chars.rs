/* lexer/src/chars.rs
 * Eryn Wells <eryn@erynwells.me>
 */

pub trait Lexable {
    fn is_left_paren(&self) -> bool;
    fn is_right_paren(&self) -> bool;
}

impl Lexable for char {
    fn is_left_paren(&self) -> bool { *self == '(' }
    fn is_right_paren(&self) -> bool { *self == ')' }
}
