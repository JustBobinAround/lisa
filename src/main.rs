mod lexer;
mod parser;
mod expr;
use crate::lexer::{Lexer, Token};
use crate::parser::Parser;
fn main() {
    let input = r#"
        TestType: { 
            a: int,
            b: float,
        };
        TestTypeB: (int,int);

        TestType({
            a: 2,
            b: 3.3
        }).as(test);

        TestTypeB((0,1)).as(test_b);

        0.as(some_variable);
        42u.as(some_other);

        some_variable == 0;
        some_variable != 0;
        1.as(var_a)
         .as_shared(var_b);
         .as_shared(var_c);

        ~var_a += var_b;
        (int) -> int {
            @
        }.as(test_function);
        ~~var_b += 3;
    "#;
    let input = r#"
        {
            0.as("some_variable");
            if some_variable == 0 {
                1.as("var_a");
            } else {
                2.as("var_b");
            }
        }
    "#;
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    match parser.parse() {
        Ok(ast) => println!("{:#?}", ast),
        Err(e) => println!("Error: {:?}", e),
    }

    let mut lexer = Lexer::new(input);
    loop {
        let token = lexer.next_token();
        //println!("{:?}", token);
        if token == Token::EOF {
            break;
        }
    }
}

