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
        // let or const
        variable_type: Var,
        // name of the variable
        identifier: Box<Node>,
        // value of the variable
        value: Box<Node>,
    },
    // e.g. 2+2
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
    EOI,
}
