/* char.rs
 * Eryn Wells <eryn@erynwells.me>
 */

use lexer::charset;

pub trait Lexable {
    fn is_left_paren(&self) -> bool;
    fn is_right_paren(&self) -> bool;
    fn is_identifier_initial(&self) -> bool;
    fn is_identifier_subsequent(&self) -> bool;
    fn is_identifier_single(&self) -> bool;
    fn is_hash(&self) -> bool;
    fn is_boolean_true(&self) -> bool;
    fn is_boolean_false(&self) -> bool;
    fn is_newline(&self) -> bool;
    fn is_comment_initial(&self) -> bool;
}

impl Lexable for char {
    fn is_left_paren(&self) -> bool {
        *self == '('
    }

    fn is_right_paren(&self) -> bool {
        *self == ')'
    }

    fn is_identifier_initial(&self) -> bool {
        charset::identifier_initials().contains(&self)
    }

    fn is_identifier_subsequent(&self) -> bool {
        charset::identifier_subsequents().contains(&self)
    }

    fn is_identifier_single(&self) -> bool {
        charset::identifier_singles().contains(&self)
    }

    fn is_hash(&self) -> bool {
        *self == '#'
    }

    fn is_boolean_true(&self) -> bool {
        *self == 't'
    }

    fn is_boolean_false(&self) -> bool {
        *self == 'f'
    }

    fn is_newline(&self) -> bool {
        *self == '\n'
    }

    fn is_comment_initial(&self) -> bool {
        *self == ';'
    }
}

