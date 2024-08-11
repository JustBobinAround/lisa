use std::{str::Chars, sync::Arc};

use crate::type_def::Type;

#[derive(Debug, PartialEq, Clone)]
pub enum Operator {
    AssignOp(Box<Operator>),
    Sub, // done
    Add, // done
    Mul, // done
    Div, // done
    Mod, // done
    Not, // done
    Eq,  // done
    Neq, // done
    Gt,
    Lt,
    GtEq,
    LtEq,
    BitXor,
    And,
    BitAnd,
    Or,
    BitOr,
}
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Generic,
    Option,
    TNone,
    TBool,
    TInt,
    TUint,
    TChar,
    TFloat,
    TString,
    FnTypes,
    Bool(bool),
    Int(i64),
    Uint(u64),
    Char(char),
    Float(f64),
    String(Arc<String>),
    Identifier(Arc<String>),
    If,
    Else,
    Operator(Arc<Operator>),
    Arrow,
    Comma,
    Period,
    Colon,
    Semicolon,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    EOF,
    Param,
    Mut,
    SharedMut,
    Macro,
    Invalid(char),
}

pub struct Lexer<'a> {
    input: Chars<'a>,
    current_char: Option<char>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer {
            input: input.chars(),
            current_char: None,
        };
        lexer.advance();
        lexer
    }

    fn advance(&mut self) {
        self.current_char = self.input.next();
    }

    fn peek(&self) -> Option<char> {
        self.input.clone().next()
    }

    pub fn next_token(&mut self) -> Token {
        while let Some(c) = self.current_char {
            match c {
                ' ' | '\t' | '\n' | '\r' => {
                    while self.current_char.is_some_and(|c| c==' ' || c=='\t'|| c=='\n'|| c=='\r'){
                        self.advance();
                    }
                    continue;
                }
                '>' => {
                    self.advance();
                    if self.current_char.is_some_and(|c|c=='='){
                        self.advance();
                        return Token::Operator(Operator::GtEq.into());
                    } else {
                        return Token::Operator(Operator::Gt.into());
                    }
                },
                '<' => {
                    self.advance();
                    if self.current_char.is_some_and(|c|c=='='){
                        self.advance();
                        return Token::Operator(Operator::LtEq.into());
                    } else {
                        return Token::Operator(Operator::Lt.into());
                    }
                },
                '(' => return self.consume(Token::LeftParen),
                ')' => return self.consume(Token::RightParen),
                '{' => return self.consume(Token::LeftBracket),
                '}' => return self.consume(Token::RightBracket),
                '[' => return self.consume(Token::LeftBrace),
                ']' => return self.consume(Token::RightBrace),
                ',' => return self.consume(Token::Comma),
                '.' => return self.consume(Token::Period),
                ':' => return self.consume(Token::Colon),
                ';' => return self.consume(Token::Semicolon),
                '-' => {
                    if self.peek() == Some('>') {
                        self.advance();
                        self.advance();
                        return Token::Arrow;
                    } else {
                        return self.consume(Token::Operator(Operator::Sub.into()).into());
                    }
                }
                '@' => {
                    self.advance();
                    return Token::Param;
                }
                '#' => {
                    self.advance();
                    return Token::Macro;
                }
                '+' => {
                    self.advance();
                    if self.current_char.is_some_and(|c|c=='='){
                        self.advance();
                        return Token::Operator(Operator::AssignOp(Box::new(Operator::Add)).into());
                    } else {
                        return Token::Operator(Operator::Add.into());
                    }
                }
                '~' => {
                    self.advance();
                    if self.current_char.is_some_and(|c|c=='~'){
                        self.advance();
                        return Token::SharedMut;
                    } else {
                        return Token::Mut;
                    }
                }
                '*' => {
                    self.advance();
                    if self.current_char.is_some_and(|c|c=='='){
                        self.advance();
                        return Token::Operator(Operator::AssignOp(Operator::Mul.into()).into());
                    } else {
                        return Token::Operator(Operator::Mul.into());
                    }
                }
                '/' => {
                    self.advance();
                    if self.current_char.is_some_and(|c|c=='='){
                        self.advance();
                        return Token::Operator(Operator::AssignOp(Operator::Div.into()).into());
                    } else {
                        return Token::Operator(Operator::Div.into());
                    }
                }
                '%' => {
                    self.advance();
                    if self.current_char.is_some_and(|c|c=='='){
                        self.advance();
                        return Token::Operator(Operator::AssignOp(Operator::Mod.into()).into());
                    } else {
                        return Token::Operator(Operator::Mod.into());
                    }
                }
                '?' => {
                    self.advance();
                    return Token::Option;
                }
                '!' => {
                    self.advance();
                    if self.current_char.is_some_and(|c|c=='='){
                        self.advance();
                        return Token::Operator(Operator::Neq.into());
                    } else {
                        return Token::Operator(Operator::Not.into());
                    }
                }
                '=' => {
                    self.advance();
                    if self.current_char.is_some_and(|c|c=='='){
                        self.advance();
                        return Token::Operator(Operator::Eq.into());
                    } else {
                        return Token::Invalid('=');
                    }
                }
                '^' => {
                    self.advance();
                    return Token::Operator(Operator::BitXor.into());
                }
                '&' => {
                    self.advance();
                    if self.current_char.is_some_and(|c|c=='&'){
                        self.advance();
                        return Token::Operator(Operator::And.into());
                    } else {
                        return Token::Operator(Operator::BitAnd.into());
                    }
                }
                '|' => {
                    self.advance();
                    if self.current_char.is_some_and(|c|c=='|'){
                        self.advance();
                        if self.current_char.is_some_and(|c|c=='|'){
                            self.advance();
                            return Token::Operator(Operator::BitOr.into());
                        } else {
                            return Token::Operator(Operator::Or.into());
                        }
                    } else {
                        return Token::FnTypes;
                    }
                }
                '0'..='9' => return self.number(),
                '\'' => return self.char_literal(),
                '"' => return self.string_literal(),
                'a'..='z' | 'A'..='Z' | '_' => return self.identifier_or_keyword(),
                _ => return self.consume(Token::Invalid(c)),
            }
        }
        Token::EOF
    }

    fn consume(&mut self, token: Token) -> Token {
        self.advance();
        token
    }

    fn number(&mut self) -> Token {
        let mut num_str = String::new();
        let mut is_unsigned = false;
        let mut is_float = false;
        while let Some(c) = self.current_char {
            if c=='u' && !num_str.chars().last().is_some_and(|c| c=='.') {
                is_unsigned = true;
                break;
            } else {
                if c.is_numeric() {
                    num_str.push(c);
                    self.advance();
                } else if c == '.' && !is_float{
                    if self.peek().is_some_and(|c| c.is_digit(10)){
                        is_float = true;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
        }
        if is_float {
            match num_str.parse::<f64>() {
                Ok(n) => Token::Float(n),
                Err(_) => Token::Invalid('.'),
            }
        } else if is_unsigned {
            match num_str.parse::<u64>() {
                Ok(n) => Token::Uint(n),
                Err(_) => Token::Invalid(' '),
            }
        } else {
            match num_str.parse::<i64>() {
                Ok(n) => Token::Int(n),
                Err(_) => Token::Invalid(' '),
            }
        }
    }

    fn char_literal(&mut self) -> Token {
        self.advance();
        if let Some(c) = self.current_char {
            self.advance();
            if self.current_char == Some('\'') {
                self.advance();
                return Token::Char(c);
            }
        }
        Token::Invalid('\'')
    }

    fn string_literal(&mut self) -> Token {
        let mut string = String::new();
        self.advance();
        while let Some(c) = self.current_char {
            if c == '"' {
                self.advance();
                return Token::String(string.into());
            } else {
                string.push(c);
                self.advance();
            }
        }
        Token::Invalid('"')
    }

    fn identifier_or_keyword(&mut self) -> Token {
        let mut ident = String::new();
        while let Some(c) = self.current_char {
            if c.is_alphanumeric() || c == '_' {
                ident.push(c);
                self.advance();
            } else {
                break;
            }
        }
        match ident.as_str() {
            "Self"      => Token::Generic,
            "None"   => Token::TNone,
            "bool"   => Token::TBool,
            "int"    => Token::TInt,
            "uint"   => Token::TUint,
            "char"   => Token::TChar,
            "float"  => Token::TFloat,
            "String" => Token::TString,
            "if" => Token::If,
            "else" => Token::Else,
            "true" => Token::Bool(true),
            "false" => Token::Bool(false),
            _ => Token::Identifier(Arc::new(ident)),
        }
    }
}

