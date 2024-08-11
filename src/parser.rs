use std::collections::{HashMap, HashSet};
use std::ops::Deref;
use std::sync::Arc;

use crate::{expr, type_def};
use crate::type_def::Type;

use super::lexer::{Token, Operator};
use super::Lexer;
use super::expr::{Expr, ParseError};

pub struct Parser<'a> {
    pub lexer: Lexer<'a>,
    current_token: Token,
}

pub struct TypeMap {
    pub name_map: HashMap<String, Arc<Type>>,
    pub sig_map: HashMap<u64, Arc<Type>>
}
impl TypeMap {
    pub fn new() -> TypeMap {
        TypeMap { name_map: HashMap::new(), sig_map: HashMap::new() }
    }

    pub fn insert(&mut self, name: String, type_def: Arc<Type>) -> Result<(), ParseError> {
        if self.name_map.contains_key(&name) {
            return Err(ParseError::BadExpress("Can't overwrite existing types".to_string()))
        } else {
            self.name_map.insert(name, type_def.clone());
        }
        let sig = type_def.get_sig();
        if !self.sig_map.contains_key(&sig) {
            self.sig_map.insert(type_def.get_sig(), type_def);
        }
        Ok(())
    }

}


impl<'a> Parser<'a> {
    pub fn new(mut lexer: Lexer<'a>) -> Self {
        let current_token = lexer.next_token();
        Parser { lexer, current_token}
    }

    fn advance(&mut self) {
        self.current_token = self.lexer.next_token();
    }

    fn expect(&mut self, expected: Token, msg: &'static str) -> Result<(), ParseError> {
        if self.current_token == expected {
            self.advance();
            Ok(())
        } else {
            Err(ParseError::UnexpectedToken(self.current_token.clone(), msg.to_string()))
        }
    }

    pub fn parse(&mut self) -> Result<Expr, ParseError> {
        let mut variables = HashMap::new();
        let mut types = TypeMap::new();
        self.parse_block(true, &mut variables, &mut types)
    }

    pub fn parse_block(
        &mut self, 
        is_main: bool,
        mut variables: &mut HashMap<String, Arc<Expr>>, 
        mut types: &mut TypeMap
    ) -> Result<Expr, ParseError> {
        if !is_main {
            self.expect(Token::LeftBracket, "Expected starting bracket at start of block")?;
        }
        let mut exprs = Vec::new();
        let mut prior_expr = None;
        while self.current_token != Token::EOF {
            let expr = self.parse_expr(&mut variables, &mut types)?;
            println!("{:?}", self.current_token);
            exprs.push(expr.clone());
            prior_expr = Some(expr);
            match self.current_token {
                Token::RightBracket => {
                    break;
                }
                Token::Semicolon => {
                    self.advance();
                }
                _ => {
                    return Err(ParseError::BadToken(self.current_token.clone(), "Expected a semicolon or a closing bracket to block".to_string()))
                }
            }
        }

        println!("Declared variables: {:?}", variables);
        if !is_main {
            self.expect(Token::RightBracket, "Expected closing bracket at end of block")?;
        } else {
            self.expect(Token::EOF, "Expected EOF token at end of main block")?;
        }
        Ok(Expr::Block(exprs))
    }

    fn parse_expr(
        &mut self, 
        mut variables: &mut HashMap<String, Arc<Expr>>, 
        mut types: &mut TypeMap
    ) -> Result<Arc<Expr>, ParseError> {
        let left_expr: Arc<Expr> = match self.current_token {
            Token::Param => {
                self.advance();
                Expr::Param.into()
            }
            Token::Operator(ref op) => {
                let op = op.clone();
                self.advance();
                self.parse_unary(&mut variables, &mut types, op)?.into()
            }
            Token::TNone => {
                self.advance();
                Expr::Option(None).into()
            }
            Token::TSome => {
                self.advance();
                self.expect(Token::LeftParen, "Expected opening paren and expression after optional")?;
                let expr = self.parse_expr(variables, types)?;
                self.expect(Token::RightParen, "Expected opening paren and expression after optional")?;
                Expr::Option(Some(expr.into())).into()
            }
            Token::Bool(b) => {
                self.advance();
                Expr::Bool(b).into()
            }
            Token::Int(i) => {
                self.advance();
                Expr::Int(i).into()
            }
            Token::Uint(u) => {
                self.advance();
                Expr::Uint(u).into()
            }
            Token::Char(c) => {
                self.advance();
                Expr::Char(c).into()
            }
            Token::Float(f) => {
                self.advance();
                Expr::Float(f).into()
            }
            Token::String(ref s) => {
                let s = s.clone();
                self.advance();
                Expr::String(s).into()
            }
            Token::Identifier(ref name) => {
                let name = name.clone();
                self.advance();
                self.parse_identifer(name, variables, types)?.into()
            }
            Token::LeftParen => {
                self.advance();
                let expr = self.parse_expr(variables, types)?;
                self.expect(Token::RightParen, "Expected closing paren after parsing inner expression")?;
                expr.into()
            }
            Token::FnTypes => {
                self.advance();
                let pt = self.parse_type(types)?.get_sig();
                self.expect(Token::Arrow, "Expected arrow function")?;
                let rt = self.parse_type(types)?.get_sig();
                self.expect(Token::FnTypes, "Expected func param close")?;
                let block = self.parse_block(false, &mut HashMap::new(), types)?;

                Expr::Function { param_sig: pt, return_sig: rt, block: block.into() }.into()
            }
            Token::LeftBracket => {
                self.advance();
                self.parse_struct(variables, types)?
            }
            Token::If => {
                self.advance();
                self.parse_if(variables, types)?
            }
            _ => {
                return Err(ParseError::BadToken(self.current_token.clone(), "Found wrong token while parsing expression".to_string()))
            }
        };
        let left_expr = self.parse_method_call(variables,types, left_expr)?;
        match self.current_token {
            Token::Operator(ref op) => {
                let op = op.clone();
                self.advance();
                self.parse_binary(&mut variables,&mut types, op, left_expr)
            }
            _ => {
                Ok(left_expr)
            }
        }
    }

