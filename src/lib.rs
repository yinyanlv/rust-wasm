//! rustup target add wasm32-unknown-unknown
//! rustc +stable --target wasm32-unknown-unknown -O ./src/lib.rs --out-dir ./target --crate-type=cdylib
//! cargo install wasm-gc
//! wasm-gc ./target/lib.wasm ./target/lib.min.wasm

pub mod math;