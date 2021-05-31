#[derive(Debug, Clone)]
pub struct Token<'a> {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
    pub literal: &'a str,
}

#[derive(Debug, Clone)]
pub enum TokenType {
    // single characters
    LParen,
    RParen,
    LBrace,
    RBrace,
    Comma,
    Period,
    Minus,
    Plus,
    Semicolon,
    Slash,

    // 1-3 characters
    NotEqual,
    NotStrictEqual,
    Equal,
    StrictEqual,
    Greater,
    GreaterThanEqualTo,
    Less,
    LessThanEqualTo,

    // literals
    String,
    Name,
    Number,

    // keywords
    And,
    AndAmpersand,
    Or,
    OrPipe,
    Match,
    Enum,
    Struct,
    Let,
    Const,

    // end of input
    EOI,
}
