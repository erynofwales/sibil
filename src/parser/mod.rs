/* parser/mod.rs
 * Eryn Wells <eryn@erynwells.me>
 */

mod nodes;

use lexer::Lexer;
use self::nodes::Program;

pub struct Parser {
    lexer: Lexer,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Parser {
        Parser { lexer: lexer }
    }

    pub fn parse(&mut self) -> Program {
        Program { }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::nodes::*;
    use lexer::Lexer;

    #[test]
    fn parses_empty_input() {
        let mut parser = Parser::new(Lexer::new(""));
        assert_eq!(parser.parse(), Program::new());
    }
}
