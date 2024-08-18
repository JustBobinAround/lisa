use std::sync::Arc;

use crate::lexer::{Lexer, Op, Token};

#[derive(Debug, PartialEq, Clone)]
pub struct PrototypeAST(pub String, pub Vec<String>);

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionAST(pub PrototypeAST, pub ExprAST);

type ParseResult<T> = Result<T, String>;


#[derive(Debug, PartialEq, Clone)]
pub enum ExprAST {
    Int(i64),
    Variable(String),
    BinOp(Op, Box<ExprAST>, Box<ExprAST>),
}


pub struct Parser<I>
where
    I: Iterator<Item = char>,
{
    lexer: Lexer<I>,
    current_token: Token,
}

impl<I> Parser<I>
where
    I: Iterator<Item = char>,
{
    pub fn new(mut lexer: Lexer<I>) -> Self {
        let token = lexer.next_token();
        Parser {
            lexer,
            current_token: token,
        }
    }
    pub fn current_token(&self) -> &Token {
        &self.current_token
    }


    pub fn next_token(&mut self) {
        self.current_token = self.lexer.next_token();
    }

    pub fn parse_top_level_expr(&mut self) -> ParseResult<FunctionAST> {
        let e = self.parse_expression()?;
        let proto = PrototypeAST("__anon_expr".into(), Vec::new());
        Ok(FunctionAST(proto, e))
    }

    fn parse_expression(&mut self) -> ParseResult<ExprAST> {
        let prior = match self.current_token() {
            Token::Int(i) => ExprAST::Int(*i),
            _ => return Err("unknown token when expecting an expression".into()),
        };
        self.next_token();
        match self.current_token() {
            Token::Op(op) => {
                let op = op.clone();
                self.next_token();
                let rhs = self.parse_expression()?;
                Ok(ExprAST::BinOp(op, prior.into(), rhs.into()))
            }
            _ => Ok(prior)
        }
    }
}

fn token_ordering(tok: &Token) -> isize {
    match tok {
        Token::Char('<') => 10,
        Token::Char('+') => 20,
        Token::Char('-') => 20,
        Token::Char('*') => 40,
        _ => -1,
    }
}
