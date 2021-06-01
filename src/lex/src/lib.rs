pub mod error;
use std::collections::HashMap;

use error::LexError;

pub mod tokens;
pub use tokens::{Token, TokenType};

#[derive(Debug)]
pub struct Lexer<'a> {
    source: &'a str,
    start: usize,
    index: usize,
    line: usize,
}

impl<'a> Lexer<'a> {
    fn new(source: &str) -> Lexer {
        Lexer {
            source,
            start: 0,
            index: 0,
            line: 0,
        }
    }

    // returns current character without advancing lexer
    fn peek(&self) -> Option<char> {
        self.source.chars().nth(self.index)
    }

    // returns the nth character after index without advancing lexer
    fn peek_nth(&self, idx: usize) -> Option<char> {
        self.source.chars().nth(self.index + idx)
    }

    // returns next character and advances lexer by one
    fn next(&mut self) -> Option<char> {
        if !self.is_ended() {
            self.index += 1;
        }

        self.peek()
    }

    fn next_nth(&mut self, idx: usize) -> Option<char> {
        if !self.is_ended() {
            self.index += idx;
        }

        self.peek_nth(idx)
    }

    fn is_ended(&self) -> bool {
        self.index > self.source.len()
    }

    /// creates a span and updates lexer placing
    fn span(&mut self) -> (usize, usize) {
        let s = (self.start, self.index);
        self.start = self.index;

        s
    }

    fn advance_line(&mut self) {
        self.start = 0;
        self.index = 0;
        self.line += 1;
    }

    fn create_token(&mut self, token_type: TokenType) -> Token<'a> {
        Token {
            token_type,
            span: self.span(),
            source: self.source,
        }
    }

    fn string(&mut self, starting_char: char) -> Result<String, LexError> {
        while self.peek().unwrap() != starting_char && self.is_ended() == false {
            if let Some(c) = self.peek() {
                if c == '\n' {
                    self.advance_line()
                }
            };

            self.next();
        }

        if self.is_ended() == true {
            return Err(LexError::UnterminatedString);
        }

        // closing starting_char
        self.next();

        // slices between the quotes/backtics
        let value = &self.source[self.start + 1..self.index - 1];

        Ok(value.to_string())
    }

    fn number(&mut self) -> Result<f64, LexError> {
        // scans until end of numbers, only advances lexer if there's a number
        while let Some(v) = self.peek() {
            if v.is_numeric() {
                self.next();
            } else {
                break;
            }
        }

        // consumes decimal point
        if let Some(p) = self.peek() {
            if p == '.' {
                // ensures that there are numbers after decimal point, consumes if there are
                if let Some(v) = self.peek_nth(1) {
                    if v.is_numeric() {
                        self.next();
                    }
                }
            }

            while let Some(v) = self.peek() {
                if v.is_numeric() {
                    self.next();
                } else {
                    break;
                }
            }
        }

        let value = &self.source[self.start..self.index];

        match value.parse::<f64>() {
            Ok(v) => Ok(v),
            Err(_) => Err(LexError::InvalidNumber),
        }
    }

    fn identifier(&mut self) -> TokenType {
        let mut reserved: HashMap<&'static str, TokenType> = HashMap::new();
        reserved.insert("and", TokenType::And);
        reserved.insert("or", TokenType::Or);
        reserved.insert("match", TokenType::Match);
        reserved.insert("enum", TokenType::Enum);
        reserved.insert("struct", TokenType::Struct);
        reserved.insert("let", TokenType::Let);
        reserved.insert("const", TokenType::Const);
        reserved.insert("false", TokenType::False);
        reserved.insert("true", TokenType::True);
        reserved.insert("fn", TokenType::Fn);
        reserved.insert("return", TokenType::Return);

        while let Some(i) = self.peek() {
            if i.is_alphabetic() || i == '_' {
                self.next();
            } else {
                break;
            }
        }

        let identifier = &self.source[self.start..self.index];
        if let Some(token_type) = reserved.get(identifier) {
            token_type.to_owned()
        } else {
            TokenType::Identifier(identifier.to_string())
        }
    }

    fn doc_comment(&mut self) -> Result<String, LexError> {
        let mut ends_with_newline = false;
        while let Some(i) = self.peek() {
            if i != '/' || i != '\n' {
                self.next();
            } else {
                if i == '\n' {
                    ends_with_newline = true;
                } else if i == '/' {
                    // todo: remove unwrap
                    let next = self.peek().unwrap();
                    let fin = self.peek_nth(1).unwrap();

                    if fin != '/' || next != '/' {
                        return Err(LexError::UnterminatedComment);
                    }
                }

                break;
            }
        }

        let idx = match ends_with_newline {
            true => self.index,
            false => self.index - 3,
        };

        let comment = &self.source[self.start + 3..idx];

        Ok(comment.to_string())
    }
}

