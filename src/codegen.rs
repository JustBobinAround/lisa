use std::collections::HashMap;

use crate::llvm::{FnValue, FunctionPassManager, IRBuilder, Module, Value};
use crate::parser::{ExprAST, FunctionAST, PrototypeAST};
use crate::Either;

use crate::lexer::Op;


type CodegenResult<T> = Result<T, String>;

/// Code generator from kaleidoscope AST to LLVM IR.
pub struct Codegen<'llvm, 'a> {
    module: &'llvm Module,
    builder: &'a IRBuilder<'llvm>,
    fpm: &'a FunctionPassManager<'llvm>,
    fn_protos: &'a mut HashMap<String, PrototypeAST>,
}

impl<'llvm, 'a> Codegen<'llvm, 'a> {
    /// Compile either a [`PrototypeAST`] or a [`FunctionAST`] into the LLVM `module`.
    pub fn compile(
        module: &'llvm Module,
        fn_protos: &mut HashMap<String, PrototypeAST>,
        compilee: Either<&PrototypeAST, &FunctionAST>,
    ) -> CodegenResult<FnValue<'llvm>> {
        let mut cg = Codegen {
            module,
            builder: &IRBuilder::with_ctx(module),
            fpm: &FunctionPassManager::with_ctx(module),
            fn_protos,
        };
        let mut variables = HashMap::new();

        match compilee {
            Either::A(proto) => Ok(cg.codegen_prototype(proto)),
            Either::B(func) => cg.codegen_function(func, &mut variables),
        }
    }

    fn codegen_expr(
        &self,
        expr: &ExprAST,
        named_values: &mut HashMap<String, Value<'llvm>>,
    ) -> CodegenResult<Value<'llvm>> {
        match expr {
            ExprAST::Int(i) => Ok(self.module.type_i64().const_i64(*i)),
            ExprAST::Variable(name) => match named_values.get(name.as_str()) {
                Some(value) => Ok(*value),
                None => Err("Unknown variable name".into()),
            },
            ExprAST::BinOp(binop, lhs, rhs) => {
                let l = self.codegen_expr(lhs, named_values)?;
                let r = self.codegen_expr(rhs, named_values)?;

                match binop {
                    Op::Add => Ok(self.builder.iadd(l, r)),
                    Op::Sub => Ok(self.builder.isub(l, r)),
                    Op::Mul => Ok(self.builder.imul(l, r)),
                    _ => Err("invalid binary operator".into()),
                }
            }
        }
    }

    fn codegen_prototype(&self, PrototypeAST(name, args): &PrototypeAST) -> FnValue<'llvm> {
        let type_i64 = self.module.type_i64();

        let mut doubles = Vec::new();
        doubles.resize(args.len(), type_i64);

        // Build the function type: fn(f64, f64, ..) -> f64
        let ft = self.module.type_fn(&mut doubles, type_i64);

        // Create the function declaration.
        let f = self.module.add_fn(name, ft);

        // Set the names of the function arguments.
        for idx in 0..f.args() {
            f.arg(idx).set_name(&args[idx]);
        }

        f
    }

    fn codegen_function(
        &mut self,
        FunctionAST(proto, body): &FunctionAST,
        named_values: &mut HashMap<String, Value<'llvm>>,
    ) -> CodegenResult<FnValue<'llvm>> {
        // Insert the function prototype into the `fn_protos` map to keep track for re-generating
        // declarations in other modules.
        self.fn_protos.insert(proto.0.clone(), proto.clone());

        let the_function = self.get_function(&proto.0)
            .expect("If proto not already generated, get_function will do for us since we updated fn_protos before-hand!");

        if the_function.basic_blocks() > 0 {
            return Err("Function cannot be redefined.".into());
        }

        // Create entry basic block to insert code.
        let bb = self.module.append_basic_block(the_function);
        self.builder.pos_at_end(bb);

        // New scope, clear the map with the function args.
        named_values.clear();

        // Update the map with the current functions args.
        for idx in 0..the_function.args() {
            let arg = the_function.arg(idx);
            named_values.insert(arg.get_name().into(), arg);
        }

        // Codegen function body.
        if let Ok(ret) = self.codegen_expr(body, named_values) {
            self.builder.ret(ret);
            assert!(the_function.verify());

            // Run the optimization passes on the function.
            self.fpm.run(the_function);

            Ok(the_function)
        } else {
            todo!("Failed to codegen function body, erase from module!");
        }
    }

    /// Lookup function with `name` in the LLVM module and return the corresponding value reference.
    /// If the function is not available in the module, check if the prototype is known and codegen
    /// it.
    /// Return [`None`] if the prototype is not known.
    fn get_function(&self, name: &str) -> Option<FnValue<'llvm>> {
        let callee = match self.module.get_fn(name) {
            Some(callee) => callee,
            None => {
                let proto = self.fn_protos.get(name)?;
                self.codegen_prototype(proto)
            }
        };

        Some(callee)
    }
}

