use expr::{Node, Op, Var};
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

    // returns the nth character after index without advancing parser
    fn peek_nth(&self, idx: usize) -> Option<&Token> {
        self.tokens.get(self.index + idx)
    }

    fn peek_end_of_statement(&self) -> Vec<Token> {
        let mut idx = 0;
        while self.peek().unwrap().token_type != TokenType::Semicolon {
            idx += 1;
        }

        self.tokens[self.index..idx].to_vec()
    }

    /// looks at the previous token without changing the parser's place
    fn peek_previous(&self) -> Option<&Token> {
        self.tokens.get(self.index - 1)
    }

    fn peek_previous_nth(&self, idx: usize) -> Option<&Token> {
        self.tokens.get(self.index - idx)
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

    fn until_rparan(&self) -> Option<Vec<Token>> {
        for (idx, i) in self.tokens.clone().into_iter().enumerate() {
            if i.token_type == TokenType::RParen {
                return Some(self.tokens[0..idx].to_vec());
            }
        }

        None
    }

    fn next_rparan_idx(&self) -> Option<usize> {
        for (idx, i) in self.tokens.clone().into_iter().enumerate() {
            if i.token_type == TokenType::RParen {
                return Some(idx);
            }
        }

        None
    }

    fn is_op(tok: Token) -> bool {
        match tok.token_type {
            TokenType::Plus | TokenType::Minus | TokenType::Asterisk | TokenType::Slash => true,
            // TokenType::RParen
            _ => false,
        }
    }

    fn tok_to_op(tok: Token) -> Option<Op> {
        match tok.token_type {
            TokenType::Plus => Some(Op::Add),
            TokenType::Minus => Some(Op::Subtract),
            TokenType::Asterisk => Some(Op::Multiply),
            TokenType::Slash => Some(Op::Divide),
            _ => None,
        }
    }

    fn expr_op(toks: Vec<Token>) -> Result<Node, ParseError> {
        for (idx, i) in toks.clone().into_iter().enumerate() {
            if Parser::is_op(i.clone()) {
                let lhs = toks.get(idx - 1).unwrap().to_owned();
                let rhs = toks.get(idx + 1).unwrap().to_owned();

                // if Parser::is_op(lhs.to_owned()) == false
                //     || Parser::is_op(rhs.to_owned()) == false
                // {
                //     let next_rparen = self.until_rparan().unwrap().len();
                //     Parser::op(toks[idx..next_rparen].to_vec());
                // }
                let l = match lhs.token_type {
                    TokenType::Number(n) => Node::Number(n),
                    TokenType::Identifier(i) => Node::Identifier(i),
                    _ => return Err(ParseError::UnexpectedToken),
                };

                let r = match rhs.token_type {
                    TokenType::Number(n) => Node::Number(n),
                    TokenType::Identifier(i) => Node::Identifier(i),
                    _ => return Err(ParseError::UnexpectedToken),
                };

                return Ok(Node::BinaryExpr {
                    operation: Parser::tok_to_op(i).unwrap(),
                    lhs: Box::new(l),
                    rhs: Box::new(r),
                });
            }
        }

        Err(ParseError::UnexpectedToken)
    }

    fn op(&self) -> Result<Node, ParseError> {
        let toks = self.tokens[self.index..self.next_rparan_idx().unwrap()].to_vec();
        for (idx, i) in toks.into_iter().enumerate() {
            match i.token_type {
                TokenType::LParen => {
                    Parser::expr_op(self.until_rparan().unwrap());
                }
                _ => return Err(ParseError::UnexpectedToken),
            }
        }

        Err(ParseError::UnexpectedToken)
    }

    fn expr(&mut self) -> Result<Node, ParseError> {
        self.is_ended()?;

        let token = self.tokens[self.index].clone();
        self.index += 1;

        match token.token_type.clone() {
            TokenType::LParen => {}
            TokenType::LBrace => {}
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
            TokenType::StrictComparisonEqual => {}
            TokenType::Greater
            | TokenType::GreaterThanEqualTo
            | TokenType::Less
            | TokenType::LessThanEqualTo => {
                // let lhs = self.peek_previous().unwrap().to_owned();
                // let expr = token;
                // let rhs = self.peek().unwrap().to_owned();
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
            TokenType::Let => {
                let declaration = self.peek_end_of_statement();
                let identifier = declaration.get(0).unwrap().source;
                let value = declaration[2..].to_vec();

                return Ok(Node::Variable {
                    variable_type: Var::Let,
                    identifier: Box::new(Node::Identifier(identifier.to_string())),
                    value: Box::new(self.op().unwrap()),
                });
            }
            TokenType::Const => {}
            TokenType::True => return Ok(Node::Boolean(true)),
            TokenType::False => return Ok(Node::Boolean(false)),
            TokenType::Fn => {}
            TokenType::Return => {}
            TokenType::EOI => {}
            _ => return Err(ParseError::UnexpectedToken),
        }

        Err(ParseError::UnexpectedTermination)
    }
}

pub fn parse(tokens: Vec<Token>) -> Result<Vec<Node>, ParseError> {
    let filtered: Vec<Token> = tokens
        .to_owned()
        .into_iter()
        .filter(|x| match x.token_type {
            TokenType::Comment
            | TokenType::DocComment(_)
            | TokenType::Whitespace
            | TokenType::Whitespace => false,
            _ => true,
        })
        .collect();

    let mut parser = Parser::new(filtered);

    parser.parse()
}

#[cfg(test)]
mod tests {
    use lex::tokenise;

    use super::*;

    #[test]
    fn let_binding_generates_proper_ast() {
        let ast = parse(tokenise("let t = 2+2").unwrap()).unwrap();
    }
}
