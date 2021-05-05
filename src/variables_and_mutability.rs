extern crate myrust;
use self::myrust::compilation_error;

#[allow(unused)]
fn main() {
    let x = 1; // no explicit type is put ("variable is not annotated with type")
    println!("let x, x is {}", x);
    compilation_error!(
        x = 2;  // cannot assign twice to immutable variable
    );
    let x = 2; // shadowing, explicitly redeclare is OK
    println!("another let x is {}", x);

    // shadowing name can become mutable
    let mut x = "Hello!"; // shadowing lets to change the type of the name
    println!("let x to be a string, value is {}", x);
    x = ""; // ok, 'x' is mutable now, warning: value assigned to `x` is never read

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

    // compilation_error:
    // let x_mut: mut i32 = 1; // error: expected type, found keyword `mut`
    //            ^^^ expected type

    let mut x_mut: i32 = 1;
    let x_ref: &mut i32 = &mut x_mut; // 'x_ref' is immutable
    compilation_error!(
        x_ref = &2; // mismatched types, expected mutable reference `&mut i32`, found reference `&i32`
        //      ^^ types differ in mutability
    );
    compilation_error!(
        x_ref = &mut 2; // cannot assign twice to immutable variable `x_ref`
    );
    let mut x_ref: &mut i32 = &mut x_mut;
    x_ref = &mut 2; //  creates a temporary which is freed while still in use, 'x_ref' becomes invalid
    // compilation_error:
    // println!("Values of variable({}) and reference({})", x_mut, x_ref); // temporary value dropped while borrowed
    let mut x_ref: &i32 = &mut x_mut;
    x_ref = &2;
    println!("Values of variable({}) and reference({})", x_mut, x_ref); // 1 2
    compilation_error!(
        *x_ref = 2; // cannot assign to `*x_ref` which is behind a `&` reference, because 'x_ref' is of type '&i32' - immutable
    );
    {
        let mut x_ref: &mut i32 = &mut x_mut;
        *x_ref = 2; // Ok
        println!("Values of reference({})", x_ref); // 2
    }
    println!("Values of variable({})", x_mut); // 2
    {
        let x_ref: &mut i32 = &mut x_mut; // 'x_ref' is immutable
        *x_ref = 3; // Ok                    but it points to a mutable value
        println!("Values of reference({})", x_ref); // 3
    }
    println!("Values of variable({})", x_mut); // 3

    // raw identifier
    // compilation_error
    // let match = 2; // expected identifier, found keyword `match`
    let r#match = 2; // Ok, raw identifier
    let x = r#match + 3; // usage
}