    fn parse_struct(
        &mut self, 
        variables: &mut HashMap<String, Arc<Expr>>, 
        types: &mut TypeMap,
    ) -> Result<Arc<Expr>, ParseError> {
        let mut var_defs= HashMap::new();
        while self.current_token!=Token::EOF {
            match self.current_token {
                Token::RightBracket => {
                    self.advance();
                    break;
                }
                Token::Identifier(ref name) => {
                    let name = name.clone();
                    self.advance();
                    self.expect(Token::Colon, "Expected type definition")?;
                    let expr = self.parse_expr(variables, types)?;
                    self.expect(Token::Comma, "Expected comma after type def")?;
                    var_defs.insert(name.to_string(), expr);
                }
                _ => {
                    return Err(ParseError::BadToken(self.current_token.clone(), "Expected expresion or unary operator".to_string()))
                }
            }
        }
        Ok(Expr::Struct { pairs: var_defs }.into())
    }


    fn parse_if(
        &mut self, 
        variables: &mut HashMap<String, Arc<Expr>>, 
        types: &mut TypeMap,
    ) -> Result<Arc<Expr>, ParseError> {
        let expr = self.parse_expr(variables, types)?;
        let then_block = self.parse_block(false, variables, types)?.into();
        let else_block = match self.current_token {
            Token::Else => {
                self.advance();
                Some(self.parse_block(false, variables, types)?)
            }
            _ => {
                None
            }
        };

        match *expr {
            Expr::Bool(b) => {
                if b {
                    Ok(then_block)
                } else {
                    if let Some(else_block) = else_block {
                        Ok(else_block.into())
                    } else {
                        Ok(Expr::Option(None).into())
                    }
                }
            }
            _ => {
                if let Some(else_block) = else_block {
                    Ok(Expr::If { 
                        condition: expr, 
                        then_branch: then_block,
                        else_branch: else_block.into()
                    }.into())
                } else {
                    Ok(Expr::If { 
                        condition: expr, 
                        then_branch: Expr::Option(Some(then_block)).into(), 
                        else_branch: Expr::Option(None).into()
                    }.into())
                }
            }
        }
    }

    fn parse_method_call(
        &mut self, 
        variables: &mut HashMap<String, Arc<Expr>>, 
        types: &mut TypeMap,
        mut left_expr: Arc<Expr>
    ) -> Result<Arc<Expr>, ParseError> {
        if self.current_token==Token::Period {
            match *left_expr {
                Expr::Type(_) => {
                    return Err(ParseError::BadToken(self.current_token.clone(), "Can't call methods on types".to_string()))
                }
                _ => {}
            }
        }
        while self.current_token==Token::Period {
            self.advance();
            match self.current_token {
                Token::Identifier(ref name) => {
                    let name = name.clone();
                    self.advance();
                    if *name=="as" {
                        self.expect(Token::LeftParen, "")?;
                        match self.current_token {
                            Token::Identifier(ref name) => {
                                let name = name.clone();
                                self.advance();
                                variables.insert(name.to_string(), left_expr.clone());
                                self.expect(Token::RightParen, "")?;
                            }
                            _ => {
                                unimplemented!("method call");
                            }
                        }
                    } else if *name=="pass_to" {
                        self.expect(Token::LeftParen, "")?;
                        let expr = self.parse_expr(variables, types)?;
                        left_expr = match *expr {
                            Expr::Identifier(ref name) => {
                                let name = name.clone();
                                match variables.get(&*name) {
                                    Some(expr) => {
                                        expr.clone()
                                    }
                                    None => {
                                        return Err(ParseError::BadToken(self.current_token.clone(), "Unknown variable".to_string()))
                                    }
                                }
                            }
                            _ => {
                                expr
                            }
                        };
                        self.expect(Token::RightParen, "")?;
                    } else {
                        unimplemented!("method call");
                    }
                }
                _ => {
                    unimplemented!("method call");
                }
            }
        }
        Ok(left_expr)
    }

