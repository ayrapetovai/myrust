extern crate myrust;
use myrust::compilation_error;
use myrust::Verbose;

#[allow(unused)]
fn main() {
// A pattern consists of some combination of the following:
// 1. Literals: 1, "abc"
// 2. Destructured arrays, enums, structs, or tuples
// 3. Variables
// 4. Wildcards
// 5. Placeholders

// A particular pattern _ will match anything, but it never binds to a variable
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
        // if let else if let
        let x: Option<bool> = None;
        let y: Option<bool> = Some(true);
        if let Some(value) = x {
            println!("x is {}...", value);
        } else if let Some(true) = y { // value hardcoded
            println!("y is true...");
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
        // while let
        let mut stack = vec![1, 2, 3];
        while let Some(the_most_right_element) = stack.pop() { // 'pop' returns 'None' at 4th time
            println!("Vector's element is {}", the_most_right_element); // 3 // 2 // 1
        }
        assert_eq!(0, stack.len());
    }
    {
        // function params
        fn print_coordinates((x, y): (i32, i32)) {
            println!("Cords are ({}, {}), sum is {}", x, y, x + y);
        }
        let c = (10, 13);
        print_coordinates(c);
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
        // 'match' borrows, only one value will be created
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
    {
        // values, 'or' operator |
        let x = "hello";
        match x {
            "hello" | "goodby" => println!("hello/goodby words"),
            _ => {}
        }
    }
    // Patterns that will match for any possible value passed are irrefutable.
    // Function parameters, let statements, and for loops can only accept irrefutable patterns,
    // because the program cannot do anything meaningful when values don’t match.

    // if let, match, while let - can accept refutable patterns.
    // "variable name" is an irrefutable pattern. literal is refutable one.
    {
        // function can take only irrefutable pattern
        compilation_error!(
            fn foo(Some(x): i32) {} // mismatched types
        );

        fn foo((x, y): (i32, i32)){} // OK, x and y matches everything

        compilation_error!(
            let Some(x) = Option::from(1); // refutable pattern in local binding: `None` not covered
        );

        if let Some(x) = Option::from(1) {
            // OK, 'if let' can take refutable pattern
        }
    }
    {
        // matching ranges
        let x = 4;
        match x {
            1..=4 => println!("The 'x' seems to be somewhat between 1 and 4 (inclusive)"),
            _ => println!("'x' does not belong to [1...4] interval")
        }
    }
    {
        // struct destruction
        let v = Verbose::new(3);
        match v {
            Verbose { id} => println!("match: id of given Verbose is {}", id)
        }
        let id = v.id; // 'v' is valid here
        println!("after: id of given Verbose is {}", id);

        let Verbose { id: the_id } = v;
        println!("let: id of given Verbose is {}", the_id);
    }
    {
        // match if
        let t = (1, "hello");
        let s = match t {
            (0, ss) => ss.to_owned(),
            (1, ss) => ss.to_uppercase(), // taken
            (_, ss) => String::from("")
        };
        println!("matching with comparison taken {}", s);
        assert_eq!("HELLO", s);
    }
    {
        let t = (1, 2);
        let (_, x) = t;
        assert_eq!(2, x);
    }
    {
        // Ignoring Remaining Parts of a Value with ..
        struct Point {
            x: i32,
            y: i32,
            z: i32,
        }

        let origin = Point { x: 0, y: 0, z: 0 };
        match origin {
            Point { x, .. } => println!("x is {}", x),
        }

        let numbers = (2, 4, 8, 16, 32);
        match numbers {
            (first, .., last) => {
                println!("Some numbers: {}, {}", first, last);
            }
        }
        compilation_error!(
            match numbers {
                // using .. must be unambiguous
                (.., second, ..) => { // `..` can only be used once per tuple pattern
                    println!("Some numbers: {}", second)
                }
            }
        );
    }
    {
        // "match guard" is an additional if condition specified after the pattern in a 'match' arm
        // "match guard" is not a pattern, it does not introduce new variables
        let num = Some(4);

        match num {
            Some(x) if x < 5 => println!("less than five: {}", x),
            Some(x) => println!("{}", x),
            None => (),
        }
    }
    {
        // "match guard" for primitive types (without deconstruction)
        // condition under 'if''s expression does play role in exhaustion analysis
        let n = 4;
        match n {
            x if x < 0 => println!("n is positive"),
            x if x >= 0 => println!("n is negative"),
            _ => {} // keep compiler happy, never happens
        }
    }
    {
        // ignoring some values
        let numbers = (2, 4, 8, 16, 32);

        match numbers {
            (first, _, third, _, fifth) => {
                println!("Some numbers: {}, {}, {}", first, third, fifth)
            }
        }
    }
    {
        struct Point {
            x: i32,
            y: i32
        }
        let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
    }
    // {
    //     // bindings with '@', '@' is used after the name of introduced variable
    //     let x = (42, 13);
    //     match x {
    //         (n @ 1..20, _)  => println!("at 1..20 arm"),
    //         (n @ 20..30, _)  => println!("at 20..30 arm"),
    //         (n @ 30..40, _)  => println!("at 30..40 arm"),
    //         (n @ 40..50, _)  => println!("at 40..50 arm"),
    //         _ => {}
    //     }
    // }
    {
        // bindings with '@'
        enum Message {
            Hello { id: i32 },
        }

        let msg = Message::Hello { id: 5 };

        match msg {
            Message::Hello {
                id: id_variable @ 3..=7,
            } => println!("Found an id in range: {}", id_variable),
            Message::Hello { id: 10..=12 } => {
                println!("Found an id in another range")
            }
            Message::Hello { id } => println!("Found some other id: {}", id),
        }
    }
}
