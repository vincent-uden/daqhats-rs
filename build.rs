use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=include/");
    println!("cargo:rerun-if-changed=lib/");

    let bindings = bindgen::Builder::default()
        .header("include/daqhats.h")
        .clang_arg("-Iinclude")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // Compile and link the library on Raspberry Pi or when explicitly requested
    if cfg!(target_arch = "arm")
        || cfg!(target_arch = "aarch64")
        || env::var("DAQHATS_FORCE_LINK").is_ok()
    {
        compile_c_library();

        println!("cargo:rustc-link-search=native=lib/build");
        println!("cargo:rustc-link-lib=dylib=daqhats");
        println!("cargo:rustc-link-lib=dylib=gpiod");
        println!("cargo:rustc-link-lib=dylib=m");
        println!("cargo:rustc-link-lib=dylib=pthread");
    }
}

fn compile_c_library() {
    let output = Command::new("make")
        .current_dir("lib")
        .output()
        .expect("Failed to execute make command. Make sure you're on a Raspberry Pi with required dependencies.");

    if !output.status.success() {
        panic!(
            "Failed to compile C library:\nstdout: {}\nstderr: {}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }

    println!("C library compiled successfully");
}
