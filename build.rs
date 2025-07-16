use std::env;
use std::path::PathBuf;

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

    // Only link the library on Raspberry Pi or when explicitly requested
    if cfg!(target_arch = "arm") || cfg!(target_arch = "aarch64") || env::var("DAQHATS_FORCE_LINK").is_ok() {
        println!("cargo:rustc-link-search=native=lib");
        println!("cargo:rustc-link-lib=static=daqhats");
    }
}
