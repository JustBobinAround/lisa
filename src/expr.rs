use std::collections::HashMap;
use std::sync::Arc;

use crate::type_def::Type;

use super::lexer::Operator;
use super::lexer::Token;
#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Type(Type),
    Bool(bool),
    Option(Option<Arc<Expr>>),
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
        else_branch: Arc<Expr>,
    },
    Function {
        param_sig: u64,
        return_sig: u64,
        block: Arc<Expr>,
    },
    Block(Vec<Arc<Expr>>),
    Param,
    MethodCall {
        name: Arc<String>,
        context: Arc<Expr>,
        param: Arc<Expr>,
        type_def: Option<Arc<Type>>
    }
}
type TypeEnv = HashMap<Arc<String>, Type>;
impl Expr {
    pub fn type_check(&self, env: &mut TypeEnv) -> Result<Type, String> {
        match self {
            Expr::Type(t) => Ok(t.clone()),
            Expr::Bool(_) => Ok(Type::Bool),
            Expr::Option(opt_expr) => {
                if let Some(expr) = opt_expr {
                    let expr_type = expr.type_check(env)?;
                    Ok(Type::Optional {
                        type_def: Arc::new(expr_type),
                    })
                } else {
                    Err("Option must contain an expression.".to_string())
                }
            }
            Expr::Int(_) => Ok(Type::Int),
            Expr::Uint(_) => Ok(Type::Uint),
            Expr::Char(_) => Ok(Type::Char),
            Expr::Float(_) => Ok(Type::Float),
            Expr::String(_) => Ok(Type::String),
            Expr::Array(arr) => {
                let mut element_type = None;
                for expr in arr {
                    let expr_type = expr.type_check(env)?;
                    match &element_type {
                        None => element_type = Some(expr_type),
                        Some(t) if *t != expr_type => {
                            return Err("Array elements must be of the same type.".to_string());
                        }
                        _ => {}
                    }
                }
                Ok(Type::Array {
                    array_type: Arc::new(element_type.unwrap_or(Type::None)),
                })
            }
            Expr::Struct { pairs } => {
                let mut struct_pairs = Vec::new();
                for (key, expr) in pairs {
                    let expr_type = expr.type_check(env)?;
                    struct_pairs.push(Arc::new(expr_type));
                }
                Ok(Type::Struct { pairs: struct_pairs })
            }
            Expr::Identifier(name) => {
                if let Some(t) = env.get(name) {
                    Ok(t.clone())
                } else {
                    Err(format!("Undefined identifier: {}", name))
                }
            }
            Expr::BinaryOp { left, op, right } => {
                let left_type = left.type_check(env)?;
                let right_type = right.type_check(env)?;
                // Type checking logic for binary operations
                match (op.as_ref(), left_type, right_type) {
                    // Add type checking rules for each operator
                    // Example for a simple integer addition operator
                    (_, Type::Int, Type::Int) => Ok(Type::Int),
                    _ => Err("Type mismatch in binary operation.".to_string()),
                }
            }
            Expr::UnaryOp { op, expr } => {
                let expr_type = expr.type_check(env)?;
                match (op.as_ref(), expr_type) {
                    _ => Err("Type mismatch in unary operation.".to_string()),
                }
            }
            Expr::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let condition_type = condition.type_check(env)?;
                if condition_type != Type::Bool {
                    return Err("Condition of 'if' must be a boolean.".to_string());
                }
                let then_type = then_branch.type_check(env)?;
                let else_type = else_branch.type_check(env)?;
                if then_type == else_type {
                    Ok(then_type)
                } else {
                    Err("Type mismatch in 'if' branches.".to_string())
                }
            }
            Expr::Function {
                param_sig,
                return_sig,
                block,
            } => {
                Ok(Type::Function {
                    param_type: Arc::new(Type::Generic), // Placeholder for now
                    return_type: Arc::new(Type::Generic), // Placeholder for now
                })
            }
            Expr::Block(exprs) => {
                let mut last_type = Type::None;
                for expr in exprs {
                    last_type = expr.type_check(env)?;
                }
                Ok(last_type)
            }
            Expr::Param => Err("Params should not be type-checked directly.".to_string()),
            Expr::MethodCall {
                name,
                context,
                param,
                type_def,
            } => {
                let context_type = context.type_check(env)?;
                if name.as_str() == "as" {
                    match **param {
                        Expr::Identifier(ref var_name) => {
                            env.insert(var_name.clone(), context_type.clone());
                            Ok(context_type)
                        }
                        _ => {
                            return Err(format!(
                                "Invalid parameter for 'as': expected identifier, found {:?}",
                                param
                            ));
                        }
                    }
                } else {
                    let param = context.type_check(env)?;
                    match type_def {
                        Some(t) => Ok(t.as_ref().clone()),
                        None => Err(format!(
                            "Method '{}' not found in context type '{:?}'",
                            name, context_type
                        )),
                    }
                }
            }        
        }
    }
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

