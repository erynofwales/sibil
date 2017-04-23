/* lexer/src/named_char.rs
 * Eryn Wells <eryn@erynwells.me>
 */

use std::collections::HashMap;
use std::sync::{Once, ONCE_INIT};

const ALARM: &'static str = "alarm";
const BACKSPACE: &'static str = "backspace";
const DELETE: &'static str = "delete";
const ESCAPE: &'static str = "escape";
const NEWLINE: &'static str = "newline";
const NULL: &'static str = "null";
const RETURN: &'static str = "return";
const SPACE: &'static str = "space";
const TAB: &'static str = "tab";

/// Mapping of names to `char` values. Returns the name of the given character, if a name exists.
/// Otherwise, returns `None`.
fn char_for(named: &str) -> Option<char> {
    static ONCE = ONCE_INIT;
    static mut names_to_chars: HashMap<&'static str, char>;
    unsafe {
        ONCE.call_once(|| {
            names_to_chars = HashMap::new();
            names_to_chars.insert(ALARM, '\x07');
            names_to_chars.insert(BACKSPACE, '\x08');
            names_to_chars.insert(DELETE, '\x7F');
            names_to_chars.insert(ESCAPE, '\x1B');
            names_to_chars.insert(NEWLINE, '\n');
            names_to_chars.insert(NULL, '\0');
            names_to_chars.insert(RETURN, '\r');
            names_to_chars.insert(SPACE, ' ');
            names_to_chars.insert(TAB, '\t');
        });
        names_to_chars.get(named)
    }
}

/// Mapping of `char` values to names. Returns the name of the given character, if a name exists.
/// Otherwise, returns `None`.
fn name_of(c: char) -> Option<&'static str> {
    static ONCE = ONCE_INIT;
    static mut chars_to_names: HashMap<char, &'static str>;
    unsafe {
        ONCE.call_once(|| {
            chars_to_names = HashMap::new();
            chars_to_names.insert('\x07', ALARM);
            chars_to_names.insert('\x08', BACKSPACE);
            chars_to_names.insert('\x7F', DELETE);
            chars_to_names.insert('\x1B', ESCAPE);
            chars_to_names.insert('\n', NEWLINE);
            chars_to_names.insert('\0', NULL);
            chars_to_names.insert('\r', RETURN);
            chars_to_names.insert(' ', SPACE);
            chars_to_names.insert('\t', TAB);
        });
        chars_to_names.get(c)
    }
}
