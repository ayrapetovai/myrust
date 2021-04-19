
// rust: recoverable (Result<T,E>) and unrecoverable (panic!) errors
// When the panic! macro executes, your program will print a failure message, unwind and clean up the stack, and then quit
use std::fs::File;
use std::io::{ErrorKind, Read};
use std::error::Error;

fn ask_for_panic(message: &str) -> bool {
    println!("{} y/n:", message);
    let mut answer = String::new();
    std::io::stdin().read_line(&mut answer).expect("io");
    answer.trim() == "y"
}

fn main() {
    {
        if ask_for_panic("Call 'panic!' macro?") {
            panic!("OMG!");
            // thread 'main' panicked at 'OMG!', src/error_handling.rs:11:9
        }
    }
    {
        let a: [i32; 4] = [1, 2, 3, 4];
        let index_out_of_bounds = a.len();
        if ask_for_panic("Call index out of bounds?") {
            a[index_out_of_bounds];
            // thread 'main' panicked at 'index out of bounds: the len is 4 but the index is 4', src/error_handling.rs:32:13
        }
    }
// Recoverable Errors with Result
    {
        if ask_for_panic("Read a file?") {
            let f = File::open("file.txt");
            let f = match f {
                Ok(file) => file,
                Err(e) => panic!("Cannot open file {:?}", e)
            };
            // thread 'main' panicked at 'Cannot open file Os { code: 2, kind: NotFound, message: "No such file or directory" }', src/error_handling.rs:34:27
        }
    }
    {
        let file_name = "file.txt";
        let f = File::open(file_name);
        let f = match f {
            Ok(file) => file,
            Err(e) => match e.kind() {
                ErrorKind::NotFound => match File::create(file_name) {
                    Ok(file) => file,
                    Err(e) => panic!("Cannot create file {}", file_name)
                },
                other_error => panic!("Got other error {:?}", other_error)
            }
        };
    }
// Shortcuts for Panic on Error: unwrap and expect
    {
        let file_name = "file.txt";
        let f = File::open(file_name).unwrap(); // do 'match', return value of Ok variant of 'panic!' on Err variant of Result
        let f = File::open(file_name).expect("message"); // do 'match', return value of Ok variant of 'panic!' on Err variant of Result
    }
// Propagating Errors
    {
        fn make_an_error() -> Result<(), String> { // 'make_an_error' propagates an error
            Err(String::from("this is a message"))
        }
        match make_an_error() {
            Err(message) => println!("message is '{}'", message),
            Ok(_) => {}
        }
    }
    {
        // propagate error with '?'
        fn read_file_to_string() -> Result<String, Box<dyn Error>> { // Any error 'Box<dyn Error>'
            let mut buf = String::new();
            // caned calls wih '?.'
            let bytes_read = File::open("file.txt")?.read_to_string(&mut buf)?; // enclosing function must return 'Result' for '?' to work
            /* same as code below
            match ... {
                ...,
                Err(e) => return Err(e)
            }
             */
            println!("Bytes read {}, content: {}", bytes_read, buf);
            Ok(buf)
        }
        read_file_to_string();
    }
}
