use std::collections::{HashMap, HashSet};
use std::ops::Deref;
use std::sync::Arc;

use crate::expr;
use crate::type_def::Type;

use super::lexer::{Token, Operator};
use super::Lexer;
use super::expr::{Expr, ParseError};

pub struct Parser<'a> {
    pub lexer: Lexer<'a>,
    current_token: Token,
}
pub struct BlockState {
    current_expr: Arc<Expr>,
    variables: HashMap<String, Arc<Expr>>
}

impl<'a> Parser<'a> {
    pub fn new(mut lexer: Lexer<'a>) -> Self {
        let current_token = lexer.next_token();
        Parser { lexer, current_token}
    }

    fn advance(&mut self) {
        self.current_token = self.lexer.next_token();
    }

    fn expect(&mut self, expected: Token, msg: &'static str) -> Result<(), ParseError> {
        if self.current_token == expected {
            self.advance();
            Ok(())
        } else {
            Err(ParseError::UnexpectedToken(self.current_token.clone(), msg.to_string()))
        }
    }

    pub fn parse(&mut self) -> Result<Expr, ParseError> {
        self.parse_block(true)
    }

    pub fn parse_block(&mut self, is_main: bool) -> Result<Expr, ParseError> {
        if !is_main {
            self.expect(Token::LeftBracket, "Expected starting bracket at start of block")?;
        }
        let mut variables = HashMap::new();
        let mut exprs = Vec::new();
        let mut prior_expr = None;
        while self.current_token != Token::EOF {
            let expr = self.parse_expr(&mut variables, prior_expr)?;
            println!("{:?}", self.current_token);
            exprs.push(expr.clone());
            prior_expr = Some(expr);
            match self.current_token {
                Token::RightBracket => {
                    break;
                }
                Token::Semicolon => {
                    self.advance();
                }
                _ => {
                    return Err(ParseError::BadToken(self.current_token.clone(), "Expected a semicolon or a closing bracket to block".to_string()))
                }
            }
        }

        println!("Declared variables: {:?}", variables);
        if !is_main {
            self.expect(Token::RightBracket, "Expected closing bracket at end of block")?;
        } else {
            self.expect(Token::EOF, "Expected EOF token at end of main block")?;
        }
        Ok(Expr::Block(exprs))
    }

    fn parse_expr(&mut self, mut variables: &mut HashMap<String, Arc<Expr>>, prior_expr: Option<Arc<Expr>>) -> Result<Arc<Expr>, ParseError> {
        let left_expr: Arc<Expr> = match self.current_token {
            Token::Operator(ref op) => {
                let op = op.clone();
                self.advance();
                self.parse_unary(&mut variables, op)?.into()
            }
            Token::Bool(b) => {
                self.advance();
                Expr::Bool(b).into()
            }
            Token::Int(i) => {
                self.advance();
                Expr::Int(i).into()
            }
            Token::Uint(u) => {
                self.advance();
                Expr::Uint(u).into()
            }
            Token::Char(c) => {
                self.advance();
                Expr::Char(c).into()
            }
            Token::Float(f) => {
                self.advance();
                Expr::Float(f).into()
            }
            Token::String(ref s) => {
                let s = s.clone();
                self.advance();
                Expr::String(s).into()
            }
            Token::Identifier(ref name) => {
                let name = name.clone();
                self.advance();
                self.parse_identifer(name)?.into()
            }
            Token::LeftParen => {
                self.advance();
                let expr = self.parse_expr(variables, prior_expr)?;
                self.expect(Token::RightParen, "Expected closing paren after parsing inner expression")?;
                expr.into()
            }
            _ => {
                return Err(ParseError::BadToken(self.current_token.clone(), "Found wrong token while parsing expression".to_string()))
            }
        };
        if self.current_token==Token::Period {
            self.advance();
            match self.current_token {
                Token::Identifier(ref name) => {
                    let name = name.clone();
                    self.advance();
                    if *name=="as" {
                        self.expect(Token::LeftParen, "")?;
                        match self.current_token {
                            Token::Identifier(ref name) => {
                                let name = name.clone();
                                self.advance();
                                variables.insert(name.to_string(), left_expr.clone());
                                self.expect(Token::RightParen, "")?;
                            }
                            _ => {
                                unimplemented!("method call");
                            }
                        }
                    } else {
                        unimplemented!("method call");
                    }
                }
                _ => {
                    unimplemented!("method call");
                }
            }
        }
        match self.current_token {
            Token::Operator(ref op) => {
                let op = op.clone();
                self.advance();
                self.parse_binary(&mut variables, op, left_expr)
            }
            _ => {
                Ok(left_expr)
            }
        }
    }

    fn parse_binary(&mut self, mut variables: &mut HashMap<String, Arc<Expr>>, op: Arc<Operator>, left_expr: Arc<Expr>) -> Result<Arc<Expr>, ParseError> {
        let right_expr: Arc<Expr> = self.parse_expr(&mut variables, Some(left_expr.clone()))?;
        match *op {
            Operator::Add => {
                Ok(Expr::add(left_expr, right_expr).into())
            }
            Operator::Sub=> {
                Ok(Expr::sub(left_expr, right_expr).into())
            }
            Operator::Mul=> {
                Ok(Expr::mult(left_expr, right_expr).into())
            }
            Operator::Div=> {
                Ok(Expr::div(left_expr, right_expr).into())
            }
            Operator::Mod=> {
                Ok(Expr::modd(left_expr, right_expr).into())
            }
            Operator::Eq=> {
                Ok(Expr::eq(left_expr, right_expr).into())
            }
            Operator::Neq=> {
                Ok(Expr::neq(left_expr, right_expr).into())
            }
            _ => {
                Ok(Expr::BinaryOp{left: left_expr, op, right: right_expr}.into())
            }
        }
    }
    fn parse_unary(&mut self, mut variables: &mut HashMap<String, Arc<Expr>>,op: Arc<Operator>) -> Result<Expr, ParseError> {
        let right_expr: Arc<Expr> = self.parse_expr(&mut variables, None)?;
        match *op {
            Operator::Not => {
                Ok(Expr::UnaryOp{op: op.clone(), expr:right_expr})
            }
            Operator::Sub => {
                match *right_expr {
                    Expr::Int(i) => {
                        Ok(Expr::Int(-i))
                    }
                    Expr::Float(f) => {
                        Ok(Expr::Float(-f))
                    }
                    _ => {
                        Err(ParseError::BadToken(self.current_token.clone(), "Expected floating point or integer following negative unary operator".to_string()))
                    }
                }
            }
            _ => {
                Err(ParseError::BadToken(self.current_token.clone(), "Expected expresion or unary operator".to_string()))
            }
        }
    }

    fn parse_identifer(&mut self, name: Arc<String>) -> Result<Expr, ParseError> {
        match self.current_token {
            Token::Colon => {
                unimplemented!("type declaration");
            }
            Token::LeftParen => {
                unimplemented!("function call");
            }
            _ => {
                Ok(Expr::Identifier(name))
            }
        }
    }
}

