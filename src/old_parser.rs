use std::collections::{HashMap, HashSet};
use std::ops::Deref;
use std::sync::Arc;

use crate::type_def::Type;

use super::lexer::{Token, Operator};
use super::Lexer;
use super::expr::{Expr, ParseError};

pub struct Parser<'a> {
    pub lexer: Lexer<'a>,
    current_token: Token,
    type_sig_map: HashMap<u64, Arc<Type>>,
    type_name_map: HashMap<String, u64>
}

impl<'a> Parser<'a> {
    pub fn new(mut lexer: Lexer<'a>) -> Self {
        let current_token = lexer.next_token();
        Parser { lexer, current_token, type_sig_map: HashMap::new(), type_name_map: HashMap::new() }
    }

    fn add_key(&mut self, sig: u64, name:Option<String>, type_def: Type) -> Arc<Type>{
        let t = if let Some(t) = self.type_sig_map.get(&sig) {
            t.clone()
        } else {
            let t = Arc::new(type_def);
            self.type_sig_map.insert(sig, t.clone());
            t
        };
        if let Some(name) = name {
            self.type_name_map.insert(name, sig);
        }
        t
    }

    fn advance(&mut self) {
        self.current_token = self.lexer.next_token();
        println!("advanced to: {:?}",self.current_token);
    }

    fn expect(&mut self, expected: Token) -> Result<(), ParseError> {
        if self.current_token == expected {
            self.advance();
            Ok(())
        } else {
            println!("hit14");
            Err(ParseError::UnexpectedToken(self.current_token.clone()))
        }
    }

    pub fn parse(&mut self) -> Result<Expr, ParseError> {
        self.parse_block(true)
    }

    pub fn parse_block(&mut self, is_main: bool) -> Result<Expr, ParseError> {
        if !is_main {
            self.expect(Token::LeftBracket)?;
        }
        let mut exprs = Vec::new();
        let mut prior_expr = None;
        while self.current_token != Token::EOF && self.current_token!= Token::RightBracket {
            println!("expra");
            //println!(">>>>>>>{:?}", self.current_token);
            let expr = self.parse_expr(prior_expr)?;
            exprs.push(expr.clone());
            println!(">>>>>>>{:?}", &expr);
            prior_expr = Some(expr);
            /*
            match self.current_token {
                _ => {
                    exprs.push(expr)
                }
            }
            */
        }
        if !is_main {
            self.expect(Token::RightBracket)?;
        } else {
            self.expect(Token::EOF)?;
        }
        Ok(Expr::Block(exprs))
    }

    fn parse_expr(&mut self, prior_expr: Option<Expr>) -> Result<Expr, ParseError> {
        println!("hitexpr: {:?}", self.current_token);
        let expr = match self.current_token {
            Token::Period => {
                println!("function call hit");
                self.advance();
                if let Some(prior_expr) = prior_expr {
                    self.parse_fn_call(prior_expr)
                } else {
                    return Err(ParseError::UnexpectedToken(self.current_token.clone()));
                }
            }
            Token::Semicolon => {
                self.advance();
                if let Some(prior_expr) = prior_expr {
                    Ok(prior_expr)
                } else {
                    return Err(ParseError::UnexpectedToken(self.current_token.clone()));
                }
            }
            Token::Operator(ref op) => {
                let bop = op.clone();
                self.advance();
                match bop.as_ref() {
                    Operator::Not => {
                        let expr = self.parse_expr(None)?;
                        Ok(Expr::UnaryOp { op: bop, expr: expr.into()})
                    }
                    //maybe mut should be uniary casting op?
                    //
                    _ => {
                        let right = self.parse_expr(prior_expr.clone())?;
                        if let Some(prior_expr) = prior_expr {
                            Ok(Expr::BinaryOp { left: prior_expr.into(), op: bop, right: right.into() })
                        } else {
                            return Err(ParseError::UnexpectedToken(self.current_token.clone()));
                        }
                    }
                }
            }
            Token::LeftParen => {
                self.advance();
                let mut prior_expr = None;
                while self.current_token!=Token::RightParen && self.current_token!= Token::EOF {
                    prior_expr = Some(self.parse_expr(prior_expr)?);
                }
                self.expect(Token::RightParen)?;

                if let Some(prior_expr) = prior_expr {
                    Ok(prior_expr)
                } else {
                    return Err(ParseError::UnexpectedToken(self.current_token.clone()));
                }
            }
            Token::TNone => {
                self.advance();
                Ok(Expr::Type(Type::None))
            },
            Token::TBool => {
                self.advance();
                Ok(Expr::Type(Type::Bool))
            },
            Token::TInt => {
                self.advance();
                Ok(Expr::Type(Type::Int))
            },
            Token::TUint => {
                self.advance();
                Ok(Expr::Type(Type::Uint))
            },
            Token::TChar => {
                self.advance();
                Ok(Expr::Type(Type::Char))
            },
            Token::TFloat => {
                self.advance();
                Ok(Expr::Type(Type::Float))
            },
            Token::TString => {
                self.advance();
                Ok(Expr::Type(Type::String))
            },
            Token::FnTypes=> {
                self.parse_fn()
            },
            Token::Bool(b) => {
                self.advance();
                Ok(Expr::Bool(b.to_owned()))
            },
            Token::Int(i) => {
                self.advance();
                Ok(Expr::Int(i.to_owned()))
            },
            Token::Uint(u) => {
                self.advance();
                Ok(Expr::Uint(u.to_owned()))
            },
            Token::Char(c) => {
                self.advance();
                Ok(Expr::Char(c.to_owned()))
            },
            Token::Float(f) => {
                self.advance();
                Ok(Expr::Float(f.to_owned()))
            },
            Token::String(ref s) => {
                let b = s.clone();
                self.advance();
                Ok(Expr::String(b))
            },
            Token::Identifier(ref name) => {
                let b = name.clone();
                self.advance();
                match self.current_token {
                    Token::Colon => {
                        self.advance();
                        let t = self.parse_type_def(b)?;
                        match self.current_token {
                            Token::Semicolon => {
                                self.advance();
                                Ok(t)
                            }
                            _ => {
                                println!("hit17");
                                Err(ParseError::UnexpectedToken(self.current_token.clone()))
                            }
                        }
                    }
                    _ => {
                        Ok(Expr::Identifier(b))
                    }
                }
            }
            _ => {
                println!("hit19");
                Err(ParseError::UnexpectedToken(self.current_token.clone()))
            }
        };
        println!("hitexpr: {:?}", &expr);
        expr
    }
    fn parse_mut_assign(&mut self)  -> Result<Expr, ParseError> {
        unimplemented!("don't know how I should do this yet");
    }

