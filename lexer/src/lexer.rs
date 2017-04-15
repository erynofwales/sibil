/* lexer.rs
 * Eryn Wells <eryn@erynwells.me>
 */

use std::collections::HashSet;
use sibiltypes::{Bool, Char};

use char::Lexable;
use named_char;
use number::Exactness;
use number::NumberBuilder;
use number::Radix;
use number::Sign;
use str::CharAt;
use str::RelativeIndexable;
use token::Lex;
use token::Token;

type StateResult = Result<Option<Token>, String>;

trait HasResult {
    fn has_token(&self) -> bool;
}

#[derive(Debug)]
enum State {
    Char,
    NamedChar(HashSet<&'static str>, String),
    Comment,
    Initial,
    Id,
    Dot,
    Hash,
    Number,
    NumberExactness,
    NumberDecimal,
    NumberRadix,
    NumberSign,
    Sign,
    String,
    StringEscape,
}

pub fn lex(input: &str) -> Lexer {
    Lexer::new(&input)
}

pub struct Lexer {
    input: String,
    begin: usize,
    forward: usize,
    line: usize,
    line_offset: usize,
    state: State,
    number_builder: NumberBuilder,
    string_value: String,
}

impl Lexer {
    pub fn new(input: &str) -> Lexer {
        Lexer {
            input: String::from(input),
            begin: 0,
            forward: 0,
            line: 1,
            line_offset: 1,
            state: State::Initial,
            number_builder: NumberBuilder::new(),
            string_value: String::new(),
        }
    }
}

impl Lexer {
    fn begin_lexing(&mut self) {
        self.forward = self.begin;
        self.state = State::Initial;
    }

    /// Advance the forward pointer to the next character.
    fn advance(&mut self) {
        self.forward = self.input.index_after(self.forward);
        self.line_offset += 1;
        println!("> forward={}", self.forward);
    }

    /// Retract the forward pointer to the previous character.
    fn retract(&mut self) {
        self.forward = self.input.index_before(self.forward);
        self.line_offset -= 1;
        println!("< forward={}", self.forward);
    }

    /// Advance the begin pointer to prepare for the next iteration.
    fn advance_begin(&mut self) {
        self.begin = self.input.index_after(self.forward);
        self.forward = self.begin;
        println!("> begin={}, forward={}", self.begin, self.forward);
    }

    /// Update lexer state when it encounters a newline.
    fn handle_newline(&mut self) {
        self.line += 1;
        self.line_offset = 1;
    }

    /// Get the substring between the two input indexes. This is the value to give to a new Token instance.
    fn value(&self) -> String {
        self.input[self.begin .. self.forward].to_string()
    }

    fn error_string(&self, message: String) -> String {
        format!("{}:{}: {}", self.line, self.line_offset, message)
    }

    fn token_result(&self, token: Token) -> StateResult {
        Ok(Some(token))
    }

    fn generic_error(&self, c: char) -> StateResult {
        Err(self.error_string(format!("Invalid token character: {}", c)))
    }
}

impl Lexer {
    /// Handle self.state == State::Initial
    fn state_initial(&mut self, c: char) -> StateResult {
        if c.is_left_paren() {
            return self.token_result(Token::LeftParen);
        }
        else if c.is_right_paren() {
            return self.token_result(Token::RightParen);
        }
        else if c.is_dot() {
            self.state = State::Dot;
            self.advance();
        }
        else if c.is_hash() {
            self.state = State::Hash;
            self.advance();
        }
        else if c.is_quote() {
            return self.token_result(Token::Quote);
        }
        else if c.is_string_quote() {
            self.string_value = String::from("");
            self.state = State::String;
            self.advance();
        }

        else if let Some(sign) = Sign::from_char(c) {
            self.number_builder = NumberBuilder::new();
            self.number_builder.sign(sign);
            self.state = State::Sign;
            self.advance();
        }
        else if c.is_identifier_initial() {
            self.state = State::Id;
            self.advance();
        }

        else if c.is_digit(10) {
            self.number_builder = NumberBuilder::new();
            self.number_builder.extend_value(c);
            self.state = State::Number;
            self.advance();
        }

        else if c.is_whitespace() {
            if c.is_newline() {
                self.handle_newline();
            }
            self.advance_begin();
        }

        else if c.is_comment_initial() {
            self.state = State::Comment;
            self.advance();
        }

        else {
            return self.generic_error(c);
        }

        Ok(None)
    }

    /// Handle self.state == State::Id
    fn state_identifier(&mut self, c: char) -> StateResult {
        if c.is_identifier_subsequent() {
            // Stay in Id state.
            self.advance();
        }
        else if c.is_identifier_delimiter() {
            let value = self.value();
            self.retract();
            return self.token_result(Token::Id(value));
        }
        else {
            return self.generic_error(c);
        }
        Ok(None)
    }

