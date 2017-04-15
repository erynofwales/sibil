/* lexer/src/named_char.rs
 * Eryn Wells <eryn@erynwells.me>
 */

use std::collections::HashSet;
use sibiltypes::Char;

const ALARM: &'static str = "alarm";
const BACKSPACE: &'static str = "backspace";
const DELETE: &'static str = "delete";
const ESCAPE: &'static str = "escape";
const NEWLINE: &'static str = "newline";
const NULL: &'static str = "null";
const RETURN: &'static str = "return";
const SPACE: &'static str = "space";
const TAB: &'static str = "tab";

pub fn set() -> HashSet<&'static str> {
    let mut set: HashSet<&'static str> = HashSet::new();
    set.insert(ALARM);
    set.insert(BACKSPACE);
    set.insert(DELETE);
    set.insert(ESCAPE);
    set.insert(NEWLINE);
    set.insert(NULL);
    set.insert(RETURN);
    set.insert(SPACE);
    set.insert(TAB);
    set
}

pub fn char_named_by(named: &str) -> Char {
    Char(match named {
        ALARM => '\x07',
        BACKSPACE => '\x08',
        DELETE => '\x7F',
        ESCAPE => '\x1B',
        NEWLINE => '\n',
        NULL => '\0',
        RETURN => '\r',
        SPACE => ' ',
        TAB => '\t',
        _ => panic!("char_named_by called with invalid named char string")
    })
}
