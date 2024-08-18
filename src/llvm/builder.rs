use llvm_sys::{
    core::{
        LLVMAddIncoming, LLVMBuildBr, LLVMBuildCondBr, LLVMBuildAdd, LLVMBuildICmp, LLVMBuildMul,
        LLVMBuildSub, LLVMBuildPhi, LLVMBuildRet, LLVMBuildUIToFP, LLVMCreateBuilderInContext,
        LLVMDisposeBuilder, LLVMGetInsertBlock, LLVMPositionBuilderAtEnd,
    },
    prelude::{LLVMBuilderRef, LLVMValueRef},
    LLVMRealPredicate,
    LLVMIntPredicate,
};

use std::marker::PhantomData;

use super::{BasicBlock, FnValue, Module, PhiValue, Type, Value};

// Definition of LLVM C API functions using our `repr(transparent)` types.
extern "C" {
    fn LLVMBuildCall2(
        arg1: LLVMBuilderRef,
        arg2: Type<'_>,
        Fn: FnValue<'_>,
        Args: *mut Value<'_>,
        NumArgs: ::libc::c_uint,
        Name: *const ::libc::c_char,
    ) -> LLVMValueRef;
}

/// Wrapper for a LLVM IR Builder.
pub struct IRBuilder<'llvm> {
    builder: LLVMBuilderRef,
    _ctx: PhantomData<&'llvm ()>,
}

impl<'llvm> IRBuilder<'llvm> {
    /// Create a new LLVM IR Builder with the `module`s context.
    ///
    /// # Panics
    ///
    /// Panics if creating the IR Builder fails.
    pub fn with_ctx(module: &'llvm Module) -> IRBuilder<'llvm> {
        let builder = unsafe { LLVMCreateBuilderInContext(module.ctx()) };
        assert!(!builder.is_null());

        IRBuilder {
            builder,
            _ctx: PhantomData,
        }
    }

    /// Position the IR Builder at the end of the given Basic Block.
    pub fn pos_at_end(&self, bb: BasicBlock<'llvm>) {
        unsafe {
            LLVMPositionBuilderAtEnd(self.builder, bb.bb_ref());
        }
    }

    /// Get the BasicBlock the IRBuilder currently inputs into.
    ///
    /// # Panics
    ///
    /// Panics if LLVM API returns a `null` pointer.
    pub fn get_insert_block(&self) -> BasicBlock<'llvm> {
        let bb_ref = unsafe { LLVMGetInsertBlock(self.builder) };
        assert!(!bb_ref.is_null());

        BasicBlock::new(bb_ref)
    }

    pub fn iadd(&self, lhs: Value<'llvm>, rhs: Value<'llvm>) -> Value<'llvm> {
        debug_assert!(lhs.is_int(), "iadd: Expected i64 as lhs operand!");
        debug_assert!(rhs.is_int(), "iadd: Expected i64 as rhs operand!");

