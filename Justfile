export LD_LIBRARY_PATH := justfile_directory() + "/LightGBM"

test +ARGS="":
	cargo test {{ARGS}}

iris:
    cargo run --example iris