use llvm_sys::{
    core::LLVMShutdown,
    error::{LLVMDisposeErrorMessage, LLVMErrorRef, LLVMGetErrorMessage},
    target::{
        LLVM_InitializeNativeAsmParser, LLVM_InitializeNativeAsmPrinter,
        LLVM_InitializeNativeTarget,
    },
};

use std::ffi::CStr;

mod basic_block;
mod builder;
mod lljit;
mod module;
mod pass_manager;
mod type_;
mod value;

pub use basic_block::BasicBlock;
pub use builder::IRBuilder;
pub use lljit::{LLJit, ResourceTracker};
pub use module::Module;
pub use pass_manager::FunctionPassManager;
pub use type_::Type;
pub use value::{FnValue, PhiValue, Value};

struct Error<'llvm>(&'llvm mut libc::c_char);

impl<'llvm> Error<'llvm> {
    fn from(err: LLVMErrorRef) -> Option<Error<'llvm>> {
        (!err.is_null()).then(|| Error(unsafe { &mut *LLVMGetErrorMessage(err) }))
    }

    fn as_str(&self) -> &str {
        unsafe { CStr::from_ptr(self.0) }
            .to_str()
            .expect("Expected valid UTF8 string from LLVM API")
    }
}

impl Drop for Error<'_> {
    fn drop(&mut self) {
        unsafe {
            LLVMDisposeErrorMessage(self.0 as *mut libc::c_char);
        }
    }
}

pub fn initialize_native_taget() {
    unsafe {
        assert_eq!(LLVM_InitializeNativeTarget(), 0);
        assert_eq!(LLVM_InitializeNativeAsmParser(), 0);
        assert_eq!(LLVM_InitializeNativeAsmPrinter(), 0);
    }
}

pub fn shutdown() {
    unsafe {
        LLVMShutdown();
    };
}
