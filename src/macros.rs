// macros: declarative(function-like) | procedural(custom derive, attribute-like, function-like)
// declarative - if pattern matches -> replace code
// procedural  - process given token stream, emit token stream

macro_rules! my_vec {
    // If the pattern matches, the associated block of code will be emitted
    // '$()' - grouping code for '*'
    // '$x:expr' matches an expression and gives it a name 'x'.

    // "pattern"
    ( $( $x:expr ),* ) => { // first (and single) arm of the macro
        {
            let mut temp_vec = Vec::new();
            // the code in '$()*' is emitted for every match of pattern '$()'
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

// TODO: add procedural macros

fn main() {
    let v = my_vec!(1, 2, 3 , 4);
    println!("vector is {:?}", v); // [1, 2, 3, 4]
}
