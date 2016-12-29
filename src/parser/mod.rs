/* mod.rs
 * Eryn Wells <eryn@erynwells.me>
 */

pub use self::nodes::Program;

mod nodes;

use lexer::Lexer;

pub struct Parser {
    lexer: Lexer,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Parser {
        Parser { lexer: lexer }
    }

    pub fn parse(&self) -> nodes::Program {
        self.parse_program()
    }
}

impl Parser {
    fn parse_program(&self) -> nodes::Program {
        let program = Program::new();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lexer::Lexer;

    #[test]
    fn parses_empty_input() {
        let parser = Parser::new(Lexer::new(""));
        let parsed = parser.parse();
        assert_eq!(parsed, Program::new());
    }
}
