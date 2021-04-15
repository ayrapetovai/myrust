macro_rules! compilation_error {
    ($s:stmt $(;)?) => {}
}

fn with_param(x: i32) {
    println!("Passed integer value is {}", x);
    compilation_error!(
        x = 13; // error: cannot assign to immutable argument `x`
    );
}

fn return_something() -> i32 {
    compilation_error!(
        let x = 2 // let construction is not an expression, it does not return anything
    );
    let x = 2; x // 'x' is an expression, this function returns x's value
    // 'x' on this new line is also OK
}

// return value by 'return'
fn return_something_else() -> i32 {
    return 13; // ';' may be omitted
}

///     0 | 1 | 1 | 2 | 3 | 5 | 8 | 13 | 21
fn fibonacci(n: i32) -> i32 {
    if n == 0 { 0 } else if n == 1 { 1 } else { fibonacci(n - 1) + fibonacci(n-2) }
}

fn main() {
    // rust support forward definition
    a_function();

    // declare function in scope of function
    inner_function();
    fn inner_function() {
        println!("Function, defined inside the main() function");
    }
    // cannot redefine function in the same scope
    compilation_error!(
        fn inner_function() {} // error: the name `inner_function` is defined multiple times
    );

    // call function with parameter, pass argument by value
    with_param(42);

    println!("Returned value {} and {}", return_something(), return_something_else()); // 2 and 13

    // implicit narrowing and "widing" of values are forbidden
    compilation_error!(
        with_param(42i16); // error: expected `i32`, found `i16`
    );

    println!("fibonacci(6) is {}", fibonacci(6));
}

// returns nothing... '()'
fn a_function() {
    println!("Hello function!");
    // inner_function() lives in the scope of main()
    compilation_error!(
        inner_function(); // cannot find function `inner_function` in this scope
    );
}