        let value_ref = unsafe {
            LLVMBuildAdd(
                self.builder,
                lhs.value_ref(),
                rhs.value_ref(),
                b"iadd\0".as_ptr().cast(),
            )
        };
        Value::new(value_ref)
    }

    pub fn isub(&self, lhs: Value<'llvm>, rhs: Value<'llvm>) -> Value<'llvm> {
        debug_assert!(lhs.is_int(), "isub: Expected i64 as lhs operand!");
        debug_assert!(rhs.is_int(), "isub: Expected i64 as rhs operand!");

        let value_ref = unsafe {
            LLVMBuildSub(
                self.builder,
                lhs.value_ref(),
                rhs.value_ref(),
                b"isub\0".as_ptr().cast(),
            )
        };
        Value::new(value_ref)
    }

    pub fn imul(&self, lhs: Value<'llvm>, rhs: Value<'llvm>) -> Value<'llvm> {
        debug_assert!(lhs.is_int(), "imul: Expected i64 as lhs operand!");
        debug_assert!(rhs.is_int(), "imul: Expected i64 as rhs operand!");

        let value_ref = unsafe {
            LLVMBuildMul(
                self.builder,
                lhs.value_ref(),
                rhs.value_ref(),
                b"imul\0".as_ptr().cast(),
            )
        };
        Value::new(value_ref)
    }

    /// Emit a [uitofp](https://llvm.org/docs/LangRef.html#uitofp-to-instruction) instruction.
    ///
    /// # Panics
    ///
    /// Panics if LLVM API returns a `null` pointer.
    pub fn uitofp(&self, val: Value<'llvm>, dest_type: Type<'llvm>) -> Value<'llvm> {
        debug_assert!(val.is_int(), "uitofp: Expected integer operand!");

        let value_ref = unsafe {
            LLVMBuildUIToFP(
                self.builder,
                val.value_ref(),
                dest_type.type_ref(),
                b"uitofp\0".as_ptr().cast(),
            )
        };
        Value::new(value_ref)
    }

    /// Emit a [call](https://llvm.org/docs/LangRef.html#call-instruction) instruction.
    ///
    /// # Panics
    ///
    /// Panics if LLVM API returns a `null` pointer.
    pub fn call(&self, fn_value: FnValue<'llvm>, args: &mut [Value<'llvm>]) -> Value<'llvm> {
        let value_ref = unsafe {
            LLVMBuildCall2(
                self.builder,
                fn_value.fn_type(),
                fn_value,
                args.as_mut_ptr(),
                args.len() as libc::c_uint,
                b"call\0".as_ptr().cast(),
            )
        };
        Value::new(value_ref)
    }

    /// Emit a [ret](https://llvm.org/docs/LangRef.html#ret-instruction) instruction.
    ///
    /// # Panics
    ///
    /// Panics if LLVM API returns a `null` pointer.
    pub fn ret(&self, ret: Value<'llvm>) {
        let ret = unsafe { LLVMBuildRet(self.builder, ret.value_ref()) };
        assert!(!ret.is_null());
    }

    /// Emit an unconditional [br](https://llvm.org/docs/LangRef.html#br-instruction) instruction.
    ///
    /// # Panics
    ///
    /// Panics if LLVM API returns a `null` pointer.
    pub fn br(&self, dest: BasicBlock<'llvm>) {
        let br_ref = unsafe { LLVMBuildBr(self.builder, dest.bb_ref()) };
        assert!(!br_ref.is_null());
    }

    /// Emit a conditional [br](https://llvm.org/docs/LangRef.html#br-instruction) instruction.
    ///
    /// # Panics
    ///
    /// Panics if LLVM API returns a `null` pointer.
    pub fn cond_br(&self, cond: Value<'llvm>, then: BasicBlock<'llvm>, else_: BasicBlock<'llvm>) {
        let br_ref = unsafe {
            LLVMBuildCondBr(
                self.builder,
                cond.value_ref(),
                then.bb_ref(),
                else_.bb_ref(),
            )
        };
        assert!(!br_ref.is_null());
    }

    /// Emit a [phi](https://llvm.org/docs/LangRef.html#phi-instruction) instruction.
    ///
    /// # Panics
    ///
    /// Panics if LLVM API returns a `null` pointer.
    pub fn phi(
        &self,
        phi_type: Type<'llvm>,
        incoming: &[(Value<'llvm>, BasicBlock<'llvm>)],
    ) -> PhiValue<'llvm> {
        let phi_ref =
            unsafe { LLVMBuildPhi(self.builder, phi_type.type_ref(), b"phi\0".as_ptr().cast()) };
        assert!(!phi_ref.is_null());

        for (val, bb) in incoming {
            debug_assert_eq!(
                val.type_of().kind(),
                phi_type.kind(),
                "Type of incoming phi value must be the same as the type used to build the phi node."
            );

            unsafe {
                LLVMAddIncoming(phi_ref, &mut val.value_ref() as _, &mut bb.bb_ref() as _, 1);
            }
        }

        PhiValue::new(phi_ref)
    }
}

impl Drop for IRBuilder<'_> {
    fn drop(&mut self) {
        unsafe { LLVMDisposeBuilder(self.builder) }
    }
}
