/* types/tests/display.rs
 * Eryn Wells <eryn@erynwells.me>
 */

//! Tests of how various types implement Display.

extern crate sibiltypes;

use sibiltypes::{Obj, Pair, Sym};


#[test]
fn empty_pairs_display_correctly() {
    let empty = Pair::empty();
    let disp = format!("{}", empty);
    assert_eq!(disp, "(())");
}

#[test]
fn pair_with_only_car_is_single_element_list() {
    let sym = Obj::new(Sym::with_str("ab"));
    let pair = Pair::with_car(sym);
    let disp = format!("{}", pair);
    assert_eq!(disp, "(ab)");
}

#[test]
fn pair_with_car_and_cdr_is_dotted_pair() {
    let ab = Obj::new(Sym::with_str("ab"));
    let cd = Obj::new(Sym::with_str("cd"));
    let pair = Pair::new(ab, cd);
    let disp = format!("{}", pair);
    assert_eq!(disp, "(ab . cd)");
}

#[test]
fn two_pair_list_displays_as_list() {
    let ab = Obj::new(Sym::with_str("ab"));
    let cd = Obj::new(Sym::with_str("cd"));
    let pair = Pair::new(ab, Obj::new(Pair::with_car(cd)));
    let disp = format!("{}", pair);
    assert_eq!(disp, "(ab cd)");
}

#[test]
fn three_element_list_with_full_second_pair() {
    let ab = Obj::new(Sym::with_str("ab"));
    let cd = Obj::new(Sym::with_str("cd"));
    let ef = Obj::new(Sym::with_str("ef"));
    let pair = Pair::new(ab, Obj::new(Pair::new(cd, ef)));
    let disp = format!("{}", pair);
    assert_eq!(disp, "(ab cd . ef)");
}

#[test]
fn syms_display_as_strings() {
    let sym = Sym::with_str("abc");
    let disp = format!("{}", sym);
    assert_eq!(disp, "abc");
}

