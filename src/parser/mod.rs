/* parser/mod.rs
 * Eryn Wells <eryn@erynwells.me>
 */

mod nodes;

use lexer::Lex;
use lexer::Lexer;
use lexer::Token;
use self::nodes::Program;
use self::nodes::Expression;

pub struct Parser {
    lexer: Lexer,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Parser {
        Parser { lexer: lexer }
    }

    pub fn parse(&mut self) -> Result<Program, Error> {
        Ok(self.parse_program())
    }
}

impl Parser {
    fn parse_program(&mut self) -> Program {
        let mut forms: Vec<Expression> = Vec::new();
        while let Some(lex) = self.lexer.next() {
            let form = match lex.token {
                _ => {
                    println!("{:?}", lex.token);
                    lex.token
                },
            };
        }
        forms.push(Expression::EOF);
        Program::new(forms)
    }
}

pub struct Error {
    lex: Lexer,
    desc: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::nodes::*;
    use lexer::Lexer;

    #[test]
    fn parses_empty_input() {
        let mut parser = Parser::new(Lexer::new(""));
        assert_eq!(parser.parse().ok().unwrap(), Program::new(vec![Expression::EOF]));
    }
}
