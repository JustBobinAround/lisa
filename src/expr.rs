use crate::type_def::Type;

use super::lexer::Operator;
use super::lexer::Token;
#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Type(Type),
    None,
    Bool(bool),
    Int(i64),
    Uint(u64),
    Char(char),
    Float(f64),
    String(String),
    Identifier(String),
    BinaryOp {
        left: Box<Expr>,
        op: Operator,
        right: Box<Expr>,
    },
    UnaryOp {
        op: Operator,
        expr: Box<Expr>,
    },
    FunctionCall {
        name: String,
        args: Vec<Expr>,
    },
    Assignment {
        name: String,
        expr: Box<Expr>,
    },
    If {
        condition: Box<Expr>,
        then_branch: Box<Expr>,
        else_branch: Option<Box<Expr>>,
    },
    Function {
        param_type: Type,
        return_type: Type,
        block: Box<Expr>,
    },
    Block(Vec<Expr>),
}

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken(Token),
    UnexpectedEOF,
}

