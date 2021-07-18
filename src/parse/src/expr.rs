use lex::TokenType;

#[derive(Debug, Clone)]
pub enum Op {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug, Clone)]
pub enum Var {
    Let,
    Const,
}

/// AST Node
#[derive(Debug, Clone)]
pub enum Node {
    Variable {
        variable_type: Var,
        identifier: Box<Node>,
        value: Box<Node>,
    },
    BinaryExpr {
        operation: Op,
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
    UnaryExpr {
        operation: TokenType,
        body: Box<Node>,
    },
    Fn {
        name: Box<Node>,
        args: Box<Node>,
        body: Box<Node>,
    },
    FnCall {
        function: Box<Node>,
        args: Vec<Node>,
    },
    Number(f64),
    String(String),
    Boolean(bool),
    Identifier(String),
}
