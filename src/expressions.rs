macro_rules! compilation_error {
    ($s:stmt $(;)?) => {}
}

#[allow(unused)]
fn main() {
    // 'let ...' is not an expression
    // let y = (let x = 1); // compilation error: `let` expressions in this position are experimental

    // {} - code block is an expression
    let x = {
        let y = 2;
        y // variable name is an expression
    };

    // {} is an expression '()' void
    let x = {};

    // 'if' is an expression
    let x = if true {
        10
    } else { // "else-arm" is mandatory, when it is used to assign value to a variable
        0
    };

    if false {} // ok, no "else-arm" is mandatory

    // compilation error
    // if false 0; // no, we cannot replace '{ 0 }' with '0', "if ... {" is a part of syntax

    // 'x' is initialized but, rust forbids omitting "else-arm" when using 'if' as expression
    let mut x = 0;
    compilation_error!(
        x = if false { // error: `if` may be missing an `else` clause
            13
        };
    );

    let y = 8;
    let x = if y % 2 == 0 {
        "y divisible by 2"
    } else if y % 3 == 0 {
        "y divisible by 3"
    } else {
        "y..."
    };

    // error, 'if' and 'else' return values are of different types
    compilation_error!(
        let x = if true { 0 } else { "" }; // `if` and `else` have incompatible types
    );

    if true { 0; } else { ""; }; // OK, if and else return values are of the same type '()'

// return value from cycles by 'break ...'
    // 'loop' is not an expressions
    let x = loop {
        break 42; // works even with ';'
    };
    println!("loop {{}} returned {}", x); // 42

    // 'while' is not an expression, it "returns" '()'
    let a = [1, 5, 2, 3, 4];
    let mut i = 0;
    let x = while a[i] != 2 {
        i += 1
    };
    println!("while result is {:?}", x); // ()

    let a = [1, 2, 3, 4];
    for x in a.iter() {
        println!("array value {}", x);
        compilation_error!(
            x = 1;  // x: &i32, expected `&{integer}`, found integer
        );
    }

    for number in (1..4).rev() { // upper bound is not inclusive
        println!("for next in range: {}!", number);
    }

    // 'match' is an expression
    let s = "1";
    let x = match s {
        "0" => 1,
        "1" => 2,
        _ => 0
    };
}