    fn parse_fn_call(&mut self, prior_expr: Expr) -> Result<Expr, ParseError> {
        match self.current_token {
            Token::Identifier(ref name) => {
                let b = name.clone();
                self.advance();
                self.expect(Token::LeftParen)?;
                println!("hita");
                let expr = if &*b=="as" {
                    self.parse_assignment(false, prior_expr.clone())?
                }else if &*b=="as_shared" {
                    self.parse_assignment(true, prior_expr.clone())?
                } else {
                    let expr = self.parse_expr(Some(prior_expr.clone()))?;
                    Expr::FunctionCall{prior_expr: prior_expr.clone().into(), name: b, arg: Box::new(expr)}
                };
                self.expect(Token::RightParen)?;
                if self.current_token == Token::Period {
                    match expr {
                        Expr::Assignment { prior_expr, expr, name } => {
                            self.advance();
                            self.parse_fn_call(*expr)
                        }
                        _ => {
                            self.advance();
                            println!("{:?}", expr);
                            self.parse_fn_call(expr)
                        }
                    }
                } else if self.current_token == Token::Semicolon {
                    println!("hit semi in fn call");
                    self.advance();
                    Ok(expr)
                } else {
                    Err(ParseError::UnexpectedToken(self.current_token.clone()))
                }
            }
            _ => {
                Err(ParseError::UnexpectedToken(self.current_token.clone()))
            }
        }
    }

    fn parse_fn(&mut self) -> Result<Expr, ParseError> {
        self.advance();
        let param_sig = match self.current_token {
            Token::Identifier(ref name) => {
                let b = name.clone();
                self.advance();
                if let Some(t) = self.type_name_map.get(&**b){
                    *t
                } else {
                    println!("hit123");
                    return Err(ParseError::UnexpectedToken(self.current_token.clone()));
                }
            }
            _ => {
                let mut param_type = self.parse_type(false)?;
                param_type.get_sig()
            }
        };
        println!("hit arrow{:?}", self.current_token);
        self.expect(Token::Arrow)?;
        let return_sig = match self.current_token {
            Token::Identifier(ref name) => {
                let name = name.clone();
                self.advance();
                if let Some(t) = self.type_name_map.get(&**name){
                    println!("hit type");
                    *t
                } else {
                    return Err(ParseError::UnexpectedToken(self.current_token.clone()));
                }
            }
            _ => {
                let mut return_type = self.parse_type(false)?;
                return_type.get_sig()
            }
        };
        self.expect(Token::FnTypes)?;

        let block = self.parse_block(false)?;
        match self.current_token {
            Token::Semicolon => {
                Ok(Expr::Function { param_sig, return_sig, block: Box::new(block) })
            },
            Token::Period => {
                Ok(Expr::Function { param_sig, return_sig, block: Box::new(block) })
            },
            Token::Colon => {
                unimplemented!("type extention");
            },
            Token::LeftParen=> {
                unimplemented!("function call");
            },
            _ => {
                return Err(ParseError::UnexpectedToken(self.current_token.clone()));
            }

        }
    }

