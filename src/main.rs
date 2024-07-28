mod lexer;
use crate::lexer::{Lexer, Token};
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
        *1.as(var_a)
          .as(var_b);
        *var_a += var_b;
        (int) -> int {
            @
        }.as(test_function);
    "#;
    let mut lexer = Lexer::new(input);
    loop {
        let token = lexer.next_token();
        println!("{:?}", token);
        if token == Token::EOF {
            break;
        }
    }
}

