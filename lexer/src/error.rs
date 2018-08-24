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

    pub fn msg(&self) -> &str { &self.message }
}
