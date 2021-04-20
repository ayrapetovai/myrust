use std::fmt::{Display, Formatter};
use std::fmt;
extern crate myrust;
use self::myrust::compilation_error;


#[derive(Debug)]
struct User {
    email: String,
    name: String,
    sign_in_count: u64,
    active: bool,
}

#[allow(unused)]
fn main() {
    {
        let name = String::from("username");
        let mut user1 = User {
            email: String::from("username@host.domen"),
            name, // if variable and field have same names
            active: true,     // active and sign_in_count field are initialized in order opposite they were declared
            sign_in_count: 0,
        };
        user1.email = String::from("newusername@host.domen");
        println!("user: {:?}", user1); // due to derive Debug procedural macro

        let user2 = User {
            email: String::from("someotheruesername@host.domen"),
            // partial move
            ..user1 // the '..' before struct's is operator RangeTo<T>: takes field from user1, set the same field in user2, works as assignment with move
        };

        // user1.name was copied to user2.name, is it valid? Was it moved?
        compilation_error!(
            println!("user: {:?}", user1); // error, use of moved value of field name
        );
        user1 = user2; // user1's object is destroyed, user2's object is moved to user1
        println!("user: {:?}", user1);
    }
    {
        struct TupleStruct(i32, i32, i32);
        let color = TupleStruct(0, 0, 0);
        let point = TupleStruct(1, 2, 4);
        // destructing tuple struct
        let TupleStruct(x, y, z) = point;
        println!("Point {} {} {}", x, y, z);
        println!("Green color {}", color.1); // .number access works
    }
    {
        struct Rectangle {
            width: i32,
            height: i32,
        }
        fn area(r: &Rectangle) -> i32 {
            r.width * r.height
        }
        let r = Rectangle { width: 50, height: 21 };
        let _a = area(&r); // borrow (take by reference, no ownership passed)
        
        impl Display for Rectangle {
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                write!(f, "Rectangle({}, {})", self.height, self.width)
            }
        }
        println!("var {}", r); // Rectangle(21, 50)
    }
    {
        #[derive(Debug)]
        struct Rectangle {
            width: u32,
            height: u32,
        }
        impl Rectangle {
            fn area(&self) -> u32 {
                self.height * self.width
            }
            fn can_hold(&self, other: &Rectangle) -> bool {
                other.width < self.width && other.height < self.height
            }
        }
        // multiple impl blocks are allowed, but single only can be used.
        impl Rectangle {
            // associated function (static analogue) has no self (that makes it "associative", "static")
            // Self is a type to be implemented
            fn square(side: u32) -> Self {
                Self { width: side, height: side }
            }
        }

        let r = Rectangle { height: 13, width: 42 };
        println!("Area of {:?} is {}", r, r.area());
        // Still 'r' is valid, println! did not take ownership. Why?
        println!("Can hold {}", r.can_hold(&Rectangle { height: 1, width: 2 })); // true

        let big_rectangle = Rectangle { height: u32::MAX, width: u32::MAX };
        println!("Can hold big rectangle {}", r.can_hold(&big_rectangle)); // false

        println!("Square {:?}", Rectangle::square(100));
    }
}