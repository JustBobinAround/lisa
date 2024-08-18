#[derive(Debug, PartialEq, Clone)]
pub enum Op{
    Add,
    Sub,
    Mul,
    Div
}
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    EOF,
    Invalid(char),
    Int(i64),
    UInt(u64),
    Float(f64),
    Char(char),
    Ident(String),
    Period,
    Op(Op),
    If,
    Else,
    LBrack,
    RBrack,
    LParen,
    RParen,
    LBrace,
    RBrace,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Lexer<I>
where
    I: Iterator<Item = char>,
{
    input: I,
    last_char: Option<char>,
}


impl<I> Lexer<I>
where
    I: Iterator<Item = char>,
{
    pub fn new(mut input: I) -> Lexer<I> {
        let last_char = input.next();
        Lexer { input, last_char }
    }

    fn advance(&mut self) -> Option<char> {
        self.last_char = self.input.next();
        self.last_char
    }

    fn skip_whitespace(&mut self) {
        while matches!(self.last_char, Some(c) if c.is_ascii_whitespace()) {
            self.advance();
        }

    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let last_char = if let Some(c) = self.last_char {
            c
        } else {
            return Token::EOF;
        };

        match last_char {
            '.' => {
                self.advance();
                Token::Period
            },
            '+' => {
                self.advance();
                Token::Op(Op::Add)
            }
            '-' => {
                self.advance();
                Token::Op(Op::Sub)
            }
            '*' => {
                self.advance();
                Token::Op(Op::Mul)
            }
            '/' => {
                match self.advance(){
                    Some('/') => {
                        self.parse_comment()
                    }
                    _ => {Token::Op(Op::Div)}
                }
            }
            '{' => {
                self.advance();
                Token::LBrack
            }
            '}' => {
                self.advance();
                Token::RBrack
            }
            '[' => {
                self.advance();
                Token::LBrace
            }
            ']' => {
                self.advance();
                Token::RBrace
            }
            '(' => {
                self.advance();
                Token::LParen
            }
            ')' => {
                self.advance();
                Token::RParen
            }
            '\'' => {
                self.parse_char()
            }
            '0'..='9' => return self.parse_number(last_char),
            'a'..='z' | 'A'..='Z' | '_' => return self.parse_ident(last_char),
            _ => Token::Invalid(last_char)
        }
    }

    pub fn parse_comment(&mut self) -> Token {
        loop {
            match self.advance() {
                Some(c) if c == '\r' || c == '\n' => return self.next_token(),
                None => return Token::EOF,
                _ => { 
                    // do nothing for now... maybe docs in the future
                }
            }
        }
    }

    fn parse_number(&mut self, first_num: char) -> Token {
        let mut num_str = String::new();
        num_str.push(first_num);
        let mut has_decimal_point = false;
        let mut is_unsigned = false;

        while let Some(c) = self.advance() {
            match c {
                'u' => {
                    self.advance();
                    is_unsigned = true;
                    break;
                }
                '0'..='9' => num_str.push(c),
                '.' => {
                    if has_decimal_point {
                        println!("hit");
                        // The second '.' encountered indicates a method call should follow, so we
                        // stop parsing here.
                        break;
                    }
                    has_decimal_point = true;
                    num_str.push(c);
                }
                _ => break
            }
        }

        if has_decimal_point && !is_unsigned {
            match num_str.parse::<f64>() {
                Ok(n) => Token::Float(n),
                Err(_) => Token::Invalid('.'),
            }
        } else if !(has_decimal_point || is_unsigned){ 
            match num_str.parse::<i64>() {
                Ok(n) => Token::Int(n),
                Err(_) => Token::Invalid('.'),
            }
        } else if !has_decimal_point && is_unsigned{
            match num_str.parse::<u64>() {
                Ok(n) => Token::UInt(n),
                Err(_) => Token::Invalid('.'),
            }
        } else {
            Token::Invalid('u')
        }
    }    
    pub fn parse_ident(&mut self, first_char: char) -> Token {
        let mut ident = String::new();
        ident.push(first_char);
        while let Some(c) = self.advance() {
            if c.is_alphanumeric() || c=='_' {
                ident.push(c);
            } else {
                break;
            }
        }

        match ident.as_str() {
            "if" => Token::If,
            "else" => Token::Else,
            _ => Token::Ident(ident)
        }
    }

    pub fn parse_char(&mut self) -> Token {
        let c = match self.advance() {
            Some(c) => c,
            None => return Token::EOF
        };
        match self.advance() {
            Some('\'') => {
                self.advance();
                Token::Char(c)
            },
            Some(c) => Token::Invalid(c),
            None => Token::Invalid(c)
        }
    }

}
#[cfg(test)]
mod test {
    use std::str::Chars;

    use super::{Lexer, Token, Op};

    macro_rules! test_tokens {
        ($t: literal => {$($x:expr),* $(,)?}) => {{
            let mut lex = Lexer::new($t.chars());
            $(
                assert_eq!($x, lex.next_token());
            )*
        }};
    }

    #[test]
    fn test_identifier() {
        test_tokens!("a asdf c_123 __ASDF" => {
            Token::Ident("a".into()),
            Token::Ident("asdf".into()),
            Token::Ident("c_123".into()),
            Token::Ident("__ASDF".into()),
            Token::EOF
        });
    }

    #[test]
    fn test_number() {
        test_tokens!("1 123 123.234 345u 222.222." => {
            Token::Int(1),
            Token::Int(123),
            Token::Float(123.234),
            Token::UInt(345),
            Token::Float(222.222),
            Token::Period,
            Token::EOF
        });
    }

    #[test]
    fn test_comment() {
        test_tokens!("asdf //1 123 123.234 345u 222.222." => {
            Token::Ident("asdf".into()),
            Token::EOF
        });
    }

    #[test]
    fn test_char() {
        test_tokens!("asdf \'a\' 234" => {
            Token::Ident("asdf".into()),
            Token::Char('a'),
            Token::Int(234),
            Token::EOF
        });
    }

    #[test]
    fn test_ops() {
        test_tokens!("asdf+1 -(d *2.2) / 'a'" => {
            Token::Ident("asdf".into()),
            Token::Op(Op::Add),
            Token::Int(1),
            Token::Op(Op::Sub),
            Token::LParen,
            Token::Ident("d".into()),
            Token::Op(Op::Mul),
            Token::Float(2.2),
            Token::RParen,
            Token::Op(Op::Div),
            Token::Char('a'),
            Token::EOF
        });
    }

    #[test]
    fn test_if() {
        test_tokens!("if asdf { 123 } else { 234 }" => {
            Token::If,
            Token::Ident("asdf".into()),
            Token::LBrack,
            Token::Int(123),
            Token::RBrack,
            Token::Else,
            Token::LBrack,
            Token::Int(234),
            Token::RBrack,
            Token::EOF
        });
    }
}
