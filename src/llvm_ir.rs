use std::io::Write;
use std::process::{Command, Stdio};

pub fn compile_to_bin(name: &String, llvm_ir: &String) -> std::io::Result<()> {
    let child = Command::new("clang")
        .args(&["-x", "ir", "-o", name, "-"])
        .stdin(Stdio::piped())
        .spawn();

    let mut child = match child {
        Ok(child) => {
            child
        }
        Err(_) => {
            Command::new("clang-16")
                .args(&["-x", "ir", "-o", name, "-"])
                .stdin(Stdio::piped())
                .spawn()
                .expect("Failed to find clang binary")
        }
    };

    let stdin = child.stdin.as_mut().ok_or_else(|| std::io::Error::new(
        std::io::ErrorKind::Other,
        "Failed to open stdin for clang",
    ))?;

    stdin.write(llvm_ir.as_bytes())?;
    let output = child.wait()?;

    println!("Clang process exited with: {:?}", output);

    Ok(())
}
