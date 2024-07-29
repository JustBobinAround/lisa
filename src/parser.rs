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

    fn add_key(&mut self, sig: u64, name:String, type_def: Type) -> Arc<Type>{
        let t = if let Some(t) = self.type_sig_map.get(&sig) {
            t.clone()
        } else {
            let t = Arc::new(type_def);
            self.type_sig_map.insert(sig, t.clone());
            t
        };
        self.type_name_map.insert(name, sig);
        t
    }

    fn advance(&mut self) {
        println!("{:?}",self.current_token);
        self.current_token = self.lexer.next_token();
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
        self.parse_main()
    }

    pub fn parse_main(&mut self) -> Result<Expr, ParseError> {
        let mut exprs = Vec::new();
        while self.current_token != Token::EOF {
            exprs.push(self.parse_expr()?);
        }
        Ok(Expr::Block(exprs))
    }

    fn parse_block(&mut self) -> Result<Expr, ParseError> {
        let mut exprs = Vec::new();
        self.expect(Token::LeftBrace)?;
        while self.current_token != Token::RightBrace && self.current_token != Token::EOF {
            exprs.push(self.parse_expr()?);
        }
        self.expect(Token::RightBrace)?;
        Ok(Expr::Block(exprs))
    }

    fn parse_expr(&mut self) -> Result<Expr, ParseError> {
        match &self.current_token {
            Token::LeftParen => {
                self.advance();
                match self.current_token {
                    Token::Identifier(_) => {
                        self.advance();
                        match self.current_token {
                            _ => {
            println!("hit15");
                                Err(ParseError::UnexpectedToken(self.current_token.clone()))
                            }
                        }
                    }
                    Token::RightParen => {
                        self.advance();
                        match self.current_token {
                            Token::Semicolon => {
                                Ok(Expr::None)
                            }
                            _ => {
            println!("hit16");
                                Err(ParseError::UnexpectedToken(self.current_token.clone()))
                            }
                        }
                    }
                    _ => {
                        self.parse_expr()
                    }
                }
            }
            Token::Identifier(name) => {
                let b = name.clone();
                self.advance();
                match self.current_token {
                    Token::Colon => {
                        self.advance();
                        match self.current_token {
                            Token::Colon => {
                                unimplemented!("This is for type extention");
                            }
                            _ => {
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
                        }
                    }
                    _ => {
            println!("hit18");
                        Err(ParseError::UnexpectedToken(self.current_token.clone()))
                    }
                }
            }

            _ => {
            println!("hit19");
                Err(ParseError::UnexpectedToken(self.current_token.clone()))
            }
        }
    }

    fn parse_type(&mut self, is_block: bool) -> Result<Type, ParseError> {
        match self.current_token {
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
        let ptr_t = self.add_key(sig,name.to_string(), t.clone());
        Ok(Expr::Type(Type::TypeDef { name, type_def: ptr_t }))
    }

    fn parse_if(&mut self) -> Result<Expr, ParseError> {
        if self.current_token == Token::If {
            self.advance();
            let condition = Box::new(self.parse_expr()?);
            let then_branch = Box::new(self.parse_block()?);
            let else_branch = if self.current_token == Token::Else {
                self.advance();
                Some(Box::new(self.parse_block()?))
            } else {
                None
            };
            Ok(Expr::If { condition, then_branch, else_branch })
        } else {
            self.parse_assignment()
        }
    }

    fn parse_assignment(&mut self) -> Result<Expr, ParseError> {
        match &self.current_token {
            Token::Int(i) => {}
            Token::Uint(i) => {}
            Token::Float(f) => {}
            _ => {}
        }
        if let Token::Identifier(name) = &self.current_token {
            let name = name.clone();
            self.advance();
            if self.current_token == Token::Operator(Operator::Eq) {
                self.advance();
                let expr = Box::new(self.parse_expr()?);
                //Ok(Expr::Assignment { name, expr })
            } else {
            }
            println!("hit21");
                Err(ParseError::UnexpectedToken(self.current_token.clone()))
        } else {
            self.parse_binary_op()
        }
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
            Token::None => {
                self.advance();
                Ok(Expr::None)
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
                Ok(Expr::String(s))
            }
            Token::Identifier(name) => {
                let name = name.clone();
                self.advance();
                if self.current_token == Token::LeftParen {
                    self.advance();
                    let mut args = Vec::new();
                    while self.current_token != Token::RightParen {
                        args.push(self.parse_expr()?);
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
            Token::LeftBrace => self.parse_block(),
            _ => {
            println!("hit24");
                Err(ParseError::UnexpectedToken(self.current_token.clone()))}
            ,
        }
    }
}

