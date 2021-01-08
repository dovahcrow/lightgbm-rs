# LightGBM Binding for Rust ![CI](https://github.com/dovahcrow/lightgbm-rs/workflows/lightgbm-rs%20CI/badge.svg)


This binding currently works for lightgbm 3.1.1.

The library is currently under heavy development. APIs are subject to change and error-prone. 

# Usage

* Make sure your machine has cmake, libstdc++ and libgomp installed.
* Add this to your project's `Cargo.toml`.
  ```toml
  lightgbm = { version = "0.0.1", git = "https://github.com/dovahcrow/lightgbm-rs" }
  ```

By default, the lightgbm library is static linked to your binary. If you'd like to use the dynamic lib,
set the dynamic feature of `lightgbm-rs`.

# Documentation

There's no documentation available yet. But you can take a look at the example folder.
