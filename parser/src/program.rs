/* parser/src/program.rs
 * Eryn Wells <eryn@erynwells.me>
 */

use list::SExpression;

struct Program {
    commands: Vec<SExpression>,
}