    /// Handle self.state == State::Char
    fn state_char(&mut self, c: char) -> StateResult {
        self.advance();
        let lower_c = c.to_lowercase().collect::<String>();
        let mut candidates: HashSet<&str> = HashSet::new();
        for c in named_char::set().iter() {
            if c.starts_with(&lower_c) {
                candidates.insert(c);
            }
        }
        if candidates.len() > 0 {
            self.state = State::NamedChar(candidates, lower_c);
        } else {
            return self.token_result(Token::Character(Char(c)));
        }
        Ok(None)
    }

    /// Handle self.state == State::NamedChar
    fn state_named_char(&mut self, c: char) -> StateResult {
        let (candidates, mut progress) = match self.state {
            State::NamedChar(ref candidates, ref progress) => (candidates.clone(), progress.clone()),
            _ => panic!("Called state_named_char without being in NamedChar state")
        };

        if c.is_identifier_delimiter() || c.is_eof() {
            if progress.len() == 1 {
                self.retract();
                return self.token_result(Token::Character(Char(progress.chars().next().unwrap())));
            }
            else {
                return self.generic_error(c);
            }
        }

        progress.push(c);

        let candidates: HashSet<&str> = {
            let filtered = candidates.iter().filter(|c| c.starts_with(&progress)).map(|c| *c);
            filtered.collect()
        };

        if candidates.len() == 1 {
            let candidate = *candidates.iter().next().unwrap();
            if candidate == &progress {
                self.token_result(Token::Character(named_char::char_named_by(&progress)))
            }
            else {
                self.state = State::NamedChar(candidates, progress);
                self.advance();
                Ok(None)
            }
        }
        else if candidates.len() > 1 {
            self.state = State::NamedChar(candidates, progress);
            self.advance();
            Ok(None)
        }
        else {
            self.generic_error(c)
        }
    }

    /// Handle self.state == State::Dot
    fn state_dot(&mut self, c: char) -> StateResult {
        if c.is_identifier_delimiter() {
            self.retract();
            return self.token_result(Token::Dot);
        }
        else if c.is_digit(10) {
            self.number_builder = NumberBuilder::new();
            self.number_builder.extend_decimal_value(c);
            self.state = State::NumberDecimal;
            self.advance();
        }
        else {
            return self.generic_error(c);
        }
        Ok(None)
    }

    /// Handle self.state == State::Hash
    fn state_hash(&mut self, c: char) -> StateResult {
        if c.is_boolean_true() || c.is_boolean_false() {
            self.advance();
            return self.token_result(Token::Boolean(Bool(c.is_boolean_true())));
        }
        else if c.is_left_paren() {
            self.advance();
            return self.token_result(Token::LeftVectorParen);
        }
        else if c.is_character_leader() {
            self.state = State::Char;
            self.advance();
        }
        else if let Some(radix) = Radix::from_char(c) {
            self.number_builder.radix(radix);
            self.state = State::NumberRadix;
            self.advance();
        }
        else if let Some(exactness) = Exactness::from_char(c) {
            self.number_builder.exact(exactness);
            self.state = State::NumberExactness;
            self.advance();
        }
        else {
            return self.generic_error(c);
        }
        Ok(None)
    }

    /// Handle self.state == State::Number
    fn state_number(&mut self, c: char) -> StateResult {
        if c.is_digit(self.number_builder.radix_value()) {
            self.number_builder.extend_value(c);
            self.advance();
        }
        else if c.is_dot() {
            self.state = State::NumberDecimal;
            self.advance();
        }
        else if c.is_identifier_delimiter() {
            self.retract();
            return self.token_result(Token::Number(self.number_builder.resolve()));
        }
        else {
            return self.generic_error(c);
        }
        Ok(None)
    }

    fn state_number_exactness(&mut self, c: char) -> StateResult {
        if c.is_hash() {
            self.state = State::Hash;
            self.advance();
        }
        else if let Some(sign) = Sign::from_char(c) {
            self.number_builder.sign(sign);
            self.state = State::NumberSign;
            self.advance();
        }
        else if c.is_digit(self.number_builder.radix_value()) {
            self.number_builder.extend_value(c);
            self.state = State::Number;
            self.advance();
        }
        else {
            return self.generic_error(c);
        }
        Ok(None)
    }

