use std::env;
use std::path::PathBuf;

fn main() {
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    
    // Select the correct library path based on the target architecture
    let lib_dir = match target_arch.as_str() {
        "x86_64" => "libs/x86_64",
        "aarch64" => "libs/aarch64",
        "arm" => "libs/armv7",
        "x86" => "libs/x86",
        _ => "libs/x86_64", // Default to x86_64 for development
    };

    let project_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let abs_lib_path = PathBuf::from(project_dir).join(lib_dir);

    println!("cargo:rustc-link-search=native={}", abs_lib_path.display());
    println!("cargo:rustc-link-lib=dylib=vosk");
    
    // Re-run if build.rs or the libs change
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=libs/");
}
