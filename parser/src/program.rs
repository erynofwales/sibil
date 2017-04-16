/* parser/src/program.rs
 * Eryn Wells <eryn@erynwells.me>
 */

use sibillexer::Lexer;
use super::Result;
use super::Parsable;
use super::ParseError;
use list::SExpression;

struct Program {
    commands: Vec<SExpression>,
}

impl Parsable for Program {
    fn parse(lexer: &Lexer) -> Result<Program> {
        let mut commands: Vec<SExpression> = Vec::new();
        // TODO: Actually parse commands.
        Ok(Program { commands: commands })
    }
}
