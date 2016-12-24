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
    fn index_before(&self, &usize) -> Option<usize>;
    fn index_after(&self, &usize) -> Option<usize>;
}

impl RelativeIndexable for str {
    fn index_before(&self, index: &usize) -> Option<usize> {
        let mut prev_index = index - 1;
        if prev_index <= 0 {
            return None;
        }
        while !self.is_char_boundary(prev_index) {
            prev_index -= 1;
        }
        Some(prev_index)
    }

    fn index_after(&self, index: &usize) -> Option<usize> {
        let mut next_index = index + 1;
        if next_index >= self.len() {
            return None;
        }
        while !self.is_char_boundary(next_index) {
            next_index += 1;
        }
        Some(next_index)
    }
}
