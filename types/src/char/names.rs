/* types/src/char/names.rs
 * Eryn Wells <eryn@erynwells.me>
 */

use std::collections::HashMap;
use std::mem;
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
pub fn char_for(name: &str) -> Option<char> {
    type NameMap = HashMap<&'static str, char>;
    static ONCE: Once = ONCE_INIT;
    static mut NAMES_TO_CHARS: *const NameMap = 0 as *const NameMap;
    unsafe {
        ONCE.call_once(|| {
            let mut map = NameMap::new();
            map.insert(ALARM, '\x07');
            map.insert(BACKSPACE, '\x08');
            map.insert(DELETE, '\x7F');
            map.insert(ESCAPE, '\x1B');
            map.insert(NEWLINE, '\n');
            map.insert(NULL, '\0');
            map.insert(RETURN, '\r');
            map.insert(SPACE, ' ');
            map.insert(TAB, '\t');
            NAMES_TO_CHARS = mem::transmute(Box::new(map));
        });
        (*NAMES_TO_CHARS).get(name).map(|c| *c)
    }
}

/// Mapping of `char` values to names. Returns the name of the given character, if a name exists.
/// Otherwise, returns `None`.
pub fn name_of(c: char) -> Option<&'static str> {
    type CharMap = HashMap<char, &'static str>;
    static ONCE: Once = ONCE_INIT;
    static mut CHARS_TO_NAMES: *const CharMap = 0 as *const CharMap;
    unsafe {
        ONCE.call_once(|| {
            let mut map = CharMap::new();
            map.insert('\x07', ALARM);
            map.insert('\x08', BACKSPACE);
            map.insert('\x7F', DELETE);
            map.insert('\x1B', ESCAPE);
            map.insert('\n', NEWLINE);
            map.insert('\0', NULL);
            map.insert('\r', RETURN);
            map.insert(' ', SPACE);
            map.insert('\t', TAB);
            CHARS_TO_NAMES = mem::transmute(Box::new(map));
        });
        (*CHARS_TO_NAMES).get(&c).map(|s| *s)
    }
}