pub fn tokenise(src: &str) -> Result<Vec<Token>, LexError> {
    let mut tokens = Vec::new();
    let mut lexer = Lexer::new(src);

    while !lexer.is_ended() {
        let c = lexer.peek();
        lexer.next();

        match c {
            Some(c) => {
                let tok = match c {
                    '(' => TokenType::LParen,
                    ')' => TokenType::RParen,
                    '{' => TokenType::LBrace,
                    '}' => TokenType::RBrace,
                    ',' => TokenType::Comma,
                    '.' => TokenType::Period,
                    '+' => TokenType::Plus,
                    '-' => TokenType::Minus,
                    '*' => TokenType::Asterisk,
                    '/' => match lexer.peek() {
                        Some(c) if c == '/' => {
                            if let Some(c) = lexer.peek_nth(1) {
                                match c {
                                    '/' => {
                                        lexer.next_nth(2);
                                        let d = lexer.doc_comment();
                                        match d {
                                            Ok(d) => TokenType::DocComment(d),
                                            Err(_) => todo!(),
                                        }
                                    }
                                    _ => {
                                        lexer.advance_line();
                                        TokenType::Comment
                                    }
                                }
                            } else {
                                TokenType::EOI
                            }
                        }
                        Some(_) => TokenType::Slash,
                        None => TokenType::EOI,
                    },
                    ';' => TokenType::Semicolon,
                    '!' => match lexer.peek() {
                        Some(c) => match c {
                            '=' => {
                                if let Some(c) = lexer.peek_nth(1) {
                                    match c {
                                        '=' => {
                                            lexer.next_nth(2);
                                            TokenType::NotStrictEqual
                                        }
                                        _ => {
                                            lexer.next();
                                            TokenType::NotEqual
                                        }
                                    }
                                } else {
                                    TokenType::EOI
                                }
                            }
                            _ => TokenType::Not,
                        },
                        None => TokenType::EOI,
                    },
                    '=' => match lexer.peek() {
                        Some(c) => match c {
                            '=' => {
                                if let Some(c) = lexer.peek_nth(1) {
                                    match c {
                                        '=' => {
                                            lexer.next_nth(2);
                                            TokenType::StrictEqual
                                        }
                                        _ => {
                                            lexer.next();
                                            TokenType::ComparisonEqual
                                        }
                                    }
                                } else {
                                    TokenType::EOI
                                }
                            }
                            _ => TokenType::Equal,
                        },
                        None => TokenType::EOI,
                    },
                    '>' => match lexer.peek() {
                        Some(c) => {
                            if c == '=' {
                                lexer.next();
                                TokenType::GreaterThanEqualTo
                            } else {
                                TokenType::Greater
                            }
                        }
                        None => TokenType::EOI,
                    },
                    '<' => match lexer.peek() {
                        Some(c) => {
                            if c == '=' {
                                lexer.next();
                                TokenType::LessThanEqualTo
                            } else {
                                TokenType::Less
                            }
                        }
                        None => TokenType::EOI,
                    },
                    '\t' | '\r' | ' ' => TokenType::Whitespace,
                    '\n' => {
                        lexer.advance_line();
                        TokenType::Whitespace
                    }
                    '"' | '`' => {
                        let s = lexer.string(c);
                        match s {
                            Ok(s) => TokenType::String(s),
                            Err(_) => todo!(),
                        }
                    }
                    '0'..='9' => {
                        let n = lexer.number();
                        match n {
                            Ok(n) => TokenType::Number(n),
                            Err(_) => todo!(),
                        }
                    }
                    'a'..='z' | 'A'..='Z' | '_' => lexer.identifier(),
                    _ => TokenType::Unknown,
                };

                tokens.push(lexer.create_token(tok))
            }
            None => tokens.push(lexer.create_token(TokenType::EOI)),
        }
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lexemes() {
        assert_eq!(tokenise(r"!*+-/=<> <= >= == === !==").unwrap().len(), 19);
        // should be
        // 1. 0-1 NOT
        // 2. 1-2 ASTERISK
        // 3. 2-3 PLUS
        // 4. 0-1 MINUS
        // 5. 0-1 SLASH
        // 6. 0-1 EQUAL
        // 7. 0-1 LESS
        // 8. 0-1 GREATER
        // 9. 0-1 WHITESPACE
        // 10. 0-1 LessThanEqualTo
        // 11. 0-1 WHITESPACE
        // 12. 0-1 GreaterThanEqualTo
        // 13. 0-1 WHITESPACE
        // 14. 0-1 ComparisonEquals
        // 15. 0-1 WHITESPACE
        // 16. 0-1 StrictEqual
        // 17. 0-1 Whitespace
        // 18. 0-1 NotStrictEqual
        // 19. 0-1 EOI
    }

    #[test]
    fn string_literals() {
        let tok = tokenise("\"This is a test.\"").unwrap();
        assert_eq!(tok.len(), 2);
        // 1. String
        // 2. EOI

        let lex = tok.get(0).unwrap();
        if let TokenType::String(s) = lex.token_type.clone() {
            assert_eq!(s, "This is a test.");
        } else {
            // purposefully fail
            assert!(false);
        }
    }

    #[test]
    fn integer_literals() {
        let int = tokenise("12").unwrap();

        assert_eq!(int.len(), 2);

        let int_lex = int.get(0).unwrap();

        if let TokenType::Number(n) = int_lex.token_type.clone() {
            assert_eq!(n, 12.0);
        } else {
            assert!(false)
        }
    }

    #[test]
    fn float_literals() {
        let fp = tokenise("12.2").unwrap();

        assert_eq!(fp.len(), 2);

        let fp_lex = fp.get(0).unwrap();

        if let TokenType::Number(n) = fp_lex.token_type.clone() {
            assert_eq!(n, 12.2);
        } else {
            assert!(false)
        }
    }

    #[test]
    fn valid_identifiers() {
        let valid = tokenise("aAzA__").unwrap();

        assert_eq!(valid.len(), 2);

        let lex = valid.get(0).unwrap();

        if let TokenType::Identifier(i) = lex.token_type.clone() {
            assert_eq!(i, String::from("aAzA__"));
        } else {
            assert!(false)
        }
    }

    #[test]
    fn keyword_identifiers() {
        let invalid = tokenise("and").unwrap();

        assert_eq!(invalid.len(), 2);

        let lex = invalid.get(0).unwrap();

        if let TokenType::And = lex.token_type.clone() {
            assert!(true);
        } else {
            assert!(false)
        }
    }

    #[test]
    fn comment_tests() {}

    #[test]
    fn doc_comment_tests() {
        let doc_comment = tokenise("/// This is a doc comment ///").unwrap();
        assert_eq!(doc_comment.len(), 2);

        let lex = doc_comment.get(0).unwrap();

        if let TokenType::DocComment(doc) = lex.token_type.clone() {
            assert_eq!(doc, " This is a doc comment ");
        } else {
            assert!(false);
        }
    }
}
