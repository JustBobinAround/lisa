use llvm_sys::orc2::{
    lljit::{
        LLVMOrcCreateLLJIT, LLVMOrcLLJITAddLLVMIRModuleWithRT, LLVMOrcLLJITGetGlobalPrefix,
        LLVMOrcLLJITGetMainJITDylib, LLVMOrcLLJITLookup, LLVMOrcLLJITRef,
    },
    LLVMOrcCreateDynamicLibrarySearchGeneratorForProcess, LLVMOrcDefinitionGeneratorRef,
    LLVMOrcJITDylibAddGenerator, LLVMOrcJITDylibCreateResourceTracker, LLVMOrcJITDylibRef,
    LLVMOrcReleaseResourceTracker, LLVMOrcResourceTrackerRef, LLVMOrcResourceTrackerRemove,
};

use std::convert::TryFrom;
use std::marker::PhantomData;

use super::{Error, Module};
use crate::SmallCStr;

pub trait JitFn {}

impl JitFn for unsafe extern "C" fn() -> i64 {}

pub struct LLJit {
    jit: LLVMOrcLLJITRef,
    dylib: LLVMOrcJITDylibRef,
}

impl LLJit {
    pub fn new() -> LLJit {
        let (jit, dylib) = unsafe {
            let mut jit = std::ptr::null_mut();
            let err = LLVMOrcCreateLLJIT(
                &mut jit as _,
                std::ptr::null_mut(), /* builder: nullptr -> default */
            );

            if let Some(err) = Error::from(err) {
                panic!("Error: {}", err.as_str());
            }

            let dylib = LLVMOrcLLJITGetMainJITDylib(jit);
            assert!(!dylib.is_null());

            (jit, dylib)
        };

        LLJit { jit, dylib }
    }

    pub fn add_module(&self, module: Module) -> ResourceTracker<'_> {
        let tsmod = module.into_raw_thread_safe_module();

        let rt = unsafe {
            let rt = LLVMOrcJITDylibCreateResourceTracker(self.dylib);
            let err = LLVMOrcLLJITAddLLVMIRModuleWithRT(self.jit, rt, tsmod);

            if let Some(err) = Error::from(err) {
                panic!("Error: {}", err.as_str());
            }

            rt
        };

        ResourceTracker::new(rt)
    }

    pub fn find_symbol<F: JitFn>(&self, sym: &str) -> F {
        let sym =
            SmallCStr::try_from(sym).expect("Failed to convert 'sym' argument to small C string!");

        unsafe {
            let mut addr = 0u64;
            let err = LLVMOrcLLJITLookup(self.jit, &mut addr as _, sym.as_ptr());

            if let Some(err) = Error::from(err) {
                panic!("Error: {}", err.as_str());
            }

            debug_assert_eq!(core::mem::size_of_val(&addr), core::mem::size_of::<F>());
            std::mem::transmute_copy(&addr)
        }
    }

    pub fn enable_process_symbols(&self) {
        unsafe {
            let mut proc_syms_gen: LLVMOrcDefinitionGeneratorRef = std::ptr::null_mut();
            let err = LLVMOrcCreateDynamicLibrarySearchGeneratorForProcess(
                &mut proc_syms_gen as _,
                self.global_prefix(),
                None,                 /* filter */
                std::ptr::null_mut(), /* filter ctx */
            );

            if let Some(err) = Error::from(err) {
                panic!("Error: {}", err.as_str());
            }

            LLVMOrcJITDylibAddGenerator(self.dylib, proc_syms_gen);
        }
    }

    fn global_prefix(&self) -> libc::c_char {
        unsafe { LLVMOrcLLJITGetGlobalPrefix(self.jit) }
    }
}

pub struct ResourceTracker<'jit>(LLVMOrcResourceTrackerRef, PhantomData<&'jit ()>);

impl<'jit> ResourceTracker<'jit> {
    fn new(rt: LLVMOrcResourceTrackerRef) -> ResourceTracker<'jit> {
        assert!(!rt.is_null());
        ResourceTracker(rt, PhantomData)
    }
}

impl Drop for ResourceTracker<'_> {
    fn drop(&mut self) {
        unsafe {
            let err = LLVMOrcResourceTrackerRemove(self.0);

            if let Some(err) = Error::from(err) {
                panic!("Error: {}", err.as_str());
            }

            LLVMOrcReleaseResourceTracker(self.0);
        };
    }
}
