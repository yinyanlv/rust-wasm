//! rustup target add wasm32-unknown-unknown
//! rustc +stable --target wasm32-unknown-unknown -O ./src/lib.rs --out-dir ./target --crate-type=cdylib
//! cargo install --git https://github.com/alexcrichton/wasm-gc
//! wasm-gc ./target/lib.wasm ./target/lib.min.wasm

#[no_mangle]
pub extern fn add(a: i32, b: i32) -> i32 {

    a + b
}

#[no_mangle]
pub extern fn subtract(a: i32, b: i32) -> i32 {

    a - b
}

#[no_mangle]
pub extern fn multiply(a: i32, b: i32) -> i32 {

    a * b
}

#[no_mangle]
pub extern fn divide(a: i32, b: i32) -> i32 {

    a / b
}