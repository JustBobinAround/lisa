use std::convert::TryFrom;
mod lexer;
mod parser;

#[cfg(feature = "llvm")]
mod llvm;
#[cfg(feature = "llvm")]
mod codegen;
#[cfg(feature = "llvm")]
use codegen::Codegen;

use lexer::{Lexer, Token};
use parser::{Parser, PrototypeAST};
use std::{collections::HashMap, str::Chars};

fn parsing_loop<I>(mut parser: Parser<I>)
where
    I: Iterator<Item = char>,
{
    #[cfg(feature = "llvm")]
    {
        let mut module = llvm::Module::new();

        // Create a new JIT, based on the LLVM LLJIT.
        let jit = llvm::LLJit::new();

        // Enable lookup of dynamic symbols in the current process from the JIT.
        jit.enable_process_symbols();

        // Keep track of prototype names to their respective ASTs.
        //
        // This is useful since we jit every function definition into its own LLVM module.
        // To allow calling functions defined in previous LLVM modules we keep track of their
        // prototypes and generate IR for their declarations when they are called from another module.
        let mut fn_protos: HashMap<String, PrototypeAST> = HashMap::new();

        // When adding an IR module to the JIT, it will hand out a ResourceTracker. When the
        // ResourceTracker is dropped, the code generated from the corresponding module will be removed
        // from the JIT.
        //
        // For each function we want to keep the code generated for the last definition, hence we need
        // to keep their ResourceTracker alive.
        let mut fn_jit_rt: HashMap<String, llvm::ResourceTracker> = HashMap::new();

        loop {
            match parser.current_token() {
                Token::EOF=> break,
                _ => match parser.parse_top_level_expr() {
                    Ok(func) => {
                        println!(">>>{:#?}", func);
                        #[cfg(feature = "llvm")]
                        {
                            println!("Parse top-level expression");
                            if let Ok(func) = Codegen::compile(&module, &mut fn_protos, Either::B(&func)) {
                                func.dump();

                                // Add module to the JIT. Code will be removed when `_rt` is dropped.
                                let _rt = jit.add_module(module);

                                // Initialize a new module.
                                module = llvm::Module::new();

                                // Call the top level expression.
                                let fp = jit.find_symbol::<unsafe extern "C" fn() -> i64>("__anon_expr");
                                unsafe {
                                    println!("Evaluated to {}", fp());
                                }
                            }
                        }
                    }
                    Err(err) => {
                        eprintln!("Error: {:?}", err);
                        parser.next_token();
                    }
                },
            }
        }
    }

}

fn run<I>(lexer: Lexer<I>)
where
    I: Iterator<Item = char>,
{
    let mut parser = Parser::new(lexer);

    #[cfg(feature = "llvm")]
    {
        llvm::initialize_native_taget();
    }

    parsing_loop(parser);


    #[cfg(feature = "llvm")]
    llvm::shutdown();
}




/// Fixed size of [`SmallCStr`] including the trailing `\0` byte.
pub const SMALL_STR_SIZE: usize = 16;

/// Small C string on the stack with fixed size [`SMALL_STR_SIZE`].
///
/// This is specially crafted to interact with the LLVM C API and get rid of some heap allocations.
#[derive(Debug, PartialEq)]
pub struct SmallCStr([u8; SMALL_STR_SIZE]);

impl SmallCStr {
    /// Create a new C string from `src`.
    /// Returns [`None`] if `src` exceeds the fixed size or contains any `\0` bytes.
    pub fn new<T: AsRef<[u8]>>(src: &T) -> Option<SmallCStr> {
        let src = src.as_ref();
        let len = src.len();

        // Check for \0 bytes.
        let contains_null = unsafe { !libc::memchr(src.as_ptr().cast(), 0, len).is_null() };

        if contains_null || len > SMALL_STR_SIZE - 1 {
            None
        } else {
            let mut dest = [0; SMALL_STR_SIZE];
            dest[..len].copy_from_slice(src);
            Some(SmallCStr(dest))
        }
    }

    /// Return pointer to C string.
    pub const fn as_ptr(&self) -> *const libc::c_char {
        self.0.as_ptr().cast()
    }
}

impl TryFrom<&str> for SmallCStr {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        SmallCStr::new(&value).ok_or(())
    }
}

/// Either type, for APIs accepting two types.
pub enum Either<A, B> {
    A(A),
    B(B),
}

#[cfg(test)]
mod test {
    use super::{SmallCStr, SMALL_STR_SIZE};
    use std::convert::TryInto;

    #[test]
    fn test_create() {
        let src = "\x30\x31\x32\x33";
        let scs = SmallCStr::new(&src).unwrap();
        assert_eq!(&scs.0[..5], &[0x30, 0x31, 0x32, 0x33, 0x00]);

        let src = b"abcd1234";
        let scs = SmallCStr::new(&src).unwrap();
        assert_eq!(
            &scs.0[..9],
            &[0x61, 0x62, 0x63, 0x64, 0x31, 0x32, 0x33, 0x34, 0x00]
        );
    }

    #[test]
    fn test_contain_null() {
        let src = "\x30\x00\x32\x33";
        let scs = SmallCStr::new(&src);
        assert_eq!(scs, None);

        let src = "\x30\x31\x32\x33\x00";
        let scs = SmallCStr::new(&src);
        assert_eq!(scs, None);
    }

    #[test]
    fn test_too_large() {
        let src = (0..SMALL_STR_SIZE).map(|_| 'a').collect::<String>();
        let scs = SmallCStr::new(&src);
        assert_eq!(scs, None);

        let src = (0..SMALL_STR_SIZE + 10).map(|_| 'a').collect::<String>();
        let scs = SmallCStr::new(&src);
        assert_eq!(scs, None);
    }

    #[test]
    fn test_try_into() {
        let src = "\x30\x31\x32\x33";
        let scs: Result<SmallCStr, ()> = src.try_into();
        assert!(scs.is_ok());

        let src = (0..SMALL_STR_SIZE).map(|_| 'a').collect::<String>();
        let scs: Result<SmallCStr, ()> = src.as_str().try_into();
        assert!(scs.is_err());

        let src = (0..SMALL_STR_SIZE + 10).map(|_| 'a').collect::<String>();
        let scs: Result<SmallCStr, ()> = src.as_str().try_into();
        assert!(scs.is_err());
    }
}

fn main() {
    let input = r#" 2+2+3"#;

    let mut lexer: Lexer<Chars> = Lexer::new(
        input.chars()
    );
    run(lexer);

}
