use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Arc;

use crate::type_def::Type;
use crate::expr::Expr;
use crate::parser::Parser;
use crate::lexer::{Lexer, Operator};

#[derive(Debug)]
pub struct Interpreter {
    variables: HashMap<String, Value>,
    type_sig_map: HashMap<u64, Arc<Type>>,
    type_name_map: HashMap<String, u64>,
}

#[derive(Clone, Debug)]
pub enum Value {
    None,
    Bool(bool),
    Int(i64),
    Uint(u64),
    Char(char),
    Float(f64),
    String(String),
    Type(Type),
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            variables: HashMap::new(),
            type_sig_map: HashMap::new(),
            type_name_map: HashMap::new(),
        }
    }

    pub fn interpret(&mut self, parser: &mut Parser) -> Result<(), String> {
        let expr = parser.parse().map_err(|e| format!("{:?}", e))?;
        self.evaluate(&expr)?;
        Ok(())
    }

    fn evaluate(&mut self, expr: &Expr) -> Result<Value, String> {
        match expr {
            Expr::Block(exprs) => {
                let mut last_value = Value::None;
                for expr in exprs {
                    last_value = self.evaluate(expr)?;
                }
                Ok(last_value)
            }
            Expr::UnaryOp { op, expr } => {
                let value = self.evaluate(expr)?;
                self.evaluate_unary_op(op, value)
            }
            Expr::BinaryOp { left, op, right } => {
                let left_value = self.evaluate(left)?;
                let right_value = self.evaluate(right)?;
                self.evaluate_binary_op(op, left_value, right_value)
            }
            Expr::If { condition, then_branch, else_branch } => {
                let condition_value = self.evaluate(condition)?;
                if self.is_true(&condition_value)? {
                    self.evaluate(then_branch)
                } else {
                    if let Some(else_branch) = else_branch {
                        self.evaluate(else_branch)
                    } else {
                        Ok(Value::None)
                    }
                }
            }
            Expr::Function { param_sig, return_sig, block } => {
                unimplemented!("Function evaluation is not yet implemented")
            }
            Expr::Type(t) => Ok(Value::Type(t.clone())),
            Expr::Identifier(ref name) => {
                if let Some(value) = self.variables.get(&**name) {
                    Ok(value.clone())
                } else {
                    Err(format!("Undefined variable: {}", name))
                }
            }
            Expr::Bool(b) => Ok(Value::Bool(*b)),
            Expr::Int(i) => Ok(Value::Int(*i)),
            Expr::Uint(u) => Ok(Value::Uint(*u)),
            Expr::Char(c) => Ok(Value::Char(*c)),
            Expr::Float(f) => Ok(Value::Float(*f)),
            Expr::String(s) => Ok(Value::String(s.clone().deref().clone())),
            Expr::Assignment { prior_expr, name } => {
                let value = self.evaluate(prior_expr)?;
                self.variables.insert(name.clone().deref().clone(), value.clone());
                Ok(value)
            }
            Expr::SharedAssignment { prior_expr, name } => {
                let value = self.evaluate(prior_expr)?;
                self.variables.insert(name.clone().deref().clone(), value.clone());
                Ok(value)
            }
            Expr::FunctionCall { prior_expr, name, arg } => {
                unimplemented!("Function call evaluation is not yet implemented")
            }
            _ => {
                unimplemented!("engine still in progress sorry")
            }
        }
    }

    fn evaluate_unary_op(&self, op: &Operator, value: Value) -> Result<Value, String> {
        match op {
            Operator::Not => {
                if let Value::Bool(b) = value {
                    Ok(Value::Bool(!b))
                } else {
                    Err(format!("Invalid type for unary operator Not: {:?}", value))
                }
            }
            _ => {
                unimplemented!("engine still in progress sorry")
            }
        }
    }

    fn evaluate_binary_op(&self, op: &Operator, left: Value, right: Value) -> Result<Value, String> {
        match op {
            Operator::Add => {
                match (&left, &right) {
                    (Value::Int(l), Value::Int(r)) => Ok(Value::Int(l + r)),
                    (Value::Float(l), Value::Float(r)) => Ok(Value::Float(l + r)),
                    (Value::String(l), Value::String(r)) => Ok(Value::String(l.to_owned() + r)),
                    _ => Err(format!("Invalid types for binary operator Add: {:?} and {:?}", left, right)),
                }
            }
            Operator::Sub => {
                match (&left, &right) {
                    (Value::Int(l), Value::Int(r)) => Ok(Value::Int(l - r)),
                    (Value::Float(l), Value::Float(r)) => Ok(Value::Float(l - r)),
                    _ => Err(format!("Invalid types for binary operator Sub: {:?} and {:?}", left, right)),
                }
            }
            Operator::Mul => {
                match (&left, &right) {
                    (Value::Int(l), Value::Int(r)) => Ok(Value::Int(l * r)),
                    (Value::Float(l), Value::Float(r)) => Ok(Value::Float(l * r)),
                    _ => Err(format!("Invalid types for binary operator Mul: {:?} and {:?}", left, right)),
                }
            }
            Operator::Div => {
                match (&left, &right) {
                    (Value::Int(l), Value::Int(r)) => {
                        if *r == 0 {
                            Err("Division by zero".to_string())
                        } else {
                            Ok(Value::Int(l / r))
                        }
                    }
                    (Value::Float(l), Value::Float(r)) => {
                        if *r == 0.0 {
                            Err("Division by zero".to_string())
                        } else {
                            Ok(Value::Float(l / r))
                        }
                    }
                    _ => Err(format!("Invalid types for binary operator Div: {:?} and {:?}", left, right)),
                }
            }
            _ => Err(format!("Unsupported operator: {:?}", op)),
        }
    }

    fn is_true(&self, value: &Value) -> Result<bool, String> {
        match value {
            Value::Bool(b) => Ok(*b),
            _ => Err(format!("Expected boolean, found: {:?}", value)),
        }
    }
}
