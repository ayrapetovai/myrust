use std::mem::size_of;
use std::cmp::Ordering;

macro_rules! debug_runtime_error {
    ($s:stmt $(;)?) => {}
}

macro_rules! compilation_error {
    ($s:stmt $(;)?) => {}
}

// Each signed variant can store numbers from -(2^(n - 1)) to 2^(n - 1) - 1 inclusive

// The isize and usize types depend on the kind of computer your program is running on:
// 64 bits if you’re on a 64-bit architecture and
// 32 bits if you’re on a 32-bit architecture.

#[allow(unused)]
fn main() {
// integers
    let decimal = 92; // literal without postfix is i32 by default
    let hex = 0xff;
    let octal = 0o2571;
    let binary = 0b1010_1001; // '_' is a visual separator
    let byte = b'A'; // byte: u8

    // use typename as postfix to specify number literal type
    let an_unsigned_64bit_number = 92u64;
    let bigest_integer = 1u128;

    let l = "".len(); // l: usize; isaze and usize for indexing some collection

    let max_8bit_value: i8 = i8::MAX;
    println!("signed 8-bit max value is {}", max_8bit_value);

    // When compiling in debug mode, Rust includes checks for integer overflow that cause the program to panic at runtime
    debug_runtime_error!(
        let var_to_overflow = max_8bit_value + 1; // error: this arithmetic operation will overflow, attempt to compute `i8::MAX + 1_i8`, which would overflow
    );

    // whatever, still operating on i8
    debug_runtime_error!(
        let var_to_overflow: i16 = (max_8bit_value + 1) as i16; // error: attempt to add with overflow
    );

    // now operating on i16
    let var_not_to_overflow = max_8bit_value as i16 + 1; // now, OK
    println!("i8::MAX as i16 + 1 is {}", var_not_to_overflow); // 128

    let a: i64 = 1;
    let b: i32 = 2;
    // operator + with according types is chosen by the first argument
    compilation_error!(
        let c = a + b; // c: i64, first argument is a: i64, error: mismatched types, expected `i64`, found `i32`
    );
    compilation_error!(
        let d = b + a; // d: i32, first argument is b: i32, error: mismatched types, expected `i32`, found `i64`
    );

// floating points
    let x = 3.0f32; // f32, single precision
    let y = 3.0;    // f64, double precision

    // addition
    let sum = 5 + 10; // : i32

    // subtraction
    let difference = 95.5 - 4.3; // : f46

    // multiplication
    let product = 4 * 30; // : i32

    // division
    let quotient = 56.7 / 32.2; // : f64

    // remainder
    let remainder = 43 % 5; // : i32

// boolean
    let size_of_bool = size_of::<bool>();
    println!("size of bool is {}", size_of_bool); // 1

// characters
    // Unicode Scalar Values range from U+0000 to U+D7FF and U+E000 to U+10FFFF inclusive.
    let ch: char = 'Я';
    println!("Unicode character {}", ch);
    println!("char size is {}", size_of::<char>()); // 4

// Compound types: tuple (elements of arbitrary types) and arrays (elements of the same type); both of fixed length
    let tup: (i8, i16, i32) = (i8::MAX, i16::MIN, f32::MAX_10_EXP);
    compilation_error!(
        println!("strange tuple (i8, i16, i32) is {}", tup); // (i8, i16, i32)` doesn't implement `std::fmt::Display
    );
    // destructuring tuple for three (immutable) variables
    let (x, y, z) = tup;
    println!("strange tuple (i8, i16, i32) is ({}, {}, {})", x, y, z);

    let tup = (1, 2.0);
    println!("but we can print tuple by accessing it's elements via '.index'! {} {}", tup.0, tup.1);

    let a = [1, 2, 3, 4, 5]; // a: [i32; 5] - an array of five 32-but integers, array size is fixed
    compilation_error!(
        println!("array {}", a); // error: doesn't implement `std::fmt::Display
    );
    println!("print array with '{{:?}}', got {:?}", a);

    // "fixed size" means that size if a part of a type
    compilation_error!(
        let a: [i32; 3] = [1, 2, 3, 4]; // error: mismatched types; expected an array with a fixed size of 3 elements, found one with 4 elements
    );

    let a = [3; 5]; // array of five elements, all are 3
    let b = [3, 3, 3, 3, 3];
    println!("compare two arrays... result {}", match a.cmp(&b) { Ordering::Equal => "equal", _ => "not equal" }); // equal

    // array element access with [] operator
    let first = a[0];
    let second = a[1];
}
