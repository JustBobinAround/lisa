mod lexer;
mod parser;
mod expr;
mod type_def;
mod engine;

use engine::Interpreter;

use crate::lexer::Lexer;
use crate::parser::Parser;

fn main() {
    let input = r#"
        TypeA: int;
        TypeB: TypeA;
        TypeC: TypeB;
        TypeD: TypeA;
        TypeE: TypeD;
        |TypeE -> uint| {
            0.as(test);
            '1'.as(var_b);


            a + b + c;
            a + (b + c);
            (!a + !(b + c)).as(will_this_work);
        }.test(None)
         .as(test)
         .as_shared(asdf);
    "#;
    let input = r#"
        TypeTest: int;
        |{a: int, b: int} -> int| {

        }.as(TypeTest::test);
        |TypeTest -> int| {

        }.as(int::test);
    "#;
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    //let mut interpreter = Interpreter::new();
    //interpreter.interpret(&mut parser).expect("Interpretation failed");
    match parser.parse() {
        Ok(ast) => println!("{:#?}", ast),
        Err(e) => println!("Error: {:?}", e),
    }
}
