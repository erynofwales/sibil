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
        self.parse_program()
    }
}

impl Parser {
    fn parse_program(&mut self) -> Result<Program, Error> {
        let mut forms: Vec<Expression> = Vec::new();
        loop {
            match self.parse_expression() {
                Ok(expr) => {
                    let is_eof = expr == Expression::EOF;
                    forms.push(expr);
                    if is_eof {
                        break;
                    }
                },
                Err(error) => panic!("PARSE ERROR: error = {}, lex = {:?}", error.desc, error.lex)
            }
        }
        Ok(Program::new(forms))
    }

    fn parse_expression(&mut self) -> Result<Expression, Error> {
        if let Some(next) = self.lexer.next() {
            match next.token {
                Token::Boolean(value) => Ok(Expression::Atom(Box::new(value))),
                Token::Character(value) => Ok(Expression::Atom(Box::new(value))),
                _ => Err(Error { lex: next, desc: "Invalid token".to_string() })
            }
        }
        else {
            Ok(Expression::EOF)
        }
    }
}

pub struct Error {
    lex: Lex,
    desc: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::nodes::*;
    use lexer::Lexer;
    use types::Boolean;

    #[test]
    fn parses_empty_input() {
        let mut parser = Parser::new(Lexer::new(""));
        assert_eq!(parser.parse().ok().unwrap(), Program::new(vec![Expression::EOF]));
    }

    #[test]
    fn parses_single_boolean() {
        let mut parser = Parser::new(Lexer::new("#t"));
        assert_eq!(parser.parse().ok().unwrap(), Program::new(vec![Expression::Atom(Box::new(Boolean::new(true))), Expression::EOF]));
    }
}
