/* lexer/src/error.rs
 * Eryn Wells <eryn@erynwells.me>
 */

#[derive(Debug, Eq, PartialEq)]
pub struct Error {
    message: String
}

impl Error {
    pub fn new(msg: String) -> Error {
        Error {
            message: msg
        }
    }

    pub fn invalid_char(c: char) -> Error {
        Error::new(format!("invalid character: {}", c))
    }

    pub fn unexpected_eof() -> Error {
        Error::new("unexpected EOF".to_string())
    }

    pub fn msg(&self) -> &str { &self.message }
}
