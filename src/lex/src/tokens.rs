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
    // (
    LParen,
    // )
    RParen,
    // {
    LBrace,
    // }
    RBrace,
    // ,
    Comma,
    // .
    Period,
    // +
    Plus,
    // -
    Minus,
    // *
    Asterisk,
    // /
    Slash,
    // ;
    Semicolon,

    // 1-3 characters
    // !=
    NotEqual,
    // !==
    NotStrictEqual,
    // ==
    ComparisonEqual,
    // assignment equals
    // =
    Equal,
    // ===
    StrictComparisonEqual,
    // >
    Greater,
    // >=
    GreaterThanEqualTo,
    // <
    Less,
    // <=
    LessThanEqualTo,
    // `!`
    Not,

    // literals
    String(String),
    Name,
    Number(f64),

    Identifier(String),

    // keywords
    // 'and'
    And,
    // &
    AndAmpersand,
    // 'or'
    Or,
    // |
    OrPipe,
    // 'match'
    Match,
    // 'enum'
    Enum,
    // 'struct'
    Struct,
    // 'let'
    Let,
    // 'const'
    Const,
    // 'true'
    True,
    // 'false'
    False,
    // 'fn'
    Fn,
    // 'return'
    Return,

    // end of input
    EOI,

    // ignored lexemes
    // '//'
    Comment,
    // '///'
    DocComment(String),
    // \r, \n, \t, ' '
    Whitespace,

    // errors
    Unknown,
}

// impl From<T> for TokenType {
//     fn from(token: T) -> Self {
//         match token {
//             TokenType::And => TokenType::And,
//             TokenType::LParen => TokenType::LParen,
//             TokenType::RParen => TokenType::RParen,
//             TokenType::LBrace => TokenType::LBrace,
//             TokenType::RBrace => TokenType::RBrace,
//             TokenType::Comma => TokenType::Comma,
//             TokenType::Period => TokenType::Period,
//             TokenType::Plus => TokenType::Plus,
//             TokenType::Minus => TokenType::Minus,
//             TokenType::Asterisk => TokenType::Asterisk,
//             TokenType::Slash => TokenType::Slash,
//             TokenType::Semicolon => TokenType::Semicolon,
//             TokenType::NotEqual => TokenType::NotEqual,
//             TokenType::NotStrictEqual => TokenType::NotStrictEqual,
//             TokenType::ComparisonEqual => TokenType::ComparisonEqual,
//             TokenType::Equal => TokenType::Equal,
//             TokenType::StrictComparisonEqual => TokenType::StrictComparisonEqual,
//             TokenType::Greater => TokenType::Greater,
//             TokenType::GreaterThanEqualTo => TokenType::GreaterThanEqualTo,
//             TokenType::Less => TokenType::Less,
//             TokenType::LessThanEqualTo => TokenType::LessThanEqualTo,
//             TokenType::Not => TokenType::Not,
//             TokenType::String(_) => TokenType::String(_),
//             TokenType::Name => TokenType::Name,
//             TokenType::Number(_) => TokenType::Number(_),
//             TokenType::Identifier(_) => TokenType::Identifier(_),
//             TokenType::AndAmpersand => TokenType::AndAmpersand,
//             TokenType::Or => TokenType::Or,
//             TokenType::OrPipe => TokenType::OrPipe,
//             TokenType::Match => TokenType::Match,
//             TokenType::Enum => TokenType::Enum,
//             TokenType::Struct => TokenType::Struct,
//             TokenType::Let => TokenType::Let,
//             TokenType::Const => TokenType::Const,
//             TokenType::True => TokenType::True,
//             TokenType::False => TokenType::False,
//             TokenType::Fn => TokenType::Fn,
//             TokenType::Return => TokenType::Return,
//             TokenType::EOI => TokenType::EOI,
//             TokenType::Comment => TokenType::Comment,
//             TokenType::DocComment(_) => TokenType::DocComment(_),
//             TokenType::Whitespace => TokenType::Whitespace,
//             TokenType::Unknown => TokenType::Unknown,
//         }
//     }
// }