    fn state_number_decimal(&mut self, c: char) -> StateResult {
        if c.is_digit(Radix::Dec.value()) {
            self.number_builder.extend_decimal_value(c);
            self.advance();
        }
        else if c.is_identifier_delimiter() {
            self.retract();
            return self.token_result(Token::Number(self.number_builder.resolve()));
        }
        else {
            return self.generic_error(c);
        }
        Ok(None)
    }

    fn state_number_radix(&mut self, c: char) -> StateResult {
        if c.is_digit(self.number_builder.radix_value()) {
            self.number_builder.extend_value(c);
            self.state = State::Number;
            self.advance();
        }
        else if c.is_dot() {
            self.state = State::NumberDecimal;
            self.advance();
        }
        else if c.is_hash() {
            self.state = State::Hash;
            self.advance();
        }
        else if let Some(sign) = Sign::from_char(c) {
            self.number_builder.sign(sign);
            self.state = State::NumberSign;
            self.advance();
        }
        else {
            return self.generic_error(c);
        }
        Ok(None)
    }

    fn state_number_sign(&mut self, c: char) -> StateResult {
        if c.is_digit(self.number_builder.radix_value()) {
            self.number_builder.extend_value(c);
            self.state = State::Number;
            self.advance();
        }
        else if c.is_dot() {
            self.state = State::NumberDecimal;
            self.advance();
        }
        else {
            return self.generic_error(c);
        }
        Ok(None)
    }

    fn state_sign(&mut self, c: char) -> StateResult {
        if c.is_digit(Radix::Dec.value()) {
            self.number_builder.extend_value(c);
            self.state = State::Number;
            self.advance();
        }
        else if c.is_identifier_delimiter() {
            let value = self.value();
            self.retract();
            return self.token_result(Token::Id(value));
        }
        else {
            return self.generic_error(c);
        }
        Ok(None)
    }

    fn state_string(&mut self, c: char) -> StateResult {
        self.advance();
        if c.is_string_quote() {
            return self.token_result(Token::String(self.string_value.clone()));
        }
        else if c.is_string_escape_leader() {
            self.state = State::StringEscape;
        }
        else {
            self.string_value.push(c);
        }
        Ok(None)
    }

    fn state_string_escape(&mut self, c: char) -> StateResult {
        let char_to_push = match c {
            '0' => '\0',
            'n' => '\n',
            't' => '\t',
            '"' => '"',
            '\\' => '\\',
            _ => return Err(self.error_string(format!("Invalid string escape character: {}", c))),
        };
        self.string_value.push(char_to_push);
        self.state = State::String;
        self.advance();
        Ok(None)
    }

    fn state_comment(&mut self, c: char) -> StateResult {
        if c.is_newline() {
            self.handle_newline();
            return self.token_result(Token::Comment(self.value()));
        }
        else if c.is_eof() {
            return self.token_result(Token::Comment(self.value()));
        }
        self.advance();
        Ok(None)
    }
}

impl Iterator for Lexer {
    type Item = Lex;

    fn next(&mut self) -> Option<Lex> {
        self.begin_lexing();
        if self.begin == self.input.len() {
            return None;
        }
        let mut token: Option<Token> = None;
        println!("Lexing '{}'", &self.input[self.begin ..]);
        while token.is_none() {
            let c = match self.input.char_at(self.forward) {
                Some(c) => c,
                None => '\0',
            };
            println!("{:?}! c='{}'", self.state, c);
            let previous_forward = self.forward;
            let result = match self.state {
                State::Char=> self.state_char(c),
                State::NamedChar(_, _) => self.state_named_char(c),
                State::Comment => self.state_comment(c),
                State::Dot => self.state_dot(c),
                State::Hash => self.state_hash(c),
                State::Id => self.state_identifier(c),
                State::Initial => self.state_initial(c),
                State::Number => self.state_number(c),
                State::NumberDecimal => self.state_number_decimal(c),
                State::NumberExactness => self.state_number_exactness(c),
                State::NumberRadix => self.state_number_radix(c),
                State::NumberSign => self.state_number_sign(c),
                State::Sign => self.state_sign(c),
                State::String => self.state_string(c),
                State::StringEscape => self.state_string_escape(c),
            };
            debug_assert!(result.has_token() || self.forward != previous_forward, "No lexing progress made!");
            if result.has_token() {
                token = result.ok().unwrap();
            }
            else if result.is_err() {
                assert!(false, "{}", result.err().unwrap());
            }
        }
        self.advance_begin();
        match token {
            Some(t) => Some(Lex::new(t, self.line, self.line_offset)),
            None => None,
        }
    }
}

impl HasResult for StateResult {
    fn has_token(&self) -> bool {
        match *self {
            Ok(ref token) => match *token {
                Some(_) => true,
                None => false,
            },
            Err(_) => false
        }
    }
}