    fn parse_binary(
        &mut self, 
        mut variables: &mut HashMap<String, Arc<Expr>>,mut types: &mut TypeMap, op: Arc<Operator>, left_expr: Arc<Expr>) -> Result<Arc<Expr>, ParseError> {
        let right_expr: Arc<Expr> = self.parse_expr(&mut variables, &mut types)?;
        match *op {
            Operator::Add => {
                Ok(Expr::add(left_expr, right_expr)?.into())
            }
            Operator::Sub=> {
                Ok(Expr::sub(left_expr, right_expr)?.into())
            }
            Operator::Mul=> {
                Ok(Expr::mult(left_expr, right_expr)?.into())
            }
            Operator::Div=> {
                Ok(Expr::div(left_expr, right_expr)?.into())
            }
            Operator::Mod=> {
                Ok(Expr::modd(left_expr, right_expr)?.into())
            }
            Operator::Eq=> {
                Ok(Expr::eq(left_expr, right_expr)?.into())
            }
            Operator::Neq=> {
                Ok(Expr::neq(left_expr, right_expr)?.into())
            }
            _ => {
                Ok(Expr::BinaryOp{left: left_expr, op, right: right_expr}.into())
            }
        }
    }

    fn parse_unary(&mut self, mut variables: &mut HashMap<String, Arc<Expr>>, mut types: &mut TypeMap, op: Arc<Operator>) -> Result<Expr, ParseError> {
        let right_expr: Arc<Expr> = self.parse_expr(&mut variables, &mut types)?;
        match *op {
            Operator::Not => {
                Ok(Expr::UnaryOp{op: op.clone(), expr:right_expr})
            }
            Operator::Sub => {
                match *right_expr {
                    Expr::Int(i) => {
                        Ok(Expr::Int(-i))
                    }
                    Expr::Float(f) => {
                        Ok(Expr::Float(-f))
                    }
                    _ => {
                        Err(ParseError::BadToken(self.current_token.clone(), "Expected floating point or integer following negative unary operator".to_string()))
                    }
                }
            }
            _ => {
                Err(ParseError::BadToken(self.current_token.clone(), "Expected expresion or unary operator".to_string()))
            }
        }
    }

    fn parse_type(&mut self, types: &mut TypeMap) -> Result<Arc<Type>, ParseError> {
        match self.current_token {
            Token::TNone => {
                self.advance();
                Ok(Type::None.into())
            }
            Token::Option => {
                self.advance();
                let t = self.parse_type(types)?;
                Ok(Type::Optional { type_def: t.into() }.into())
            }
            Token::TBool => {
                self.advance();
                Ok(Type::Bool.into())
            }
            Token::TInt => {
                self.advance();
                Ok(Type::Int.into())
            }
            Token::TUint => {
                self.advance();
                Ok(Type::Uint.into())
            }
            Token::TChar => {
                self.advance();
                Ok(Type::Char.into())
            }
            Token::TFloat => {
                self.advance();
                Ok(Type::Float.into())
            }
            Token::TString => {
                self.advance();
                Ok(Type::String.into())
            }
            Token::LeftBracket => {
                self.advance();
                let mut type_defs = Vec::new();
                while self.current_token!=Token::EOF {
                    match self.current_token {
                        Token::RightBracket => {
                            self.advance();
                            break;
                        }
                        Token::Identifier(ref name) => {
                            let name = name.clone();
                            self.advance();
                            self.expect(Token::Colon, "Expected type definition")?;
                            let t = self.parse_type(types)?.into();
                            self.expect(Token::Comma, "Expected comma after type def")?;
                            type_defs.push(Type::TypeDef { name, type_def: t }.into())
                        }
                        _ => {
                            return Err(ParseError::BadToken(self.current_token.clone(), "Expected expresion or unary operator".to_string()))
                        }
                    }
                }
                Ok(Type::Struct { pairs: type_defs }.into())
            }
            Token::Identifier(ref name) => {
                if let Some(t) = types.name_map.get(&**name) {
                    Ok(t.clone())
                } else {
                    return Err(ParseError::BadToken(self.current_token.clone(), "Expected expresion or unary operator".to_string()));
                }
            }
            _ => {
                Err(ParseError::BadToken(self.current_token.clone(), "Expected expresion or unary operator".to_string()))
            }
        }
    }

    fn parse_type_def(&mut self, name: Arc<String>, types: &mut TypeMap) -> Result<Expr, ParseError> {
        let t: Arc<Type> = self.parse_type(types)?.into();
        types.insert(name.to_string(), t.clone())?;
        Ok(Expr::Type(Type::TypeDef { name, type_def: t}))
    }

    fn parse_identifer(
        &mut self, 
        name: Arc<String>, 
        variables: &mut HashMap<String, Arc<Expr>>,
        types: &mut TypeMap
    ) -> Result<Arc<Expr>, ParseError> {
        match self.current_token {
            Token::Colon => {
                self.advance();
                Ok(self.parse_type_def(name, types)?.into())
            }
            Token::LeftParen => {
                unimplemented!("function call");
            }
            _ => {
                match variables.get(&*name) {
                    Some(expr) => {
                        Ok(expr.clone())
                    }
                    None => {
                        Ok(Expr::Identifier(name).into())
                    }
                }
            }
        }
    }
}

