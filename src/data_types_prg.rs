use std::io;
use std::process::exit;

fn main() {
    let ar = [1 , 2, 3, 4, 5];
    println!("I have an array: {:?}", ar);
    println!("Input index to access an element: ");

    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Cannot read input");

    let user_input = user_input.trim();
    let index: usize = match user_input.parse() {
        Ok(n) => n,
        Err(e) => {
            println!("Invalid number '{}', error '{}'!", user_input, e);
            exit(1);
        }
    };

    println!("The {}'th element is {}", index, ar[index]);
    // When index == 12 and compiled to Debug, then ar[index] throws error: index out of bounds: the len is 5 but the index is 12
}
