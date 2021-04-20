// TODO match
extern crate myrust;
use myrust::compilation_error;

#[derive(Debug)]
struct Verbose {
    id: i32
}

impl Verbose {
    fn new(new_id: i32) -> Verbose {
        Verbose { id: new_id }
    }
}

impl Drop for Verbose {
    fn drop(&mut self) {
        println!("Dropping {:?}", self);
    }
}

#[allow(unused)]
fn main() {
    {
        let some_u8_value = Some(0u8);
        match some_u8_value {
            Some(3) => println!("three"),
            _ => (),
        }
        // consider equivalent
        // match one pattern and ignore others
        let x = Some(42);
        if let Some(42) = x {
            println!("42!");
        }
    }
    {
        fn returns_none_or_number(number: bool) -> Option<i32> {
            if number {
                Some(12)
            } else {
                None
            }
        }
        // "destructing" Option
        fn test_and_print_plus_one(op: Option<i32>) {
            if let Some(var) = op {
                println!("returned + 1 is {} ", var + 1);
            } else {
                println!("Empty value :(");
            }
        }
        test_and_print_plus_one(returns_none_or_number(true));
        test_and_print_plus_one(returns_none_or_number(false));
    }
    {
        // 'match' takes ownership, only one value will be created
        println!("A println before match");
        let v = match Verbose::new(1) { //                <--|
            // match pattern's type corresponds to the type of expression ---|
            inner_variable => { // : Verbose; move value to 'inner_variable', because type of `match`'s expression is not a reference
                println!("This is println from 'match' {:?}", inner_variable);
                inner_variable
            }
        };
        println!("A println after match, {:?}", v);
        // value will be dropped here
    }
    {
        // 'match' takes ownership, only one value will be created
        println!("A println before match");
        let v = match &Verbose::new(2) { // creates a temporary which is freed while still in use
            inner_variable => { // : &Verbose; borrow value, because type of `match`'s expression is a reference
                println!("This is println from 'match' {:?}", inner_variable);
                inner_variable // is valid in 'match', because 'match ... {}' is a one expression
            }
        };// value will be dropped here
        compilation_error!(
            println!("A println after match, {:?}", v); // temporary value dropped while borrowed
        );
        println!("A println after match")
    }
}
