/* str.rs
 * Eryn Wells <eryn@erynwells.me>
 */

pub trait RelativeIndexable {
    /// Get the index of the character boundary preceding the given index. The index does not need to be on a character
    /// boundary.
    fn index_before(&self, usize) -> usize;

    /// Get the index of the character boundary following the given index. The index does not need to be on a character
    /// boundary.
    fn index_after(&self, usize) -> usize;
}

pub trait CharAt {
    /// Get the character at the given byte index. This index must be at a character boundary as defined by
    /// `is_char_boundary()`.
    fn char_at(&self, usize) -> Option<char>;
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

impl CharAt for str {
    fn char_at(&self, index: usize) -> Option<char> {
        if !self.is_char_boundary(index) {
            return None;
        }
        let end = self.index_after(index);
        let char_str = &self[index .. end];
        char_str.chars().nth(0)
    }
}

#[test]
fn index_before_is_well_behaved_for_ascii() {
    let s = "abc";

    // Sanity
    assert_eq!(s.index_before(0), 0);
    assert_eq!(s.index_before(2), 1);

    // An index beyond the string bounds returns the index of the last character in the string.
    {
        let idx = s.index_before(4);
        assert_eq!(idx, 2);
        assert!(s.is_char_boundary(idx));
        let last_char = &s[idx ..];
        assert_eq!(last_char.len(), 1);
        assert_eq!(last_char.chars().nth(0), Some('c'));
    }
}

#[test]
fn index_after_is_well_behaved_for_ascii() {
    let s = "abc";

    // Sanity
    assert_eq!(s.index_after(0), 1);
    assert_eq!(s.index_after(2), 3);

    // An index beyond the string bounds returns the length of the string
    {
        let idx = s.index_after(4);
        assert_eq!(idx, s.len());
        assert!(s.is_char_boundary(idx));
    }
}
