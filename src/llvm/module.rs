use llvm_sys::{
    core::{
        LLVMAddFunction, LLVMAppendBasicBlockInContext, LLVMCreateBasicBlockInContext,
        LLVMDisposeModule, LLVMInt64TypeInContext, LLVMDumpModule, LLVMGetNamedFunction,
        LLVMModuleCreateWithNameInContext,
    },
    orc2::{
        LLVMOrcCreateNewThreadSafeContext, LLVMOrcCreateNewThreadSafeModule,
        LLVMOrcDisposeThreadSafeContext, LLVMOrcThreadSafeContextGetContext,
        LLVMOrcThreadSafeContextRef, LLVMOrcThreadSafeModuleRef,
    },
    prelude::{LLVMBool, LLVMContextRef, LLVMModuleRef, LLVMTypeRef},
    LLVMTypeKind,
};

use std::convert::TryFrom;

use super::{BasicBlock, FnValue, Type};
use crate::SmallCStr;

extern "C" {
    fn LLVMFunctionType(
        ReturnType: Type<'_>,
        ParamTypes: *mut Type<'_>,
        ParamCount: ::libc::c_uint,
        IsVarArg: LLVMBool,
    ) -> LLVMTypeRef;
}

pub struct Module {
    tsctx: LLVMOrcThreadSafeContextRef,
    ctx: LLVMContextRef,
    module: LLVMModuleRef,
}

impl<'llvm> Module {
    pub fn new() -> Self {
        let (tsctx, ctx, module) = unsafe {
            let tc = LLVMOrcCreateNewThreadSafeContext();
            assert!(!tc.is_null());

            let c = LLVMOrcThreadSafeContextGetContext(tc);
            let m = LLVMModuleCreateWithNameInContext(b"module\0".as_ptr().cast(), c);
            assert!(!c.is_null() && !m.is_null());
            (tc, c, m)
        };

        Module { tsctx, ctx, module }
    }

    #[inline]
    pub(super) fn ctx(&self) -> LLVMContextRef {
        self.ctx
    }

    #[inline]
    pub(super) fn module(&self) -> LLVMModuleRef {
        self.module
    }

    #[inline]
    pub(super) fn into_raw_thread_safe_module(mut self) -> LLVMOrcThreadSafeModuleRef {
        let m = std::mem::replace(&mut self.module, std::ptr::null_mut());
        let tm = unsafe { LLVMOrcCreateNewThreadSafeModule(m, self.tsctx) };
        assert!(!tm.is_null());

        tm
    }

    pub fn dump(&self) {
        unsafe { LLVMDumpModule(self.module) };
    }

    pub fn type_i64(&self) -> Type<'llvm> {
        let type_ref = unsafe { LLVMInt64TypeInContext(self.ctx) };
        Type::new(type_ref)
    }

    pub fn type_fn(&'llvm self, args: &mut [Type<'llvm>], ret: Type<'llvm>) -> Type<'llvm> {
        let type_ref = unsafe {
            LLVMFunctionType(
                ret,
                args.as_mut_ptr(),
                args.len() as libc::c_uint,
                0, /* IsVarArg */
            )
        };
        Type::new(type_ref)
    }

    pub fn add_fn(&'llvm self, name: &str, fn_type: Type<'llvm>) -> FnValue<'llvm> {
        debug_assert_eq!(
            fn_type.kind(),
            LLVMTypeKind::LLVMFunctionTypeKind,
            "Expected a function type when adding a function!"
        );

        let name = SmallCStr::try_from(name)
            .expect("Failed to convert 'name' argument to small C string!");

        let value_ref = unsafe { LLVMAddFunction(self.module, name.as_ptr(), fn_type.type_ref()) };
        FnValue::new(value_ref)
    }

    pub fn get_fn(&'llvm self, name: &str) -> Option<FnValue<'llvm>> {
        let name = SmallCStr::try_from(name)
            .expect("Failed to convert 'name' argument to small C string!");

        let value_ref = unsafe { LLVMGetNamedFunction(self.module, name.as_ptr()) };

        (!value_ref.is_null()).then(|| FnValue::new(value_ref))
    }

    pub fn append_basic_block(&'llvm self, fn_value: FnValue<'llvm>) -> BasicBlock<'llvm> {
        let block = unsafe {
            LLVMAppendBasicBlockInContext(
                self.ctx,
                fn_value.value_ref(),
                b"block\0".as_ptr().cast(),
            )
        };
        assert!(!block.is_null());

        BasicBlock::new(block)
    }

    pub fn create_basic_block(&self) -> BasicBlock<'llvm> {
        let block = unsafe { LLVMCreateBasicBlockInContext(self.ctx, b"block\0".as_ptr().cast()) };
        assert!(!block.is_null());

        BasicBlock::new(block)
    }
}

impl Drop for Module {
    fn drop(&mut self) {
        unsafe {
            if !self.module.is_null() {
                LLVMDisposeModule(self.module);
            }

            LLVMOrcDisposeThreadSafeContext(self.tsctx);
        }
    }
}
