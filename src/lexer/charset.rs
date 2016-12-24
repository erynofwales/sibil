/* charset.rs
 * Eryn Wells <eryn@erynwells.me>
 */

use std::collections::HashSet;
use std::iter::FromIterator;

pub type CharSet = HashSet<char>;

// TODO: Use std::sync::Once for these sets?
// https://doc.rust-lang.org/beta/std/sync/struct.Once.html

fn ascii_letters() -> CharSet {
    let letters = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars();
    CharSet::from_iter(letters)
}

fn ascii_digits() -> CharSet {
    let digits = "1234567890".chars();
    CharSet::from_iter(digits)
}

/// A set of all characters allowed to start Scheme identifiers.
pub fn identifier_initials() -> CharSet {
    let letters = ascii_letters();
    let extras = CharSet::from_iter("!$%&*/:<=>?~_^".chars());
    let mut initials = CharSet::new();
    initials.extend(letters.iter());
    initials.extend(extras.iter());
    initials
}

/// A set of all characters allowed to follow an identifier initial.
pub fn identifier_subsequents() -> CharSet {
    let initials = identifier_initials();
    let digits = ascii_digits();
    let extras = CharSet::from_iter(".+-".chars());
    let mut subsequents = CharSet::new();
    subsequents.extend(initials.iter());
    subsequents.extend(digits.iter());
    subsequents.extend(extras.iter());
    subsequents
}
