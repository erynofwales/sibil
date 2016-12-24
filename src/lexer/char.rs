/* char.rs
 * Eryn Wells <eryn@erynwells.me>
 */

use lexer::charset;

pub trait Lexable {
    fn is_left_paren(&self) -> bool;
    fn is_right_paren(&self) -> bool;
    fn is_identifier_initial(&self) -> bool;
    fn is_identifier_subsequent(&self) -> bool;
}

impl Lexable for char {
    fn is_left_paren(&self) -> bool {
        self == &'('
    }

    fn is_right_paren(&self) -> bool {
        self == &')'
    }

    fn is_identifier_initial(&self) -> bool {
        charset::identifier_initials().contains(&self)
    }

    fn is_identifier_subsequent(&self) -> bool {
        charset::identifier_subsequents().contains(&self)
    }
}
