macro_rules! compilation_error {
    ($s:stmt $(;)?) => {}
}

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
    } else { // else branch is mandatory, when it is used as expression
        0
    };

    // 'match' is an expression
    let s = "1";
    let x = match s {
        "0" => 1,
        "1" => 2,
        _ => 0
    };
}