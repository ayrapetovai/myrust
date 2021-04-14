macro_rules! compilation_error {
    ($msg:stmt $(;)?) => {}
}

fn main() {
    let x = 1; // no explicit type is put
    println!("let x, x is {}", x);
    compilation_error!(
        x = 2;  // cannot assign twice to immutable variable, make mutable?
    );
    let x = 2; // shadowing, explicitly redeclare is OK
    println!("another let x is {}", x);

    // shadowing name can become mutable
    let mut x = "Hello!"; // shadowing lets to change the type of the name
    println!("let x to be a string, value is {}", x);
    x = ""; // ok, x is mutable now, warning: value assigned to `x` is never read

    // however, types are checked
    compilation_error!(
        x = x.len(); // mismatched types. Expected type did not match the received type: expected `&str`, found `usize`
    );

    const Y: i32 = 1; // explicitly typed, static, local scoped, substitutable
    // constant name cannot be shadowed
    compilation_error!(
        const Y: i32 = 2; // name hase already being defined
    );
    let x = 1; // warning: unused variable: `x`, if this is intentional, prefix it with an underscore: `_x`
    compilation_error!(
        const Z: i32 = 1 + x; // x is non-constant value (but immutable!)
    );
    const Z: i32 = 1 + Y; // ok, Y is constant value
}
