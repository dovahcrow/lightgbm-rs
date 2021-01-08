fn main() {
    add_search_path();
    add_llvm_path();
    build_lightgbm();
}

#[cfg(feature = "static")]
fn build_lightgbm() {
    let dst = cmake::Config::new("LightGBM").define("BUILD_STATIC_LIB", "ON").build();
    println!("cargo:rustc-link-search={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=_lightgbm");
    println!("cargo:rustc-link-lib=stdc++");
    println!("cargo:rustc-link-lib=gomp");
}

#[cfg(feature = "dynamic")]
fn build_lightgbm() {
    let dst = cmake::Config::new("LightGBM").pic(true).build();
    println!("cargo:rustc-link-search={}/lib", dst.display());
    println!("cargo:rustc-link-lib=_lightgbm");
}

#[cfg(not(target_os = "windows"))]
fn add_search_path() {
    for path in std::env::var("LD_LIBRARY_PATH").unwrap_or_else(|_| "".to_string()).split(":") {
        if path.trim().len() == 0 {
            continue;
        }
        println!("cargo:rustc-link-search={}", path);
    }
}

#[cfg(target_os = "windows")]
fn add_search_path() {
    for path in std::env::var("PATH").unwrap_or_else(|_| "".to_string()).split(";") {
        if path.trim().len() == 0 {
            continue;
        }
        println!("cargo:rustc-link-search={}", path);
    }
}

fn add_llvm_path() {
    if let Some(llvm_config_path) = option_env!("LLVM_CONFIG_PATH") {
        println!("cargo:rustc-env=LLVM_CONFIG_PATH={}", llvm_config_path);
    }
}
