#[derive(Debug, Clone, PartialEq)]
pub struct Token<'a> {
    /// the type of token
    pub token_type: TokenType,
    /// token placement
    pub span: (usize, usize),
    /// token source
    pub source: &'a str,
}

#[derive(Debug)]
pub struct Position {
    line: usize,
    col: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // single characters
    LParen,
    RParen,
    LBrace,
    RBrace,
    Comma,
    Period,
    Plus,
    Minus,
    Asterisk,
    Slash,
    Semicolon,

    // 1-3 characters
    NotEqual,
    NotStrictEqual,
    ComparisonEqual,
    // assignment equals
    Equal,
    StrictEqual,
    Greater,
    GreaterThanEqualTo,
    Less,
    LessThanEqualTo,
    // `!x`
    Not,

    // literals
    String(String),
    Name,
    Number(f64),

    Identifier(String),

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
    True,
    False,
    Fn,
    Return,

    // end of input
    EOI,

    // ignored lexemes
    Comment,
    DocComment(String),
    Whitespace,

    // errors
    Unknown,
}
