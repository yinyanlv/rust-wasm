use std::str::FromStr;

#[no_mangle]
pub fn add(a: i32, b: i32) -> i32 {

    a + b
}

#[no_mangle]
pub fn subtract(a: i32, b: i32) -> i32 {

    a - b
}

#[no_mangle]
pub fn multiply(a: i32, b: i32) -> i32 {

    a * b
}

#[no_mangle]
pub fn divide(a: i32, b: i32) -> i32 {

    a / b
}

#[no_mangle]
pub fn modulo(a: i32, b: i32) -> i32 {

    a % b
}

#[no_mangle]
pub fn concat_number(a: i32, b: i32) -> i32 {

    i32::from_str(&*(a.to_string() + &*b.to_string())).unwrap()
}