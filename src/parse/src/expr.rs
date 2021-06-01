use lex::TokenType;

#[derive(Debug, Clone)]
pub enum Node {
    BinaryExpr {
        operation: TokenType,
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
    UnaryExpr {
        operation: TokenType,
        body: Box<Node>
    },
    Number(f64),
    String(String),
    Boolean(bool),
    Identity(String),
}
