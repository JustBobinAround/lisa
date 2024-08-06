# Lisa Includes Static Analysis 

This language is going to be used as a tool to explore static type analysis and
code synthesis techniques along with the possible incorporation of AI.

### Todo
- [x] Implement initial Lexer
- [x] Implement initial Parser
- [x] Implement initial Interpreter
- [x] Implement initial type signatures
- [ ] Fix type signatures
- [ ] Implement initial semantic analyzer 
- [ ] Refactor Lexer
- [ ] Finish implementing Parser
- [ ] Refactor Parser
- [ ] Finish implementing Interpreter
- [ ] Refactor Interpreter
- [ ] Refine and finish initial spec
- [ ] General Refactor
- [ ] Performance Analysis
- [ ] Implement intermediate pre-processor 
- [ ] Implement standard library and external lib binding aids
- [ ] Bootstrap glibc if possible


### Change Log

- **2024-07-29 17:39**: Implemented most of parser. Ran into issue with after
attempting to add operators to expression parsing.
- **2024-07-29 19:21**: Updated Assignment expressions:
Assignment methods now have access to the prior express within their internal
data structure.
**CODE**
```rs
        TypeA: int;
        TypeB: TypeA;
        TypeC: TypeB;
        TypeD: TypeA;
        TypeE: TypeD;
        |TypeE -> uint| {
            0.as(test);
            '1'.as(var_b);
        }.test(None)
         .as(test)
         .as_shared(asdf);

0.as(test);
```
**AST DATA**
```rs
Assignment {
    prior_expr: Int(
        0,
    ),
    name: "test",
},
```
- **2024-07-29 20:18**: Finished Most of Expression AST part
- **2024-07-29 20:38**: Added initial engine boilerplate
