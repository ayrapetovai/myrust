macro_rules! compilation_error {
    ($s:stmt $(;)?) => {}
}

macro_rules! runtime_error {
    ($s:stmt $(;)?) => {}
}

#[allow(unused)]
fn main() {
    {
        enum A {
            One, Two
        }
        #[derive(PartialEq)]
        enum IpVersion {
            // enumerators kind of are living in the "namespace" of the enum
            V4,
            V6,
        }

        let four = IpVersion::V4;
        let six = IpVersion::V6;

        compilation_error!(
            four == A::One; // mismatched types: expected enum `IpVersion`, found enum `A`
        );

        struct IpAddress {
            version: IpVersion,
            address: String
        }

        let localhost = IpAddress { version: IpVersion::V4, address: String::from("0.0.0.0") };
        let loopback = IpAddress { version: IpVersion::V6, address: String::from("::1") };
    }
    {
        // there are only 2 ways to get associated value from variant: 'match' and 'if let'
        enum IpVersion {
            V4(String), // enumerator (variant) has associated String value
            V6(String),
        }
        let localhost = IpVersion::V4(String::from("0.0.0.0"));
        let loopback = IpVersion::V6(String::from("::1"));
    }
    {
        enum IpVersion {
            V4(u8, u8, u8, u8), // enumerator (variant) has 4 associated u8 values (tuple struct)
            V6(String),
        }

        // enumerator (variant) instantiation
        let localhost = IpVersion::V4(0, 0, 0, 0);
        let loopback = IpVersion::V6(String::from("::1"));
    }
    {
        // enumerators (variants) can be arbitrary types simultaneously
        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(u8, u8, u8)
        }

        struct QuitMessage; // unit struct
        struct MoveMessage { x: i32, y: i32 } // basic struct
        struct WriteMessage(String); // tuple struct
        struct ChangeColorMessage(u8, u8, u8); // tuple struct

        // we can write a method for enum
        impl Message {
            fn call(&self) {
                match self {
                    Self::Quit => println!("Quiting..."),
                    Self::Move { x, y} => println!("Moving to ({}, {})", x, y),
                    Self::Write(s) => println!("Writing a string: '{}'", s),
                    Self::ChangeColor(r, g, b) => println!("Set color to {} {} {}", r, g, b)
                }
            }
        }

        let m = Message::Write(String::from("Hello!"));
        m.call();

        fn return_some_message() -> Message {
            Message::Move { x: 13, y: 42 }
        }
        return_some_message().call();
        Message::Quit.call();
    }
    {
        // enum Option<T> { Some(T), None } for nul-concept
        let some_number1 = Some(1);
        let some_number2 = Some(2);
        compilation_error!(
            let absent_number = None; // : Option<<unknowen>>, type annotations needed for `Option<T>`
        );
        let x = some_number1.unwrap() + some_number2.unwrap();
        println!("some numbers sum is {}", x);

        let absent_number: Option<i32> = None;
        runtime_error!(
            let y = absent_number.unwrap(); // thread 'main' panicked at 'called `Option::unwrap()` on a `None` value'
        );
        println!("Does 'absent_number' exist: {}", if absent_number.is_some() { "yes" } else { "no"});
    }
    {
        fn add_one(x: Option<i32>) -> Option<i32> {
            match x {
                Some(y) => Option::Some(y + 1),
                None => None
            }
        }

        let a = add_one(Option::Some(10));
        println!("add_one to 10 is {:?}", a);
        println!("add_on to None is {:?}", add_one(None));
    }
}