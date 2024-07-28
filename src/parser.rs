use super::lexer::{Token, Operator};
use super::Lexer;
use super::expr::{Expr, ParseError};

pub struct Parser<'a> {
    pub lexer: Lexer<'a>,
    current_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(mut lexer: Lexer<'a>) -> Self {
        let current_token = lexer.next_token();
        Parser { lexer, current_token }
    }

    fn advance(&mut self) {
        self.current_token = self.lexer.next_token();
    }

    fn expect(&mut self, expected: Token) -> Result<(), ParseError> {
        if self.current_token == expected {
            self.advance();
            Ok(())
        } else {
            Err(ParseError::UnexpectedToken(self.current_token.clone()))
        }
    }

    pub fn parse(&mut self) -> Result<Expr, ParseError> {
        self.parse_block()
    }

    fn parse_block(&mut self) -> Result<Expr, ParseError> {
        let mut exprs = Vec::new();
        self.expect(Token::LeftBrace)?;
        while self.current_token != Token::RightBrace && self.current_token != Token::EOF {
            exprs.push(self.parse_expr()?);
        }
        self.expect(Token::RightBrace)?;
        Ok(Expr::Block(exprs))
    }

    fn parse_expr(&mut self) -> Result<Expr, ParseError> {
        self.parse_if()
    }

    fn parse_if(&mut self) -> Result<Expr, ParseError> {
        if self.current_token == Token::If {
            self.advance();
            let condition = Box::new(self.parse_expr()?);
            let then_branch = Box::new(self.parse_block()?);
            let else_branch = if self.current_token == Token::Else {
                self.advance();
                Some(Box::new(self.parse_block()?))
            } else {
                None
            };
            Ok(Expr::If { condition, then_branch, else_branch })
        } else {
            self.parse_assignment()
        }
    }

    fn parse_assignment(&mut self) -> Result<Expr, ParseError> {
        match &self.current_token {
            Token::Int(i) => {}
            Token::Uint(i) => {}
            Token::Float(f) => {}
            _ => {}
        }
        if let Token::Identifier(name) = &self.current_token {
            let name = name.clone();
            self.advance();
            if self.current_token == Token::Operator(Operator::Eq) {
                self.advance();
                let expr = Box::new(self.parse_expr()?);
                Ok(Expr::Assignment { name, expr })
            } else {
                Err(ParseError::UnexpectedToken(self.current_token.clone()))
            }
        } else {
            self.parse_binary_op()
        }
    }

    fn parse_binary_op(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_unary_op()?;
        while let Token::Operator(op) = &self.current_token {
            let op = op.clone();
            self.advance();
            let right = self.parse_unary_op()?;
            left = Expr::BinaryOp {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_unary_op(&mut self) -> Result<Expr, ParseError> {
        if let Token::Operator(op) = &self.current_token {
            let op = op.clone();
            self.advance();
            let expr = self.parse_primary()?;
            Ok(Expr::UnaryOp {
                op,
                expr: Box::new(expr),
            })
        } else {
            self.parse_primary()
        }
    }

    fn parse_primary(&mut self) -> Result<Expr, ParseError> {
        match &self.current_token {
            Token::None => {
                self.advance();
                Ok(Expr::None)
            }
            Token::Bool(b) => {
                let b = *b;
                self.advance();
                Ok(Expr::Bool(b))
            }
            Token::Int(i) => {
                let i = *i;
                self.advance();
                Ok(Expr::Int(i))
            }
            Token::Uint(u) => {
                let u = *u;
                self.advance();
                Ok(Expr::Uint(u))
            }
            Token::Char(c) => {
                let c = *c;
                self.advance();
                Ok(Expr::Char(c))
            }
            Token::Float(f) => {
                let f = *f;
                self.advance();
                Ok(Expr::Float(f))
            }
            Token::String(s) => {
                let s = s.clone();
                self.advance();
                Ok(Expr::String(s))
            }
            Token::Identifier(name) => {
                let name = name.clone();
                self.advance();
                if self.current_token == Token::LeftParen {
                    self.advance();
                    let mut args = Vec::new();
                    while self.current_token != Token::RightParen {
                        args.push(self.parse_expr()?);
                        if self.current_token == Token::Comma {
                            self.advance();
                        }
                    }
                    self.expect(Token::RightParen)?;
                    Ok(Expr::FunctionCall { name, args })
                } else {
                    Ok(Expr::Identifier(name))
                }
            }
            Token::LeftBrace => self.parse_block(),
            _ => Err(ParseError::UnexpectedToken(self.current_token.clone())),
        }
    }
}

