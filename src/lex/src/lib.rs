pub mod tokens;

use tokens::{Token, TokenType};

#[derive(Debug, Clone)]
struct Scanner<'a> {
    pub source: &'a str,
    pub token_list: Vec<Token<'a>>,
    /// first character of lexeme being scanned
    start: usize,
    /// current character being scanned
    current: usize,
    /// line being scanned
    line: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(src: &str) -> Scanner {
        Scanner {
            source: src,
            token_list: Vec::new(),
            start: 0,
            line: 0,
            current: 0,
        }
    }

    pub fn scan(mut self) {
        let mut current = self.current;
        while !&self.clone().is_ended() {
            let s = self.clone();
            current = s.current;

            self.start = current;

            self.scan_token();
        }

        self.clone().token_list.push(Token {
            token_type: tokens::TokenType::EOI,
            line: 0,
            lexeme: String::new(),
            literal: "",
        });
    }

    fn is_ended(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(self) {
        let character = self.source.chars().nth(self.current + 1).unwrap();
        let token_type = match character {
            '(' => Some(TokenType::LParen),
            _ => None,
        };

        match token_type {
            Some(token_type) => self.push_token(token_type),
            None => (),
        };
    }

    fn push_token(mut self, token: TokenType) {
        let token_text_string = String::from(self.source);
        let token_text = &token_text_string[self.start..self.current];

        self.token_list.push(Token {
            token_type: token,
            line: self.line,
            lexeme: token_text.to_string(),
            literal: "",
        })
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
