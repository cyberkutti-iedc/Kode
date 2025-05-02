use std::process::Command;

fn main() {
    // Get LLVM configuration
    let llvm_config = std::env::var("LLVM_CONFIG").unwrap_or_else(|_| "llvm-config-14".to_string());
    
    // Get LLVM linking flags
    let output = Command::new(&llvm_config)
        .arg("--libs")
        .arg("--ldflags")
        .arg("--system-libs")
        .output()
        .expect("Failed to execute llvm-config");
    
    let flags = String::from_utf8_lossy(&output.stdout);
    
    // Parse the flags and add them as link arguments
    for flag in flags.split_whitespace() {
        if flag.starts_with("-l") {
            println!("cargo:rustc-link-lib={}", &flag[2..]);
        } else if flag.starts_with("-L") {
            println!("cargo:rustc-link-search=native={}", &flag[2..]);
        }
    }

    // Add LLVM library path
    let output = Command::new(&llvm_config)
        .arg("--libdir")
        .output()
        .expect("Failed to execute llvm-config");
    
    let libdir = String::from_utf8_lossy(&output.stdout).trim().to_string();
    println!("cargo:rustc-link-search=native={}", libdir);
}