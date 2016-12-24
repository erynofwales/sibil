//! Characters
//!
//! Utilities for dealing with chars of various sorts.

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

//
// char
//

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
        identifier_initials().contains(&self)
    }

    fn is_identifier_subsequent(&self) -> bool {
        identifier_subsequents().contains(&self)
    }
}

//
// str and String
//

pub trait RelativeIndexable {
    /// Get the index of the character boundary preceding the given index. The index does not need to be on a character
    /// boundary.
    fn index_before(&self, usize) -> usize;

    /// Get the index of the character boundary following the given index. The index does not need to be on a character
    /// boundary.
    fn index_after(&self, usize) -> usize;
}

impl RelativeIndexable for str {
    fn index_before(&self, index: usize) -> usize {
        if index == 0 {
            return 0;
        }
        let mut index = index;
        if index > self.len() {
            index = self.len();
        }
        loop {
            index -= 1;
            if self.is_char_boundary(index) {
                break;
            }
        }
        index
    }

    fn index_after(&self, index: usize) -> usize {
        if index >= self.len() {
            return self.len();
        }
        let mut index = index;
        loop {
            index += 1;
            if self.is_char_boundary(index) {
                break;
            }
        }
        index
    }
}
