use llvm_sys::{
    core::{LLVMConstInt,LLVMConstReal, LLVMDumpType, LLVMGetTypeKind},
    prelude::LLVMTypeRef,
    LLVMTypeKind,
};

use std::marker::PhantomData;

use super::Value;

/// Wrapper for a LLVM Type Reference.
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct Type<'llvm>(LLVMTypeRef, PhantomData<&'llvm ()>);

impl<'llvm> Type<'llvm> {
    /// Create a new Type instance.
    ///
    /// # Panics
    ///
    /// Panics if `type_ref` is a null pointer.
    pub(super) fn new(type_ref: LLVMTypeRef) -> Self {
        assert!(!type_ref.is_null());
        Type(type_ref, PhantomData)
    }

    /// Get the raw LLVM type reference.
    #[inline]
    pub(super) fn type_ref(&self) -> LLVMTypeRef {
        self.0
    }

    /// Get the LLVM type kind for the given type reference.
    pub(super) fn kind(&self) -> LLVMTypeKind {
        unsafe { LLVMGetTypeKind(self.type_ref()) }
    }

    /// Dump the LLVM Type to stdout.
    pub fn dump(&self) {
        unsafe { LLVMDumpType(self.type_ref()) };
    }

    /// Get a value reference representing the const `f64` value.
    ///
    /// # Panics
    ///
    /// Panics if LLVM API returns a `null` pointer.
    pub fn const_i64(self, n: i64) -> Value<'llvm> {
        debug_assert_eq!(
            self.kind(),
            LLVMTypeKind::LLVMIntegerTypeKind,
            "Expected a int type when creating const i64 value!"
        );
        let sign_extend = if n > 0 {
            1
        } else {
            0
        };
        let value_ref = unsafe { LLVMConstInt(self.type_ref(), n as u64, sign_extend) };
        Value::new(value_ref)
    }
}
