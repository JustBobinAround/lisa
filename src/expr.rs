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
    FunctionCall {
        prior_expr: Arc<Expr>,
        name: Arc<String>,
        arg: Arc<Expr>,
    },
    SharedAssignment {
        prior_expr: Arc<Expr>,
        expr: Arc<Expr>,
        name: Arc<String>,
    },
    Assignment {
        prior_expr: Arc<Expr>,
        expr: Arc<Expr>,
        name: Arc<String>,
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
    TypeExtend {
        type_sig: u64,
        name: Arc<String>,
        expr: Arc<Expr>        
    },
    Block(Vec<Arc<Expr>>),
}

impl Expr {
    pub fn neq(left: Arc<Expr>, right: Arc<Expr>) -> Expr {
        match *left{
            Expr::Bool(ba) => {
                match *right{
                    Expr::Bool(bb) => {
                        Expr::Bool(ba!=bb)
                    }
                    _ => {
                        Expr::BinaryOp { left, op: Operator::Neq.into(), right}
                    }
                }
            }
            Expr::Uint(ua) => {
                match *right{
                    Expr::Uint(ub) => {
                        Expr::Bool(ua!=ub)
                    }
                    _ => {
                        Expr::BinaryOp { left, op: Operator::Neq.into(), right}
                    }
                }
            }
            Expr::Int(ia) => {
                match *right{
                    Expr::Int(ib) => {
                        Expr::Bool(ia!=ib)
                    }
                    _ => {
                        Expr::BinaryOp { left, op: Operator::Neq.into(), right}
                    }
                }
            }
            Expr::Float(fa) => {
                match *right{
                    Expr::Float(fb) => {
                        Expr::Bool(fa!=fb)
                    }
                    _ => {
                        Expr::BinaryOp { left, op: Operator::Neq.into(), right}
                    }
                }
            }
            _ => {
                if Arc::<Expr>::as_ptr(&left)==Arc::<Expr>::as_ptr(&right) {
                    Expr::Bool(false)
                } else {
                    Expr::BinaryOp { left, op: Operator::Neq.into(), right}
                }
            }
        }
    }
    pub fn eq(left: Arc<Expr>, right: Arc<Expr>) -> Expr {
        match *left{
            Expr::Bool(ba) => {
                match *right{
                    Expr::Bool(bb) => {
                        Expr::Bool(ba==bb)
                    }
                    _ => {
                        Expr::BinaryOp { left, op: Operator::Eq.into(), right}
                    }
                }
            }
            Expr::Uint(ua) => {
                match *right{
                    Expr::Uint(ub) => {
                        Expr::Bool(ua==ub)
                    }
                    _ => {
                        Expr::BinaryOp { left, op: Operator::Eq.into(), right}
                    }
                }
            }
            Expr::Int(ia) => {
                match *right{
                    Expr::Int(ib) => {
                        Expr::Bool(ia==ib)
                    }
                    _ => {
                        Expr::BinaryOp { left, op: Operator::Eq.into(), right}
                    }
                }
            }
            Expr::Float(fa) => {
                match *right{
                    Expr::Float(fb) => {
                        Expr::Bool(fa==fb)
                    }
                    _ => {
                        Expr::BinaryOp { left, op: Operator::Eq.into(), right}
                    }
                }
            }
            _ => {
                if Arc::<Expr>::as_ptr(&left)==Arc::<Expr>::as_ptr(&right) {
                    Expr::Bool(true)
                } else {
                    Expr::BinaryOp { left, op: Operator::Eq.into(), right}
                }
            }
        }
    }
    pub fn modd(left: Arc<Expr>, right: Arc<Expr>) -> Expr {
        match *left{
            Expr::Uint(ua) => {
                match *right{
                    Expr::Uint(ub) => {
                        Expr::Uint(ua%ub)
                    }
                    _ => {
                        Expr::BinaryOp { left, op: Operator::Mod.into(), right}
                    }
                }
            }
            Expr::Int(ia) => {
                match *right{
                    Expr::Int(ib) => {
                        Expr::Int(ia%ib)
                    }
                    _ => {
                        Expr::BinaryOp { left, op: Operator::Mod.into(), right}
                    }
                }
            }
            Expr::Float(fa) => {
                match *right{
                    Expr::Float(fb) => {
                        Expr::Float(fa%fb)
                    }
                    _ => {
                        Expr::BinaryOp { left, op: Operator::Mod.into(), right}
                    }
                }
            }
            _ => {
                Expr::BinaryOp { left, op: Operator::Mod.into(), right}
            }
        }
    }
    pub fn div(left: Arc<Expr>, right: Arc<Expr>) -> Expr {
        match *left{
            Expr::Uint(ua) => {
                match *right{
                    Expr::Uint(ub) => {
                        Expr::Uint(ua/ub)
                    }
                    _ => {
                        Expr::BinaryOp { left, op: Operator::Div.into(), right}
                    }
                }
            }
            Expr::Int(ia) => {
                match *right{
                    Expr::Int(ib) => {
                        Expr::Int(ia/ib)
                    }
                    _ => {
                        Expr::BinaryOp { left, op: Operator::Div.into(), right}
                    }
                }
            }
            Expr::Float(fa) => {
                match *right{
                    Expr::Float(fb) => {
                        Expr::Float(fa/fb)
                    }
                    _ => {
                        Expr::BinaryOp { left, op: Operator::Div.into(), right}
                    }
                }
            }
            _ => {
                Expr::BinaryOp { left, op: Operator::Div.into(), right}
            }
        }
    }
    pub fn mult(left: Arc<Expr>, right: Arc<Expr>) -> Expr {
        match *left{
            Expr::Uint(ua) => {
                match *right{
                    Expr::Uint(ub) => {
                        Expr::Uint(ua*ub)
                    }
                    _ => {
                        Expr::BinaryOp { left, op: Operator::Mul.into(), right}
                    }
                }
            }
            Expr::Int(ia) => {
                match *right{
                    Expr::Int(ib) => {
                        Expr::Int(ia*ib)
                    }
                    _ => {
                        Expr::BinaryOp { left, op: Operator::Mul.into(), right}
                    }
                }
            }
            Expr::Float(fa) => {
                match *right{
                    Expr::Float(fb) => {
                        Expr::Float(fa*fb)
                    }
                    _ => {
                        Expr::BinaryOp { left, op: Operator::Mul.into(), right}
                    }
                }
            }
            _ => {
                Expr::BinaryOp { left, op: Operator::Mul.into(), right}
            }
        }
    }
    pub fn sub(left: Arc<Expr>, right: Arc<Expr>) -> Expr {
        match *left{
            Expr::Uint(ua) => {
                match *right{
                    Expr::Uint(ub) => {
                        Expr::Uint(ua-ub)
                    }
                    _ => {
                        Expr::BinaryOp { left, op: Operator::Sub.into(), right}
                    }
                }
            }
            Expr::Int(ia) => {
                match *right{
                    Expr::Int(ib) => {
                        Expr::Int(ia-ib)
                    }
                    _ => {
                        Expr::BinaryOp { left, op: Operator::Sub.into(), right}
                    }
                }
            }
            Expr::Float(fa) => {
                match *right{
                    Expr::Float(fb) => {
                        Expr::Float(fa-fb)
                    }
                    _ => {
                        Expr::BinaryOp { left, op: Operator::Sub.into(), right}
                    }
                }
            }
            _ => {
                Expr::BinaryOp { left, op: Operator::Sub.into(), right}
            }
        }
    }
    pub fn add(left: Arc<Expr>, right: Arc<Expr>) -> Expr {
        match *left{
            Expr::Uint(ua) => {
                match *right{
                    Expr::Uint(ub) => {
                        Expr::Uint(ua+ub)
                    }
                    _ => {
                        Expr::BinaryOp { left, op: Operator::Add.into(), right}
                    }
                }
            }
            Expr::Int(ia) => {
                match *right{
                    Expr::Int(ib) => {
                        Expr::Int(ia+ib)
                    }
                    _ => {
                        Expr::BinaryOp { left, op: Operator::Add.into(), right}
                    }
                }
            }
            Expr::Float(fa) => {
                match *right{
                    Expr::Float(fb) => {
                        Expr::Float(fa+fb)
                    }
                    _ => {
                        Expr::BinaryOp { left, op: Operator::Add.into(), right}
                    }
                }
            }
            Expr::String(ref sa) => {
                match *right{
                    Expr::String(ref sb) => {
                        Expr::String(format!("{}{}",*sa,*sb).into())
                    }
                    _ => {
                        Expr::BinaryOp { left, op: Operator::Add.into(), right}
                    }
                }
            }
            _ => {
                Expr::BinaryOp { left, op: Operator::Add.into(), right}
            }
        }
    }
}

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken(Token, String),
    BadToken(Token, String),
    UnexpectedEOF,
}

