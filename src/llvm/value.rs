#![allow(unused)]

use llvm_sys::{
    analysis::{LLVMVerifierFailureAction, LLVMVerifyFunction},
    core::{
        LLVMAddIncoming, LLVMAppendExistingBasicBlock, LLVMCountBasicBlocks, LLVMCountParams,
        LLVMDumpValue, LLVMGetParam, LLVMGetValueKind, LLVMGetValueName2, LLVMGlobalGetValueType,
        LLVMIsAFunction, LLVMIsAPHINode, LLVMSetValueName2, LLVMTypeOf,
    },
    prelude::LLVMValueRef,
    LLVMTypeKind, LLVMValueKind,
};
use std::ffi::CStr;
use std::marker::PhantomData;
use std::ops::Deref;

use super::BasicBlock;
use super::Type;

#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct Value<'llvm>(LLVMValueRef, PhantomData<&'llvm ()>);

impl<'llvm> Value<'llvm> {
    pub(super) fn new(value_ref: LLVMValueRef) -> Self {
        assert!(!value_ref.is_null());
        Value(value_ref, PhantomData)
    }

    #[inline]
    pub(super) fn value_ref(&self) -> LLVMValueRef {
        self.0
    }

    pub(super) fn kind(&self) -> LLVMValueKind {
        unsafe { LLVMGetValueKind(self.value_ref()) }
    }

    pub(super) fn is_function(&self) -> bool {
        let cast = unsafe { LLVMIsAFunction(self.value_ref()) };
        !cast.is_null()
    }

    pub(super) fn is_phinode(&self) -> bool {
        let cast = unsafe { LLVMIsAPHINode(self.value_ref()) };
        !cast.is_null()
    }

    pub fn dump(&self) {
        unsafe { LLVMDumpValue(self.value_ref()) };
    }

    pub fn type_of(&self) -> Type<'llvm> {
        let type_ref = unsafe { LLVMTypeOf(self.value_ref()) };
        Type::new(type_ref)
    }

    pub fn set_name(&self, name: &str) {
        unsafe { LLVMSetValueName2(self.value_ref(), name.as_ptr().cast(), name.len()) };
    }

    pub fn get_name(&self) -> &'llvm str {
        let name = unsafe {
            let mut len: libc::size_t = 0;
            let name = LLVMGetValueName2(self.0, &mut len as _);
            assert!(!name.is_null());

            CStr::from_ptr(name)
        };

        name.to_str()
            .expect("Expected valid UTF8 string from LLVM API")
    }

    pub fn is_f64(&self) -> bool {
        self.type_of().kind() == LLVMTypeKind::LLVMDoubleTypeKind
    }

    pub fn is_int(&self) -> bool {
        self.type_of().kind() == LLVMTypeKind::LLVMIntegerTypeKind
    }
}

#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct FnValue<'llvm>(Value<'llvm>);

impl<'llvm> Deref for FnValue<'llvm> {
    type Target = Value<'llvm>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'llvm> FnValue<'llvm> {
    pub(super) fn new(value_ref: LLVMValueRef) -> Self {
        let value = Value::new(value_ref);
        debug_assert!(
            value.is_function(),
            "Expected a fn value when constructing FnValue!"
        );

        FnValue(value)
    }

    pub fn fn_type(&self) -> Type<'llvm> {
        // https://github.com/llvm/llvm-project/issues/72798
        let type_ref = unsafe { LLVMGlobalGetValueType(self.value_ref()) };
        Type::new(type_ref)
    }

    pub fn args(&self) -> usize {
        unsafe { LLVMCountParams(self.value_ref()) as usize }
    }

    pub fn arg(&self, idx: usize) -> Value<'llvm> {
        assert!(idx < self.args());

        let value_ref = unsafe { LLVMGetParam(self.value_ref(), idx as libc::c_uint) };
        Value::new(value_ref)
    }

    pub fn basic_blocks(&self) -> usize {
        unsafe { LLVMCountBasicBlocks(self.value_ref()) as usize }
    }

    pub fn append_basic_block(&self, bb: BasicBlock<'llvm>) {
        unsafe {
            LLVMAppendExistingBasicBlock(self.value_ref(), bb.bb_ref());
        }
    }

    pub fn verify(&self) -> bool {
        unsafe {
            LLVMVerifyFunction(
                self.value_ref(),
                LLVMVerifierFailureAction::LLVMPrintMessageAction,
            ) == 0
        }
    }
}

#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct PhiValue<'llvm>(Value<'llvm>);

impl<'llvm> Deref for PhiValue<'llvm> {
    type Target = Value<'llvm>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'llvm> PhiValue<'llvm> {
    pub(super) fn new(value_ref: LLVMValueRef) -> Self {
        let value = Value::new(value_ref);
        debug_assert!(
            value.is_phinode(),
            "Expected a phinode value when constructing PhiValue!"
        );

        PhiValue(value)
    }

    pub fn add_incoming(&self, ival: Value<'llvm>, ibb: BasicBlock<'llvm>) {
        debug_assert_eq!(
            ival.type_of().kind(),
            self.type_of().kind(),
            "Type of incoming phi value must be the same as the type used to build the phi node."
        );

        unsafe {
            LLVMAddIncoming(
                self.value_ref(),
                &mut ival.value_ref() as _,
                &mut ibb.bb_ref() as _,
                1,
            );
        }
    }
}
