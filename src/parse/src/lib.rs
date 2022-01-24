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

    fn advance(&mut self, by: usize) -> () {
        if let Ok(is_ended) = self.is_ended() {
            self.index += by;
        }
    }

    /// Looks at the next token without advancing the parser
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.index)
    }

    // returns the nth character after index without advancing parser
    fn peek_nth(&self, idx: usize) -> Option<&Token> {
        self.tokens.get(self.index + idx)
    }

    // returns the rest of the tokens in the present statement without advancing the parser
    fn peek_end_of_statement(&self) -> Vec<Token> {
        let mut idx = self.index;
        while idx < self.tokens.len() {
            let token_type = self.peek().unwrap().token_type.clone();
            if token_type != TokenType::Semicolon || token_type != TokenType::EOI {
                idx += 1;
            } else {
                break;
            }
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
    fn is_ended(&self) -> Result<Node, ParseError> {
        if self.index >= self.tokens.len() {
            Err(ParseError::UnexpectedTermination)
        } else {
            Ok(Node::EOI)
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

    // returns the index of the next right paranthesis
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

    // converts a standard token to an operator
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
        println!("{}", self.index);
        self.is_ended()?;

        let token = self.tokens[self.index].clone();
        self.index += 1;

        match token.token_type.clone() {
            TokenType::LParen => {}
            TokenType::LBrace => {}
            TokenType::Comma => {}
            TokenType::Period => {}
            TokenType::Plus | TokenType::Minus | TokenType::Asterisk | TokenType::Slash => {
                let node = Parser::expr_op(self.tokens.clone());
                match node {
                    Ok(ast_node) => return Ok(ast_node),
                    Err(error) => return Err(error),
                }
            }
            TokenType::Semicolon => {}
            TokenType::NotEqual => {}
            TokenType::NotStrictEqual => {}
            TokenType::ComparisonEqual => {}
            TokenType::Equal => return Ok(Node::Identifier(String::from("equal is the problem"))),
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
            TokenType::Identifier(_) => {
                // problem: since this returns nothing the test will fail
                // but does not need to be parsed in `let` declaration
                // so find a way to make the parser advance past the # of tokens in a let declaration
                // in the part of this method that handles let declarations
                return Ok(Node::Identifier(String::from("equal is the problem")));
            }
            TokenType::And => {}
            TokenType::AndAmpersand => {}
            TokenType::Or => {}
            TokenType::OrPipe => {}
            TokenType::Match => {}
            TokenType::Enum => {}
            TokenType::Struct => {}
            TokenType::Let => {
                let declaration = self.peek_end_of_statement();
                let identifier = declaration.get(0).unwrap().to_owned();

                match identifier.token_type {
                    TokenType::Identifier(ident) => {
                        return Ok(Node::Variable {
                            variable_type: Var::Let,
                            identifier: Box::new(Node::Identifier(ident)),
                            value: Box::new(Parser::expr_op(declaration[1..].to_vec()).unwrap()),
                        });
                    }
                    _ => return Err(ParseError::InvalidIdentifier),
                }
            }
            TokenType::Const => {}
            TokenType::True => return Ok(Node::Boolean(true)),
            TokenType::False => return Ok(Node::Boolean(false)),
            TokenType::Fn => {}
            TokenType::Return => {}
            TokenType::EOI => return Ok(Node::EOI),
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
            TokenType::Comment | TokenType::DocComment(_) | TokenType::Whitespace => false,
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
    fn test_plus_generates_proper_ast() {
        let ast = parse(tokenise("2+2").unwrap()).unwrap();
        assert_eq!(ast.len(), 4);
    }

    #[test]
    fn test_minus_generates_proper_ast() {
        let ast = parse(tokenise("2-2").unwrap()).unwrap();
        assert_eq!(ast.len(), 4);
    }

    #[test]
    fn test_multiply_generates_proper_ast() {
        let ast = parse(tokenise("2*2").unwrap()).unwrap();
        assert_eq!(ast.len(), 4);
    }

    #[test]
    fn test_divide_generates_proper_ast() {
        let ast = parse(tokenise("2/2").unwrap()).unwrap();
        assert_eq!(ast.len(), 4);
    }

    #[test]
    fn let_binding_generates_proper_ast() {
        let ast = parse(tokenise("let t = 2+2").unwrap()).unwrap();
        println!("{:#?}", ast)
    }
}
