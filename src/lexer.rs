use std::str::Chars;

#[derive(Debug, PartialEq, Clone)]
pub enum Operator {
    Sub,
    SubAssign,
    Add,
    AddAssign,
    Mul,
    MulAssign,
    Pow,
    PowAssign,
    Div,
    DivAssign,
    Mod,
    ModAssign,
    Not,
    Eq,
    BitXor,
    And,
    BitAnd,
    Or,
    BitOr,
}
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    None,
    Bool(bool),
    Int(i64),
    Uint(u64),
    Char(char),
    Float(f64),
    String(String),
    Identifier(String),
    Keyword(String),
    Operator(Operator),
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
                '(' => return self.consume(Token::LeftParen),
                ')' => return self.consume(Token::RightParen),
                '{' => return self.consume(Token::LeftBrace),
                '}' => return self.consume(Token::RightBrace),
                '[' => return self.consume(Token::LeftBracket),
                ']' => return self.consume(Token::RightBracket),
                ',' => return self.consume(Token::Comma),
                '.' => return self.consume(Token::Comma),
                ':' => return self.consume(Token::Period),
                ';' => return self.consume(Token::Semicolon),
                '-' => {
                    if self.peek() == Some('>') {
                        self.advance();
                        self.advance();
                        return Token::Arrow;
                    } else {
                        return self.consume(Token::Operator(Operator::Sub));
                    }
                }
                '@' => {
                    self.advance();
                    return Token::Param;
                }
                '$' => {
                    self.advance();
                    return Token::Macro;
                }
                '+' => {
                    self.advance();
                    if self.current_char.is_some_and(|c|c=='='){
                        self.advance();
                        return Token::Operator(Operator::AddAssign);
                    } else {
                        return Token::Operator(Operator::Add);
                    }
                }
                '*' => {
                    self.advance();
                    if self.current_char.is_some_and(|c|c=='*'){
                        self.advance();
                        return Token::Operator(Operator::Pow);
                    } else {
                        if self.current_char.is_some_and(|c|c=='='){
                            self.advance();
                            return Token::Operator(Operator::MulAssign);
                        } else {
                            return Token::Operator(Operator::Mul);
                        }
                    }
                }
                '/' => {
                    self.advance();
                    if self.current_char.is_some_and(|c|c=='='){
                        self.advance();
                        return Token::Operator(Operator::DivAssign);
                    } else {
                        return Token::Operator(Operator::Div);
                    }
                }
                '%' => {
                    self.advance();
                    if self.current_char.is_some_and(|c|c=='='){
                        self.advance();
                        return Token::Operator(Operator::ModAssign);
                    } else {
                        return Token::Operator(Operator::Mod);
                    }
                }
                '!' => {
                    self.advance();
                    return Token::Operator(Operator::Not);
                }
                '=' => {
                    self.advance();
                    if self.current_char.is_some_and(|c|c=='='){
                        self.advance();
                        return Token::Operator(Operator::Eq);
                    } else {
                        return Token::Invalid('=');
                    }
                }
                '^' => {
                    self.advance();
                    return Token::Operator(Operator::BitXor);
                }
                '&' => {
                    self.advance();
                    if self.current_char.is_some_and(|c|c=='&'){
                        self.advance();
                        return Token::Operator(Operator::And);
                    } else {
                        return Token::Operator(Operator::BitAnd);
                    }
                }
                '|' => {
                    self.advance();
                    if self.current_char.is_some_and(|c|c=='|'){
                        self.advance();
                        return Token::Operator(Operator::Or);
                    } else {
                        return Token::Operator(Operator::BitOr);
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
                return Token::String(string);
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
            "true" => Token::Bool(true),
            "false" => Token::Bool(false),
            "None" => Token::None,
            _ => Token::Identifier(ident),
        }
    }
}

