use std::sync::Arc;

use crate::lexer::{Lexer, Op, Token};

type ParseResult<T> = Result<T, String>;

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Int(i64),
    Variable(String),
    BinOp(Op, Box<Expr>, Box<Expr>),
}

impl Expr {
    fn to_llvm_ir(&self, counter: &mut usize) -> (String, String) {
        match self {
            Expr::Int(value) => {
                let result_var = format!("%tmp{}", counter);
                *counter += 1;
                let ir = format!("{} = add i64 {}, 0", result_var, value); 
                (result_var, ir)
            },
            Expr::Variable(name) => {
                let result_var = format!("%{}", name);
                (result_var, String::new())
            },
            Expr::BinOp(op, lhs, rhs) => {
                let (lhs_var, lhs_ir) = lhs.to_llvm_ir(counter);
                let (rhs_var, rhs_ir) = rhs.to_llvm_ir(counter);
                
                let op_ir = match op {
                    Op::Add => "add",
                    Op::Sub => "sub",
                    Op::Mul => "mul",
                    Op::Div => "sdiv", 
                };

                let result_var = format!("%tmp{}", counter);
                *counter += 1;
                let bin_op_ir = format!("{} = {} i64 {}, {}", result_var, op_ir, lhs_var, rhs_var);

                let full_ir = format!(
                    "{}\n{}\n{}",
                    lhs_ir,
                    rhs_ir,
                    bin_op_ir
                );

                (result_var, full_ir)
            }
        }
    }
    

    pub fn to_anon_fn(&self, context: u64) -> String {
        //TODO type analysis

        let (result_var, llvm_ir) = self.to_llvm_ir(&mut 0);
        format!(r#"define i64 @anon_fn_{}() {{
    {}
    ret i64 {}
}}"#,
        context,
        llvm_ir,
        result_var
        )
    }

    pub fn to_anon_call(&self, context: u64) -> String{
        format!("%anon_call_{} = call i64 @anon_fn_{}()\n", context, context)
    }

    pub fn anon_debug(&self, context: u64) -> String {
        format!("call void @print_int(i64 %anon_call_{})\n", context)
    }
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


    pub fn next_token(&mut self) -> &Token {
        self.current_token = self.lexer.next_token();
        &self.current_token
    }

    fn expect_current(&mut self, token: Token) -> ParseResult<()> {
        if self.current_token() == &token {
            Ok(())
        } else {
            Err(format!("Error while parsing - Expected: {:#?} | Found {:#?}", token, self.current_token()))
        }
    }

    fn expect_next(&mut self, token: Token) -> ParseResult<()> {
        if self.next_token() == &token {
            Ok(())
        } else {
            Err(format!("Error while parsing - Expected: {:#?} | Found {:#?}", token, self.current_token()))
        }
    }

    pub fn parse_top_level_expr(&mut self) -> ParseResult<Expr> {
        let expr = self.parse_expression()?;
        self.expect_current(Token::SemiCol)?;
        self.next_token();
        Ok(expr)
    }

    fn parse_expression(&mut self) -> ParseResult<Expr> {
        let prior = match self.current_token() {
            Token::Int(i) => Expr::Int(*i),
            _ => return Err("Expected int while parsing expr".into()),
        };
        match self.next_token() {
            Token::Op(op) => {
                let op = op.clone();
                self.next_token();
                let rhs = self.parse_expression()?;
                Ok(Expr::BinOp(op, prior.into(), rhs.into()))
            }
            _ => Ok(prior)
        }
    }
}
