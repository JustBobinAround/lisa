use std::convert::TryFrom;
mod lexer;
mod parser;
mod llvm_ir;


use lexer::{Lexer, Token};
use llvm_ir::compile_to_bin;
use parser::{Parser, PrototypeAST};
use std::{collections::HashMap, str::Chars};

fn parsing_loop<I>(mut parser: Parser<I>)
where
    I: Iterator<Item = char>,
{

}

fn run<I>(lexer: Lexer<I>)
where
    I: Iterator<Item = char>,
{
    let mut parser = Parser::new(lexer);

    parsing_loop(parser);
}


/// Either type, for APIs accepting two types.
pub enum Either<A, B> {
    A(A),
    B(B),
}

fn main() {
    let input = r#" 2+2+3"#;

    let mut lexer: Lexer<Chars> = Lexer::new(
        input.chars()
    );
    //run(lexer);

    let _ = compile_to_bin(&"test".to_string(), &include_str!("./test.ll").to_string());

}
