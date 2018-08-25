/* types/tests/equality_value.rs
 * Eryn Wells <eryn@erynwells.me>
 */

//! Tests of value-based equality for various types herein.

extern crate sibiltypes;

use sibiltypes::{Obj, Pair, Sym};

#[test]
fn pairs_with_one_similar_symbol_are_equal() {
    let pa = Pair::with_car(Obj::new(Sym::with_str("ab")));
    let pb = Pair::with_car(Obj::new(Sym::with_str("ab")));
    assert_eq!(pa, pb);
}
