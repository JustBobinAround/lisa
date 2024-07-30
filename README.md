# Lisa Includes Static Analysis 

This language is going to be used as a tool to explore static type analysis and
code synthesis techniques along with the possible incorporation of AI.

### Change Log

- **2024-07-29 17:39:00**: Implemented most of parser. Ran into issue with after
attempting to add operators to expression parsing.
- **2024-07-29 19:21:00**: Updated Assignment expressions:
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
