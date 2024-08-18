use llvm_sys::{core::LLVMGetBasicBlockParent, prelude::LLVMBasicBlockRef};

use std::marker::PhantomData;

use super::FnValue;

#[derive(Copy, Clone)]
pub struct BasicBlock<'llvm>(LLVMBasicBlockRef, PhantomData<&'llvm ()>);

impl<'llvm> BasicBlock<'llvm> {
    pub(super) fn new(bb_ref: LLVMBasicBlockRef) -> BasicBlock<'llvm> {
        assert!(!bb_ref.is_null());
        BasicBlock(bb_ref, PhantomData)
    }

    #[inline]
    pub(super) fn bb_ref(&self) -> LLVMBasicBlockRef {
        self.0
    }

    pub fn get_parent(&self) -> FnValue<'llvm> {
        let value_ref = unsafe { LLVMGetBasicBlockParent(self.bb_ref()) };
        assert!(!value_ref.is_null());

        FnValue::new(value_ref)
    }
}
