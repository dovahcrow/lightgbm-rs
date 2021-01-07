test +ARGS="":
	cargo test --features ci {{ARGS}}

iris:
    cargo run --features ci --example iris

gen-binding:
    echo "#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, unused)]" > src/bindings.rs
    bindgen src/includes/c_api.hpp >> src/bindings.rs