extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    add_search_path();
    add_llvm_path();

    println!("cargo:rustc-link-lib=_lightgbm");
    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=src/c_api.hpp");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("src/c_api.hpp")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_path.join("bindings.rs")).expect("Couldn't write bindings!");
}

#[cfg(not(target_os = "windows"))]
fn add_search_path() {
    for path in std::env::var("LD_LIBRARY_PATH").unwrap_or_else(|_| "".to_string()).split(":") {
        if path.trim().len() == 0 {
            continue;
        }
        println!("cargo:rustc-link-search=native={}", path);
    }
}

#[cfg(target_os = "windows")]
fn add_search_path() {
    for path in std::env::var("PATH").unwrap_or_else(|_| "".to_string()).split(";") {
        if path.trim().len() == 0 {
            continue;
        }
        println!("cargo:rustc-link-search=native={}", path);
    }
}

fn add_llvm_path() {
    if let Some(llvm_config_path) = option_env!("LLVM_CONFIG_PATH") {
        println!("cargo:rustc-env=LLVM_CONFIG_PATH={}", llvm_config_path);
    }
}
