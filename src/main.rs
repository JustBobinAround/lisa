mod lexer;
mod parser;
mod expr;
mod type_def;
mod engine;
mod static_analyzer;

use std::collections::HashMap;

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
        TypeTest: {
            a: Self,
            b: float,
        };
        TypeB: {
            a: int,
            b: float,
        };


        |Self -> Self| {
            5
        }.asdsf()
         .test() 
         .lkjh();
    "#;
    let input = r#"
        test: {
            a: ?int,
            b: int,
        };

        |int->int| {
            @
        };
        5.as(test1);
        2.as(a);

        if a==2 {
            Some(3)
        } else {
            None
        }.as(a).test(a);
        {
            a: 2,
            b: 3,
        };

        test2
            .pass_to(4)
            .as(test2);

        test2==test1;
    "#;
    let input = r#"
        

        |int -> int| {
            0
        }.as(test);

    2.pass_to(test);
    "#;
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    //let mut interpreter = Interpreter::new();
    //interpreter.interpret(&mut parser).expect("Interpretation failed");
    match parser.parse() {
        Ok(ast) =>  {
        println!("{:#?}", ast);
            let mut env = HashMap::new();
            let t = ast.type_check(&mut env);
            match t {
                Ok(t) => println!("{:#?}", t),
                Err(e) => println!("Error: {:?}", e),
            }
        },
        Err(e) => println!("Error: {:?}", e),
    }
}
