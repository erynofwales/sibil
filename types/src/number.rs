/* number.rs
 * Eryn Wells <eryn@erynwells.me>
 */

/// # Numbers
///
/// Scheme numbers are complex, literally.

type Int = i64;
type Flt = f64;

trait Number {
    fn is_number(&self) -> bool { true }
    fn is_complex(&self) -> bool { false }
    fn is_real(&self) -> bool { false }
    fn is_rational(&self) -> bool { false }
    fn is_integer(&self) -> bool { false }
}

struct Integer(Int);
struct Rational(Int, Int);
struct Real(Flt);
struct Complex<'a>(&'a Number, &'a Number);