    fn parse_type(&mut self, is_block: bool) -> Result<Type, ParseError> {
        match self.current_token {
            Token::Generic=> {
                self.advance();
                Ok(Type::Generic)
            }
            Token::TNone => {
                self.advance();
                Ok(Type::None)
            },
            Token::TBool => {
                self.advance();
                Ok(Type::Bool)
            },
            Token::TInt => {
                self.advance();
                Ok(Type::Int)
            },
            Token::TUint => {
                self.advance();
                Ok(Type::Uint)
            },
            Token::TChar => {
                self.advance();
                Ok(Type::Char)
            },
            Token::TFloat => {
                self.advance();
                Ok(Type::Float)
            },
            Token::TString => {
                self.advance();
                Ok(Type::String)
            },
            Token::LeftBrace => {
                println!("hit7");
                self.advance();
                let t = self.parse_type(false)?;
                match self.current_token {
                    Token::RightBrace => {
                        self.advance();
                        Ok(Type::Array { array_type: Box::new(t) })
                    }
                    _ => {
                        println!("hit7");
                        Err(ParseError::UnexpectedToken(self.current_token.clone()))
                    }
                }
            }
            Token::LeftBracket => {
                self.advance();
                let mut types = Vec::new();
                let mut set = HashSet::new();

                while self.current_token!=Token::RightBracket && self.current_token!=Token::EOF{
                    let t = self.parse_type(true)?;
                    match t {
                        Type::TypeDef { ref name, type_def: _ } => {
                            if set.insert(name.clone()) {
                                types.push(t);
                            } else {
                        println!("hit6");
                                return Err(ParseError::UnexpectedToken(self.current_token.clone()));
                            }
                        }
                        _ => {
                        println!("hit5");
                            return Err(ParseError::UnexpectedToken(self.current_token.clone()));
                        }
                    }
                }

                if self.current_token!=Token::RightBracket {
                        println!("hit4");
                    Err(ParseError::UnexpectedToken(self.current_token.clone()))
                } else {
                    self.advance();
                    Ok(Type::Struct { pairs: types})
                }
            }
            Token::Identifier(ref name) => {
                let name = name.clone();
                self.advance();
                match self.current_token {
                    Token::Colon => {
                        self.advance();
                        let type_def = self.parse_type(false)?;
                        match self.current_token {
                            Token::Semicolon => {
                                self.advance();
                                Ok(Type::TypeDef { name, type_def: Arc::new(type_def) })
                            }
                            Token::Comma => {
                                self.advance();
                                Ok(Type::TypeDef { name, type_def: Arc::new(type_def) })
                            }
                            Token::RightBracket => {
                                if is_block {
                                    Ok(Type::TypeDef { name, type_def: Arc::new(type_def) })
                                } else {
            println!("hit20");
                                    Err(ParseError::UnexpectedToken(self.current_token.clone()))
                                }
                            }
                            _ => {
                        println!("hit2");
                                Err(ParseError::UnexpectedToken(self.current_token.clone()))
                            }
                        }
                    }
                    Token::Semicolon => {
                        if let Some(sig) = self.type_name_map.get(&*name) {
                            if let Some(t) = self.type_sig_map.get(&sig) {
                                let b = t.clone()
                                    .deref()
                                    .clone(); // I hate rust sometimes
                                Ok(b)
                            } else {
                            println!("hit133");
                            Err(ParseError::UnexpectedToken(self.current_token.clone()))
                            }
                        } else {
                            println!("hit13");
                            Err(ParseError::UnexpectedToken(self.current_token.clone()))
                        }
                    }
                    _ => {
                        println!("hit1");
                        Err(ParseError::UnexpectedToken(self.current_token.clone()))
                    }
                }
            }

            _ => {
                println!("hit3");
                Err(ParseError::UnexpectedToken(self.current_token.clone()))
            }
        }
    }

    fn parse_type_def(&mut self, name: Arc<String>) -> Result<Expr, ParseError> {
        let mut t = self.parse_type(false)?;
        let sig = t.get_sig();
        println!(">>>{:?}", self.type_sig_map);
        let ptr_t = self.add_key(sig,Some(name.to_string()), t.clone());
        Ok(Expr::Type(Type::TypeDef { name, type_def: ptr_t }))
    }

    fn parse_if(&mut self) -> Result<Expr, ParseError> {
        self.advance();
        let condition = Box::new(self.parse_expr(None)?);
        let then_branch = Box::new(self.parse_block(false)?);
        let else_branch = if self.current_token == Token::Else {
            self.advance();
            Some(Box::new(self.parse_block(false)?))
        } else {
            None
        };
        Ok(Expr::If { condition, then_branch, else_branch })
    }

