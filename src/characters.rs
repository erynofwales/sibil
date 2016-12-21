//! Characters
//!
//! Utilities for dealing with chars of various sorts.

use std::collections::HashSet;
use std::iter::FromIterator;

pub type CharSet = HashSet<char>;

fn ascii_letters() -> CharSet {
    let letters = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars();
    CharSet::from_iter(letters)
}

fn ascii_digits() -> CharSet {
    let digits = "1234567890".chars();
    CharSet::from_iter(digits)
}

pub fn identifier_initials() -> CharSet {
    let letters = ascii_letters();
    let extras = CharSet::from_iter("!$%&*/:<=>?~_^".chars());
    let mut initials = CharSet::new();
    initials.extend(letters.iter());
    initials.extend(extras.iter());
    initials
}

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

pub trait RelativeIndexable {
    fn index_before(&self, &usize) -> usize;
    fn index_after(&self, &usize) -> usize;
}

impl RelativeIndexable for str {
    fn index_before(&self, index: &usize) -> usize {
        let mut prev_index = index - 1;
        if prev_index <= 0 {
            return 0;
        }
        while !self.is_char_boundary(prev_index) {
            prev_index -= 1;
        }
        prev_index
    }

    fn index_after(&self, index: &usize) -> usize {
        let mut next_index = index + 1;
        if next_index > self.len() {
            return self.len();
        }
        while !self.is_char_boundary(next_index) {
            next_index += 1;
        }
        next_index
    }
}
