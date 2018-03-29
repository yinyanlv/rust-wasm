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

#[test]
fn test_add() {
    assert_eq!(add(1, 2), 3);
}

#[test]
fn test_subtract() {
    assert_eq!(subtract(1, 2), -1);
}

#[test]
fn test_multiply() {
    assert_eq!(multiply(1, 2), 2);
}

#[test]
fn test_divide() {
    assert_eq!(divide(1, 2), 0);
}

#[test]
fn test_modulo() {
    assert_eq!(modulo(1, 2), 1);
}

#[test]
fn test_concat_number() {
    assert_eq!(concat_number(1, 2), 12);
}