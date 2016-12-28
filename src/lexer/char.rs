/* char.rs
 * Eryn Wells <eryn@erynwells.me>
 */

use lexer::charset;

pub trait Lexable {
    fn is_character_leader(&self) -> bool;
    fn is_dot(&self) -> bool;
    fn is_hash(&self) -> bool;
    fn is_quote(&self) -> bool;
    fn is_left_paren(&self) -> bool;
    fn is_right_paren(&self) -> bool;
    fn is_string_quote(&self) -> bool;
    fn is_string_escape_leader(&self) -> bool;
    fn is_string_escaped(&self) -> bool;
    fn is_newline(&self) -> bool;
    fn is_eof(&self) -> bool;

    fn is_identifier_initial(&self) -> bool;
    fn is_identifier_subsequent(&self) -> bool;
    fn is_identifier_delimiter(&self) -> bool;

    fn is_boolean_true(&self) -> bool;
    fn is_boolean_false(&self) -> bool;

    fn is_comment_initial(&self) -> bool;
}

impl Lexable for char {
    fn is_left_paren(&self) -> bool {
        *self == '('
    }

    fn is_right_paren(&self) -> bool {
        *self == ')'
    }

    fn is_character_leader(&self) -> bool {
        *self == '\\'
    }

    fn is_dot(&self) -> bool {
        *self == '.'
    }

    fn is_hash(&self) -> bool {
        *self == '#'
    }

    fn is_quote(&self) -> bool {
        *self == '\''
    }

    fn is_string_quote(&self) -> bool {
        *self == '"'
    }

    fn is_string_escape_leader(&self) -> bool {
        *self == '\\'
    }

    fn is_string_escaped(&self) -> bool {
        *self == '"' || *self == '\\'
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

    fn is_eof(&self) -> bool {
        *self == '\0'
    }

    fn is_comment_initial(&self) -> bool {
        *self == ';'
    }

    fn is_identifier_initial(&self) -> bool {
        charset::identifier_initials().contains(&self)
    }

    fn is_identifier_subsequent(&self) -> bool {
        charset::identifier_subsequents().contains(&self)
    }

    fn is_identifier_delimiter(&self) -> bool {
        self.is_whitespace()
            || self.is_comment_initial()
            || self.is_left_paren()
            || self.is_right_paren()
            || self.is_string_quote()
            || self.is_eof()
    }
}
