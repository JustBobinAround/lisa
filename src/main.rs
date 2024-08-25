use std::convert::TryFrom;
mod lexer;
mod parser;
mod llvm_ir;


use lexer::{Lexer, Token};
use llvm_ir::compile_to_bin;
use parser::{Expr, Parser};
use std::{collections::HashMap, str::Chars};

type ParseResult<T> = Result<T, String>;
fn parsing_loop<I>(mut parser: Parser<I>) -> ParseResult<Vec<Expr>>
where
    I: Iterator<Item = char>,
{
    let mut exprs = Vec::new();
    while parser.current_token()!=&Token::EOF {
        exprs.push(parser.parse_top_level_expr()?);
    }

    Ok(exprs)
}

fn run<I>(lexer: Lexer<I>) -> ParseResult<String>
where
    I: Iterator<Item = char>,
{
    let parser = Parser::new(lexer);

    let ast = parsing_loop(parser)?;

    let mut llvm_ir = String::from(r#"declare i32 @printf(i8*, ...)
define void @print_int(i64 %a) {
    %fmt = alloca [13 x i8], align 1
    store [13 x i8] c"Result: %d\n\00", [13 x i8]* %fmt, align 1
    %fmt_ptr = getelementptr [13 x i8], [13 x i8]* %fmt, i32 0, i32 0
    call i32 @printf(i8* %fmt_ptr, i64 %a)
    ret void
}

define void @print_float(double %a) {
    %fmt = alloca [13 x i8], align 1
    store [13 x i8] c"Result: %f\n\00", [13 x i8]* %fmt, align 1
    %fmt_ptr = getelementptr [13 x i8], [13 x i8]* %fmt, i32 0, i32 0
    call i32 @printf(i8* %fmt_ptr, double %a)
    ret void
}
"#);

    let mut main_debug = String::from("define i32 @main() {\n");

    for (i, expr) in ast.iter().enumerate() {
        llvm_ir.push_str(&expr.to_anon_fn(i as u64));
        main_debug.push_str(&expr.to_anon_call(i as u64));
        main_debug.push_str(&expr.anon_debug(i as u64));
    }

    main_debug.push_str("ret i32 0\n}");

    llvm_ir.push_str(&main_debug);

    /*

    format!(r#"declare i32 @printf(i8*, ...)
{}
define i32 @main() {{
    %call = call i64 @__anon_expr()
%fmt = alloca [13 x i8], align 1
store [13 x i8] c"Result: %d\n\00", [13 x i8]* %fmt, align 1

%fmt_ptr = getelementptr [13 x i8], [13 x i8]* %fmt, i32 0, i32 0
call i32 @printf(i8* %fmt_ptr, i64 %call)
    ret i32 0
}}
"#, )
*/

    Ok(llvm_ir)
}


pub enum Either<A, B> {
    A(A),
    B(B),
}

fn main() -> ParseResult<()>{
    let input = r#"2+2*4;3+3;"#;

    let lexer: Lexer<Chars> = Lexer::new(
        input.chars()
    );
    let llvm_out = run(lexer)?;

    println!("{}",&llvm_out);
    //let _ = compile_to_bin(&"test".to_string(), &include_str!("./test.ll").to_string());
    let _ = compile_to_bin(&"test".to_string(), &llvm_out);

    Ok(())
}
