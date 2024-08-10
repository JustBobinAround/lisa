use std::collections::HashMap;
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
    Array(Vec<Arc<Expr>>),
    Struct{
        pairs: HashMap<String,Arc<Expr>>
    },
    Identifier(Arc<String>),
    BinaryOp {
        left: Arc<Expr>,
        op: Arc<Operator>,
        right: Arc<Expr>,
    },
    UnaryOp {
        op: Arc<Operator>,
        expr: Arc<Expr>,
    },
    If {
        condition: Arc<Expr>,
        then_branch: Arc<Expr>,
        else_branch: Option<Arc<Expr>>,
    },
    Function {
        param_sig: u64,
        return_sig: u64,
        block: Arc<Expr>,
    },
    Block(Vec<Arc<Expr>>),
    Param,
}

impl Expr {
    pub fn primative_str(&self) -> &'static str{
        match self {
            Expr::Bool(_) => {
                "bool"
            }
            Expr::Uint(_) => {
                "uint"
            }
            Expr::Int(_) => {
                "int"
            }
            Expr::Float(_) => {
                "float"
            }
            Expr::Char(_) => {
                "char"
            }
            Expr::String(_) => {
                "string"
            }
            _ => {
                "Complex Expr"
            }
        }
    }
    pub fn is_primative(&self) -> bool {
        match self {
            Expr::Bool(_) => {
                true
            }
            Expr::Uint(_) => {
                true
            }
            Expr::Int(_) => {
                true
            }
            Expr::Float(_) => {
                true
            }
            Expr::Char(_) => {
                true
            }
            Expr::String(_) => {
                true
            }
            _ => {
                false
            }
        }
    }
    pub fn is_bad_primative(left: Arc<Expr>, op: Arc<Operator>, right: Arc<Expr>) -> Result<Expr, ParseError> {
        if right.is_primative() {
            return Err(ParseError::BadExpress(format!("Expect {} while reducing expression, found {}",left.primative_str(),right.primative_str())));
        } else {
            Ok(Expr::BinaryOp { left, op: Operator::Neq.into(), right})
        }
    }
    pub fn neq(left: Arc<Expr>, right: Arc<Expr>) -> Result<Expr, ParseError>{
        match *left{
            Expr::Bool(ba) => {
                match *right{
                    Expr::Bool(bb) => {
                        Ok(Expr::Bool(ba!=bb))
                    }
                    _ => {
                        Expr::is_bad_primative(left, Operator::Neq.into(), right)
                    }
                }
            }
            Expr::Uint(ua) => {
                match *right{
                    Expr::Uint(ub) => {
                        Ok(Expr::Bool(ua!=ub))
                    }
                    _ => {
                        Expr::is_bad_primative(left, Operator::Neq.into(), right)
                    }
                }
            }
            Expr::Int(ia) => {
                match *right{
                    Expr::Int(ib) => {
                        Ok(Expr::Bool(ia!=ib))
                    }
                    _ => {
                        Expr::is_bad_primative(left, Operator::Neq.into(), right)
                    }
                }
            }
            Expr::Float(fa) => {
                match *right{
                    Expr::Float(fb) => {
                        Ok(Expr::Bool(fa!=fb))
                    }
                    _ => {
                        Expr::is_bad_primative(left, Operator::Neq.into(), right)
                    }
                }
            }
            _ => {
                if Arc::<Expr>::as_ptr(&left)==Arc::<Expr>::as_ptr(&right) {
                    Ok(Expr::Bool(false))
                } else {
                    Ok(Expr::BinaryOp { left, op: Operator::Neq.into(), right})
                }
            }
        }
    }
    pub fn eq(left: Arc<Expr>, right: Arc<Expr>) -> Result<Expr, ParseError> {
        match *left{
            Expr::Bool(ba) => {
                match *right{
                    Expr::Bool(bb) => {
                        Ok(Expr::Bool(ba==bb))
                    }
                    _ => {
                        Expr::is_bad_primative(left, Operator::Eq.into(), right)
                    }
                }
            }
            Expr::Uint(ua) => {
                match *right{
                    Expr::Uint(ub) => {
                        Ok(Expr::Bool(ua==ub))
                    }
                    _ => {
                        Expr::is_bad_primative(left, Operator::Eq.into(), right)
                    }
                }
            }
            Expr::Int(ia) => {
                match *right{
                    Expr::Int(ib) => {
                        Ok(Expr::Bool(ia==ib))
                    }
                    _ => {
                        Expr::is_bad_primative(left, Operator::Eq.into(), right)
                    }
                }
            }
            Expr::Float(fa) => {
                match *right{
                    Expr::Float(fb) => {
                        Ok(Expr::Bool(fa==fb))
                    }
                    _ => {
                        Expr::is_bad_primative(left, Operator::Eq.into(), right)
                    }
                }
            }
            _ => {
                if Arc::<Expr>::as_ptr(&left)==Arc::<Expr>::as_ptr(&right) {
                    Ok(Expr::Bool(true))
                } else {
                    Ok(Expr::BinaryOp { left, op: Operator::Eq.into(), right})
                }
            }
        }
    }
    pub fn modd(left: Arc<Expr>, right: Arc<Expr>) -> Result<Expr, ParseError> {
        match *left{
            Expr::Uint(ua) => {
                match *right{
                    Expr::Uint(ub) => {
                        Ok(Expr::Uint(ua%ub))
                    }
                    _ => {
                        Expr::is_bad_primative(left, Operator::Mod.into(), right)
                    }
                }
            }
            Expr::Int(ia) => {
                match *right{
                    Expr::Int(ib) => {
                        Ok(Expr::Int(ia%ib))
                    }
                    _ => {
                        Expr::is_bad_primative(left, Operator::Mod.into(), right)
                    }
                }
            }
            Expr::Float(fa) => {
                match *right{
                    Expr::Float(fb) => {
                        Ok(Expr::Float(fa%fb))
                    }
                    _ => {
                        Expr::is_bad_primative(left, Operator::Mod.into(), right)
                    }
                }
            }
            _ => {
                Ok(Expr::BinaryOp { left, op: Operator::Mod.into(), right})
            }
        }
    }
    pub fn div(left: Arc<Expr>, right: Arc<Expr>) -> Result<Expr, ParseError> {
        match *left{
            Expr::Uint(ua) => {
                match *right{
                    Expr::Uint(ub) => {
                        Ok(Expr::Uint(ua/ub))
                    }
                    _ => {
                        Expr::is_bad_primative(left, Operator::Div.into(), right)
                    }
                }
            }
            Expr::Int(ia) => {
                match *right{
                    Expr::Int(ib) => {
                        Ok(Expr::Int(ia/ib))
                    }
                    _ => {
                        Expr::is_bad_primative(left, Operator::Div.into(), right)
                    }
                }
            }
            Expr::Float(fa) => {
                match *right{
                    Expr::Float(fb) => {
                        Ok(Expr::Float(fa/fb))
                    }
                    _ => {
                        Expr::is_bad_primative(left, Operator::Div.into(), right)
                    }
                }
            }
            _ => {
                Ok(Expr::BinaryOp { left, op: Operator::Div.into(), right})
            }
        }
    }
    pub fn mult(left: Arc<Expr>, right: Arc<Expr>) -> Result<Expr, ParseError> {
        match *left{
            Expr::Uint(ua) => {
                match *right{
                    Expr::Uint(ub) => {
                        Ok(Expr::Uint(ua*ub))
                    }
                    _ => {
                        Expr::is_bad_primative(left, Operator::Mul.into(), right)
                    }
                }
            }
            Expr::Int(ia) => {
                match *right{
                    Expr::Int(ib) => {
                        Ok(Expr::Int(ia*ib))
                    }
                    _ => {
                        Expr::is_bad_primative(left, Operator::Mul.into(), right)
                    }
                }
            }
            Expr::Float(fa) => {
                match *right{
                    Expr::Float(fb) => {
                        Ok(Expr::Float(fa*fb))
                    }
                    _ => {
                        Expr::is_bad_primative(left, Operator::Mul.into(), right)
                    }
                }
            }
            _ => {
                Ok(Expr::BinaryOp { left, op: Operator::Mul.into(), right})
            }
        }
    }
    pub fn sub(left: Arc<Expr>, right: Arc<Expr>) -> Result<Expr, ParseError> {
        match *left{
            Expr::Uint(ua) => {
                match *right{
                    Expr::Uint(ub) => {
                        Ok(Expr::Uint(ua-ub))
                    }
                    _ => {
                        Expr::is_bad_primative(left, Operator::Sub.into(), right)
                    }
                }
            }
            Expr::Int(ia) => {
                match *right{
                    Expr::Int(ib) => {
                        Ok(Expr::Int(ia-ib))
                    }
                    _ => {
                        Expr::is_bad_primative(left, Operator::Sub.into(), right)
                    }
                }
            }
            Expr::Float(fa) => {
                match *right{
                    Expr::Float(fb) => {
                        Ok(Expr::Float(fa-fb))
                    }
                    _ => {
                        Expr::is_bad_primative(left, Operator::Sub.into(), right)
                    }
                }
            }
            _ => {
                Ok(Expr::BinaryOp { left, op: Operator::Sub.into(), right})
            }
        }
    }
    pub fn add(left: Arc<Expr>, right: Arc<Expr>) -> Result<Expr, ParseError> {
        match *left{
            Expr::Uint(ua) => {
                match *right{
                    Expr::Uint(ub) => {
                        Ok(Expr::Uint(ua+ub))
                    }
                    _ => {
                        Expr::is_bad_primative(left, Operator::Add.into(), right)
                    }
                }
            }
            Expr::Int(ia) => {
                match *right{
                    Expr::Int(ib) => {
                        Ok(Expr::Int(ia+ib))
                    }
                    _ => {
                        Expr::is_bad_primative(left, Operator::Add.into(), right)
                    }
                }
            }
            Expr::Float(fa) => {
                match *right{
                    Expr::Float(fb) => {
                        Ok(Expr::Float(fa+fb))
                    }
                    _ => {
                        Expr::is_bad_primative(left, Operator::Add.into(), right)
                    }
                }
            }
            Expr::String(ref sa) => {
                match *right{
                    Expr::String(ref sb) => {
                        Ok(Expr::String(format!("{}{}",*sa,*sb).into()))
                    }
                    _ => {
                        Expr::is_bad_primative(left, Operator::Add.into(), right)
                    }
                }
            }
            _ => {
                Ok(Expr::BinaryOp { left, op: Operator::Add.into(), right})
            }
        }
    }
}

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken(Token, String),
    BadToken(Token, String),
    BadExpress(String),
    UnexpectedEOF,
}

