# LightGBM Rust binding


This binding currently works for lightgbm 3.1.1.

The library is currently under heavy development. APIs are subject to change and error-prone. 

# Usage

* Build your own lightgbm according to the instruction at [here](https://lightgbm.readthedocs.io/en/latest/Installation-Guide.html).
* Copy `lib_lightgbm.so` to your library search path. On linux it would be `/usr/local/lib`. Or simply
  set `LD_LIBRARY_PATH` to the folder contains `lib_lightgbm.so`.
* Add this to your project's `Cargo.toml`.
  ```toml
  lightgbm = { version = "0.0.1", git = "https://github.com/dovahcrow/lightgbm-rs" }
  ```

# Documentation

There's no documentation available yet. But you can take a look at the example folder.
