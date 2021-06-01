use expr::Node;
use lex::{Token, TokenType};

pub mod expr;

pub mod error;
use error::ParseError;

pub struct Parser<'a> {
    tokens: Vec<Token<'a>>,
    nodes: Vec<Node>,
    index: usize,
}

impl<'a> Parser<'a> {
    fn new(tok: Vec<Token>) -> Parser {
        Parser {
            tokens: tok,
            nodes: Vec::new(),
            index: 0,
        }
    }

    /// returns the next token, advancing the parser
    fn next(&mut self) -> Option<&Token> {
        if let Ok(is_ended) = self.is_ended() {
            self.index += 1;
        } else {
            return None;
        }

        self.peek_previous()
    }

    /// Looks at the next token without advancing the parser
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.index)
    }

    /// looks at the previous token without changing the parser's place
    fn peek_previous(&self) -> Option<&Token> {
        self.tokens.get(self.index - 1)
    }

    /// returns Ok(true) if there's nothing left to parse (EOI), Ok(false) if there is, and Err(ParseError) if something went wrong peeking the next token
    fn is_ended(&self) -> Result<(), ParseError> {
        if self.index >= self.tokens.len() {
            Err(ParseError::UnexpectedTermination)
        } else {
            Ok(())
        }
    }

    fn parse(&mut self) -> Result<Vec<Node>, ParseError> {
        while self.index < self.tokens.len() {
            let n = self.expr()?;
            self.nodes.push(n);
        }

        Ok(self.nodes.clone())
    }

    fn expr(&mut self) -> Result<Node, ParseError> {
        self.is_ended()?;

        let token = self.tokens[self.index].clone();
        self.index += 1;

        if tok.kind == TokKind::NegOp {
            let expr = self.expr()?;
            return Ok(Node::UnaryExpr {
                operation: tok.kind.clone(),
                body: Box::new(expr),
            });
        }

        match token.token_type.clone() {
            TokenType::LParen => {}
            TokenType::RParen => {}
            TokenType::LBrace => {}
            TokenType::RBrace => {}
            TokenType::Comma => {}
            TokenType::Period => {}
            TokenType::Plus => {}
            TokenType::Minus => {}
            TokenType::Asterisk => {}
            TokenType::Slash => {}
            TokenType::Semicolon => {}
            TokenType::NotEqual => {}
            TokenType::NotStrictEqual => {}
            TokenType::ComparisonEqual => {}
            TokenType::Equal => {}
            TokenType::StrictEqual => {}
            TokenType::Greater
            | TokenType::GreaterThanEqualTo
            | TokenType::Less
            | TokenType::LessThanEqualTo => {
                let lhs = self.peek_previous().unwrap().to_owned();
                let expr = token;
                let rhs = self.peek().unwrap().to_owned();

                return Ok(Node::BinaryExpr{ lhs: Box::new(lhs), rhs: Box::new(rhs), operation: expr.token_type})
            }
            TokenType::Not => {}
            TokenType::String(s) => return Ok(Node::String(s)),
            TokenType::Name => {}
            TokenType::Number(n) => return Ok(Node::Number(n)),
            TokenType::Identifier(_) => {}
            TokenType::And => {}
            TokenType::AndAmpersand => {}
            TokenType::Or => {}
            TokenType::OrPipe => {}
            TokenType::Match => {}
            TokenType::Enum => {}
            TokenType::Struct => {}
            TokenType::Let => {}
            TokenType::Const => {}
            TokenType::True => return Ok(Node::Boolean(true)),
            TokenType::False => return Ok(Node::Boolean(false)),
            TokenType::Fn => {}
            TokenType::Return => {}
            TokenType::EOI => {}
            TokenType::Comment => {}
            TokenType::DocComment(_) => {}
            TokenType::Whitespace => {}
            TokenType::Unknown => {}
        }

        Err(ParseError::UnexpectedTermination)
    }
}

pub fn parse(tokens: Vec<Token>) -> Result<Vec<Node>, ParseError> {
    let filtered: Vec<Token> = tokens
        .to_owned()
        .into_iter()
        .filter(|x| match x.token_type {
            TokenType::DocComment(_) | TokenType::Whitespace | TokenType::Whitespace => false,
            _ => true,
        })
        .collect();

    let mut parser = Parser::new(tokens);

    parser.parse()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
