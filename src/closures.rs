use std::time::Duration;
use std::thread;
extern crate myrust;
use myrust::compilation_error;
#[allow(unused_imports)] // this should be not needed, it is a bug, run '$ cargo test' without this macro
use myrust::Verbose;
use std::collections::HashMap;
use std::ops::Add;

/*
Closures can capture values from their environment in three ways, these are encoded in the three Fn traits as follows:
1. FnOnce takes ownership of these variables and move them into the closure when it is defined. It can be called only once.
2. FnMut can change the environment because it mutably borrows values.
3. Fn borrows values from the environment immutably.
Compiler decides which of type a closure will be.
 */
struct Catcher<T>
    where T: Fn(u32) -> u32
{
    cache: HashMap<u32, u32>,
    calculation: T,
}

impl<T> Catcher<T>
    where T: Fn(u32) -> u32
{
    fn new(function: T) -> Catcher<T> {
        Catcher {
            cache: HashMap::new(),
            calculation: function,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.cache.get(&arg) {
            None => {
                let value = (self.calculation)(arg);
                self.cache.insert(arg, value);
                value
            },
            Some(v) => *v
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut catcher = Catcher::new (
        |num| {
            println!("Calculating slowly...");
            thread::sleep(Duration::from_secs(2));
            num
        }
    );
    if intensity < 25 {
        println!("Today, do {} pushups!", catcher.value(intensity));
        println!("Next, do {} situps!", catcher.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", catcher.value(intensity));
        }
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
    {
        let example_closure = |x| x; // type of x will be inferred only once, the first time
        let _s = example_closure(String::from("hello"));
        compilation_error!(
            let n = example_closure(5); // expected struct `String`, found integer
        );
    }
    {
        // function pointers
        // 'Fn' - closure trait while 'fn' is a function pointer
        // fn: Fn: FnMut: FnOnce - we can pass function pointer anywhere where we cant pass 'Fn' of 'FnMut' etc.
        fn add_one(x: i32) -> i32 {
            x + 1
        }

        fn binary_applier<T>(f: fn(T)->T, x: T) -> T
            where T: Copy + Add<Output = T>
        {
            f(x) + f(x)
        }

        println!("(x+1) + (x+1) is {}", binary_applier(add_one, 10));
        println!("(x+1) + (x+1) is {}", binary_applier(|x| x + 1, 10));
    }
    {
        // function-like syntax of initialization can be used as function pointers
        #[derive(Debug, PartialOrd, PartialEq)]
        enum Status {
            Value(u32),
            #[allow(dead_code)]
            Stop,
        }
        let list_of_statuses: Vec<Status> = (0..2).map(Status::Value).collect();
        assert_eq!(vec![Status::Value(0), Status::Value(1),], list_of_statuses);
    }
    {
        // "returning" closures from functions
        compilation_error!(
            fn returns_closure() -> dyn Fn(i32) -> i32 { // 'dyn Fn(i32) -> i32' doesn't have a size known at compile-time
                |x| x + 1
            }
        );

        fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
            Box::new(|x| x + 1)
        }
        let incremented_ten = returns_closure()(10);
        assert_eq!(11, incremented_ten);
    }
}

// no #[cfg(test)] is necessary, run by '$ cargo test'
#[test]
fn call_with_different_values() {
    let mut c = Catcher::new(|a| a);

    let _v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 2);
}

#[test]
fn closure_captures_environment_variables() {
    let x = 2;
    let is_equal_to_x = |y| y == x;
    assert!(is_equal_to_x(2));
    assert!(!is_equal_to_x(3));
}

#[test]
fn calling_closure_right_where_we_define_it() {
    let sum = (|a, b| a + b)(13, 42);
    assert_eq!(sum, 13 + 42);
}

#[test]
fn force_closure_to_take_ownership() {
    let v = Verbose { id: 1 };
    println!("Before moving verbose to closure");
    let print_verbose = move || println!("This is verbose {:?}", v);
    compilation_error!(
        println!("External printing verbose {:?}", v); // borrow of moved value: `v`
    );
    println!("After moving verbose to closure");
    print_verbose();
    println!("After 1st calling closure");
    print_verbose(); // 'v' in closure is still alive
    println!("After 2nd calling closure");
    // Verbose drops
}