    fn parse_extention(&mut self, prior_expr: Expr, sig: Option<u64>) -> Result<Expr, ParseError> {
        let t = if let Some(sig) = sig {
            sig
        } else {
            let mut t = self.parse_type(false)?;
            t.get_sig()
        };
        self.expect(Token::Colon)?;
        self.expect(Token::Colon)?;
        match self.current_token {
            Token::Identifier(ref name) => {
                let name = name.clone();
                println!("hit type extention for generics");
                self.advance();
                let prior_expr = match prior_expr {
                    Expr::Function { param_sig, return_sig, block } => {
                        if return_sig==1000000001 && param_sig==1000000001 {
                            Expr::Function { param_sig: t, return_sig: t, block: block.clone() }
                        } else if param_sig==1000000001 {
                            Expr::Function { param_sig: t , return_sig, block: block.clone() }
                        } else if return_sig==1000000001 {
                            Expr::Function { param_sig , return_sig: t, block: block.clone() }
                        } else {
                            Expr::Function { param_sig , return_sig , block: block.clone() }
                        }
                    }
                    _ => {
                        println!("{:?} hit this error ", prior_expr);
                        return Err(ParseError::UnexpectedToken(self.current_token.clone()));
                    }
                };
                Ok(Expr::TypeExtend { type_sig: t, name: name.clone(), expr: prior_expr.into() })
            }
            _=> {
                Err(ParseError::UnexpectedToken(self.current_token.clone()))
            }
        }
    }

    fn parse_assignment(&mut self, is_shared: bool, prior_expr: Expr) -> Result<Expr, ParseError> {
        println!("hitc");
        let prior_expr = match prior_expr {
            Expr::Assignment { ref prior_expr, ref expr, ref name } => {
                *prior_expr.clone()
            }
            _ => {
                prior_expr.clone()
            } 
        };
        match self.current_token {
            Token::Identifier(ref name) => {
                let name = name.clone();
                self.advance();
                if let Some(t) = self.type_name_map.get(&*name) {
                    println!("hit type match");
                    let expr = self.parse_extention(prior_expr.clone(), Some(*t))?;
                    if is_shared {
                        Ok(Expr::SharedAssignment { prior_expr: prior_expr.into(), expr: expr.into(), name })
                    } else {
                        Ok(Expr::Assignment { prior_expr: prior_expr.into(), expr: expr.into(), name })
                    }
                } else {
                    if is_shared {
                        Ok(Expr::SharedAssignment { prior_expr: prior_expr.clone().into(), expr: prior_expr.into(), name })
                    } else {
                        Ok(Expr::Assignment { prior_expr: prior_expr.clone().into(), expr: prior_expr.into(), name })
                    }
                }
            }

            _=> {
                println!("hit default type match");
                self.parse_extention(prior_expr, None)
            }
        }
        /*
        if let Some(t) = self.type_name_map.get(&*name) {
            if let Some(t) = self.type_sig_map.get(&*t) {
                let t = t.clone()
                    .deref()
                    .clone();
                self.expect(Token::Colon)?;
                self.expect(Token::Colon)?;
                let name = match self.current_token {
                    Token::Identifier(ref name) => {
                        let name = name.clone();
                        self.advance();
                        name
                    }

                    _=> {
                        return Err(ParseError::UnexpectedToken(self.current_token.clone()));
                    }
                };
                Ok(Expr::TypeExtend {
                    type_def: t,
                    name,
                    expr: prior_expr.into()
                })
            } else {
                return Err(ParseError::UnexpectedToken(self.current_token.clone()));
            }
        } else {
        */
        //}
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
            Token::TNone => {
                self.advance();
                Ok(Expr::Type(Type::None))
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
                Ok(Expr::String(s.into()))
            }
            Token::Identifier(_) => {
                self.advance();
                if self.current_token == Token::LeftParen {
                    self.advance();
                    let mut args = Vec::new();
                    while self.current_token != Token::RightParen {
                        args.push(self.parse_expr(None)?);
                        if self.current_token == Token::Comma {
                            self.advance();
                        }
                    }
                    self.expect(Token::RightParen)?;
                    //Ok(Expr::FunctionCall { name, args })

            println!("hit22");
                Err(ParseError::UnexpectedToken(self.current_token.clone()))
                } else {
                    //Ok(Expr::Identifier(name))
                    println!("hit23");
                    Err(ParseError::UnexpectedToken(self.current_token.clone()))
                }
            }
            Token::LeftBrace => self.parse_block(false),
            _ => {
                println!("hit24");
                Err(ParseError::UnexpectedToken(self.current_token.clone()))}
            ,
        }
    }
}

