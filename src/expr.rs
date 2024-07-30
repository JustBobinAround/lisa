use std::sync::Arc;

use crate::type_def::Type;

use super::lexer::Operator;
use super::lexer::Token;
#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Type(Type),
    Bool(bool),
    Int(i64),
    Uint(u64),
    Char(char),
    Float(f64),
    String(Arc<String>),
    Array(Vec<Expr>),
    Identifier(Arc<String>),
    BinaryOp {
        left: Box<Expr>,
        op: Arc<Operator>,
        right: Box<Expr>,
    },
    UnaryOp {
        op: Arc<Operator>,
        expr: Box<Expr>,
    },
    FunctionCall {
        prior_expr: Box<Expr>,
        name: Arc<String>,
        arg: Box<Expr>,
    },
    SharedAssignment {
        prior_expr: Box<Expr>,
        name: Arc<String>,
    },
    Assignment {
        prior_expr: Box<Expr>,
        name: Arc<String>,
    },
    If {
        condition: Box<Expr>,
        then_branch: Box<Expr>,
        else_branch: Option<Box<Expr>>,
    },
    Function {
        param_sig: u64,
        return_sig: u64,
        block: Box<Expr>,
    },
    TypeExtend {
        type_sig: u64,
        name: Arc<String>,
        expr: Box<Expr>        
    },
    Block(Vec<Expr>),
}

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken(Token),
    UnexpectedEOF,
}

