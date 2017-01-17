/* parser/mod.rs
 * Eryn Wells <eryn@erynwells.me>
 */

mod nodes;

use std::fmt;

use lexer::lex;
use lexer::Lex;
use lexer::Lexer;
use lexer::Token;
use self::nodes::Program;
use self::nodes::Expression;

type ParseResult = Result<Program, Error>;

pub fn parse(input: &str) -> ParseResult {
    let mut parser = Parser::new(lex(input));
    parser.parse()
}

pub struct Parser {
    lexer: Lexer,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Parser {
        Parser { lexer: lexer }
    }

    pub fn parse(&mut self) -> ParseResult {
        self.parse_program()
    }
}

impl Parser {
    fn parse_program(&mut self) -> ParseResult {
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
                Err(error) => return Err(error),
            }
        }
        Ok(Program::new(forms))
    }

    fn parse_expression(&mut self) -> Result<Expression, Error> {
        if let Some(next) = self.lexer.next() {
            match next.token {
                Token::Boolean(value) => Ok(Expression::Literal(Box::new(value))),
                Token::Character(value) => Ok(Expression::Literal(Box::new(value))),
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

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PARSE ERROR: {}\n  token = {:?}", self.desc, self.lex)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::nodes::*;
    use lexer::Lexer;
    use lexer::Token;
    use types::Boolean;

    #[test]
    fn parses_empty_input() {
        let r = parse("");
        assert_eq!(r.unwrap(), Program::new(vec![Expression::EOF]));
    }

    #[test]
    fn parses_single_boolean() {
        let r = parse("#t");
        assert_eq!(r.unwrap(), Program::new(vec![Expression::Literal(Box::new(Boolean::new(true))), Expression::EOF]));
    }
    
    #[test]
    fn parses_single_expression() {
        let r = parse("(a)");
        let list = list("(", vec![Box::new(Expression::Id("a".to_string()))], ")");
        assert_eq!(r.unwrap(), Program::new(vec![list, Expression::EOF]));
    }

    fn list(left: &str, expr: Vec<Box<Expression>>, right: &str) -> Expression {
        Expression::List {
            left: Token::LeftParen(left.to_string()),
            expr: expr,
            right: Token::RightParen(right.to_string())
        }
    }
}
