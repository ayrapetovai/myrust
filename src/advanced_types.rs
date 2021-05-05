#![allow(unused)]
extern crate myrust;
use myrust::compilation_error;

use std::any::TypeId;

fn main() {
    {
        // type alias
        type Kilometers = i32; // alias is not a separate, new type.

        fn foo(d: i32) {
            println!();
        }
        compilation_error!(
            fn foo(d: Kilometers) { // the name `foo` is defined multiple times
                println!();
            }
        );

        let are_same = TypeId::of::<Kilometers>() == TypeId::of::<i32>();
        println!("Type and it's alias are {}", if are_same { "same" } else { "different" }); // same
    }
    {
        // aliases can be used to instantiate generics
        type VectorInt = Vec<i32>;
        let v: VectorInt = [1, 2, 3, 4].iter().cloned().collect();
        println!("VectorInt {:?}", v);
    }
    {
        // aliases can be parametrized with generics
        #[derive(Debug)]
        struct Point<T, W> { x: T, y: W }
        type ThePoint<T> = Point<T, f32>; // bound second generic's value

        let p = ThePoint { x: 1, y: 2.0 }; // alias used as type name
        println!("ThePoint {:?}", p);
    }
    {
        // The Never Type that Never Returns
        compilation_error!(
            fn baz() -> ! {} // implicitly returns `()` as its body has no tail or `return` expression
        );

        fn bar() -> ! { std::process::exit(0) } // '!' differs from '()'

        loop {
            // the types of the 'match''s arms must be equals
            let variable = match true {
                true => break, // this arm's type is '!' - "no value"
                _ => "",       // this arm's type is &str
            };
            // compiler can determine the type of 'variable' as '&str', because one of arms returns '!' (nothing) and another returns '&str'
        }
        // also 'panic!' returns '!'

        fn foo() -> ! {
            loop {} // loop returns '!'
        }
    }
    {
        // "dynamically sized types", referred to as DSTs or "unsized types"
        // 'str' is unsized type
        compilation_error!(
            let s: str = "abc";
        );
        // Rust needs to know how much memory to allocate for any value of a particular type, and all values of a type must use the same amount of memory.
        // The golden rule of dynamically sized types is that we must always put values of dynamically sized types behind a pointer of some kind.
        // Every trait is a "dynamically sized type" we can refer to by using the name of the trait

        // To work with DSTs, Rust has a particular trait called the Sized trait to determine whether or not a type’s size is known at compile time.
        // This trait is automatically implemented for everything whose size is known at compile time.

        // Rust implicitly adds a bound on Sized to every generic function
        fn function_with_generic1<T>(x: T) {}
        // is the same as
        fn function_with_generic2<T: Sized>(x: T) {} // by default
        // compilation_error!
        // fn function_with_generic3<T: ?Sized>(x: T) {} // the size for values of type `T` cannot be known at compilation time

        // A trait bound on ?Sized is the opposite of a trait bound on Sized: we would read this as “T may or may not be Sized.
        fn function_with_generic4<T: ?Sized>(x: &T) {} // Ok
        // the '?' syntax is for 'Sized' trait only.
    }
}
