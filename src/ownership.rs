
// Each value in Rust has a variable that’s called its owner.
// There can only be one owner at a time.
// When the owner goes out of scope, the value will be dropped.

macro_rules! compilation_error {
    ($s:stmt $(;)?) => {}
}

struct MyStruct {
    x: i32
}

impl Drop for MyStruct {
    fn drop(&mut self) {
        println!("Dropping the MyStruct instance with x = {}", self.x);
    }
}

impl Clone for MyStruct {
    fn clone(&self) -> Self {
        println!("Cloning MyStruct instance with x = {}", self.x);
        MyStruct{ x: self.x }
    }
}

// Cannot implement Copy and Drop simultaneously for the same enum/struct/union.
// impl Copy for MyStruct {} // compilation error: the trait `Copy` may not be implemented for this type; the type has a destructor
// trait Copy is for scalar types that don't need to be dropped: integers, bool, floats, char, tuples (only, if all elements implement Copy trait)

// impl Drop for i32 {...} // compilation error: the `Drop` trait may only be implemented for structs, enums, and unions

// passing a value (to a function) will move or copy, as assignment.

#[allow(unused)]
fn main() {
    {
        // When s comes into scope, it is valid.
        // It remains valid until it goes out of scope.
        let mut s = String::from("hello"); // 'String' is allocated at heap, weather 'str' is a string literal.
        s.push('!'); // The 'String' can be mutated
        println!("the s: String is '{}'", s);
    } // before control flow goes out of scope Rust calls 'drop' function (destructor), defined for 'String'
    {
        // variable 's' is an "Owner" of the instance of MyStruct
        let s = MyStruct { x: 2 };
        // MyStruct::drop prints: Dropping the MyStruct instance with x = 2
    }
    {
        let s1 = MyStruct { x: 1 };
        // s1 is moved to s2, shallow copy
        let s2 = s1;
        // passing the Ownership of the instance from s1 to s2, only owner can call 'drop' function, s1 is invalidated
        compilation_error!(
            println!("Attempt to read s1.x (ownership was taken) {}", s1.x); // error: borrow of moved value: `s1`
        );
        println!("Read s2.x (ownership was given) {}", s2.x); // OK
        // MyStruct::drop is called only once, for s2, not for s1 (because it is invalidated), prints: Dropping the MyStruct instance with x = 1
    }
    {
        // Stack-Only Data is copied only
        let x = 1;
        let y = x; // 'x' is a primitive on stack, no need to invalidate it, no need to pass ownership, just copy x's value to y
        println!("Attempt to read x: i32 (ownership was taken, no) {}", x);
        println!("Read y (=x, ownership was given, no) {}", y);
    }
    {
        let s1 = MyStruct { x: 3 };
        let s2 = s1.clone(); // value s1 was not moved to s2, it was deeply copied, s1 still is valid
        println!("Attempt to read s1.x (ownership was not taken, cloned) {}", s1.x); // 3
        println!("Read s2.x (=s1.clone(), ownership on another instance was given, no) {}", s2.x); // 3
        // two destructors 'drop' was called, for s1 and s2.
    }
    {
        let x = "empty";
        let y = x;   // no move, no invalidation for constant string literals of type 'str'
        println!("{}", x); // x is not invalidated, because it is of type 'str', a constant string literal
    }
    println!("***** functions and ownership");
    {
        fn takes_ownership(s: MyStruct) {
            println!("function takes_ownership owning a MyStruct with x = {}", s.x);
        }

        let s = MyStruct { x: 4 };
        takes_ownership(s); // instance of MyStruct is moved, no copy, 's' is invalidated
        // only one call of drop function for one MyStruct instance
    }
    {
        fn gives_ownership() -> MyStruct {
            MyStruct { x: 5 } // value is moved
        }
        let s = gives_ownership();
        println!("Ownership on instance was given, no drop was called");
        println!("Print s.x is {}", s.x)
        // drop for s is called, only one instance of MyStruct was created
    }
    {
        fn takes_ownership_and_gives_it_back(s: MyStruct) -> MyStruct {
            println!("Ownership on MyStruct{{ x: {} }}, was taken", s.x);
            s
        }
        let s1 = MyStruct { x: 6 };
        let s2 = takes_ownership_and_gives_it_back(s1); // s1 is invalidated, s2 has ownership
        // drop is called only once
    }
    println!("***** references and borrowing");
    {
        // references allow to refer to some value without taking ownership
        fn does_not_take_ownership(s: &MyStruct) {
            println!("function does_not_take_ownership cann access fields of referenced value MyStruct.x is {}", s.x);
            compilation_error!(
                *s = MyStruct { x: i32::MAX }; // `s` is a `&` immutable reference, so the data it refers to cannot be written
            );
            // no drop called for s, because it does not have ownership
        }
        let s = MyStruct { x: 7 };
        // '&x' means: pass 'x' without ownership transmission (by reference)
        does_not_take_ownership(&s); // pass variable 's' by reference with '&', 's' is not invalidated
        println!("Since function 'does_not_take_ownership' have no any ownership taken, 's' is valid, s.x is {}", s.x);
        // only one call of drop is needed
    }
    println!("***** mutable references");
    {
        // references are immutable, as variables
        fn add_some_word(s: &mut String) {
            s.push_str(" pushed");
        }
        let mut s = String::from("Push target");
        add_some_word(&mut s); // mutable references can refer only to mutable variable, no move, 's' is valid
        println!("String was mutated inside a function: {}", s);
    }
    {
        // no multiple mutable referencing is allowed to except data race, that can occur when:
        // 1. Two or more pointers access the same data at the same time.
        // 2. At least one of the pointers is being used to write to the data.
        // 3. There’s no mechanism being used to synchronize access to the data.

        let mut s = String::from("5123");
        let r1 = &mut s;
        let r2 = &mut s;

        // due to two mutable references was defined in the same scope
        compilation_error!(
            let s1 = *r2; // cannot borrow `s` as mutable more than once at a time
        );
    }
    {
        let mut s = String::from("5123");
        {
            let r1 = &mut s;
            println!("Accessing first mutable reference: {}", r1);
        } // r1 is gone, no drop called, due to r1 is a reference (no ownership)
        let r2 = &mut s;
        println!("Accessing second mutable reference: {}", r2);
    }
    {
        let mut s = String::from("Money");
        let immutable_ref = &s;
        let mutable_ref = &mut s;

        compilation_error!(
            let s2 = immutable_ref; // cannot borrow `s` as mutable because it is also borrowed as immutable
        );
    }
    {
        // A reference’s scope starts from where it is introduced and continues through the last time that reference is used.
        let mut s = String::from("Money");
        let immutable_ref = &s;
        println!("Using immutable reference before mutable one is declared, s is {}", immutable_ref);
        {
            let mutable_ref = &mut s;
            println!("Using mutable reference, s is {}", mutable_ref);
        }

        // At this point of execution, value of 's' could be modified throw 'mutable_ref', but 'immutable_ref' try to guarantee that it is original
        compilation_error!(
            let s2 = immutable_ref; // cannot borrow `s` as mutable because it is also borrowed as immutable
        );
    }
    println!("***** dangling references");
    {
        // after function 'dangle' returns, 's' wil be dropped, because of dangle owns 's'
        // and reference to 's' will become "dangling" - not valid

        // fn dangle() -> &String { // compilation error: missing lifetime specifier
        //     let s = String::from("hello");
        //     &s
        // }

        // one of solutions is to return string with ownership for it not to be dropped
        fn no_dangle() -> MyStruct {
            let s = MyStruct{ x: 8 };
            s
        }
        println!("return value is taken with ownership, MyStruct's x is {}", no_dangle().x);
        println!("return value is dropped before this statement begins");

        // reference rules:
        // 1. At any given time, you can have either one mutable reference or any number of immutable references.
        // 2. References must always be valid.
    }
    println!("*****  slices");
    {
        // Slices let reference a contiguous sequence of elements in a collection rather than the whole collection.
        // Slices represent no ownership.
        fn find_first_word_end_index(s: &String) -> usize {
            for (i, &c) in s.as_bytes().iter().enumerate() {
                if c == b' ' {
                    return i;
                }
            }
            s.len()
        }
        let mut s = String::from("one two three");
        let first_word_end = find_first_word_end_index(&s);
        s.clear();
        // now 'first_word_end' has no meaning for an empty string, it exists logically "separately" from string

        // slice exists in tight coupling with collection

        // A string slice is a reference to part of a String
        s = String::from("a new string");
        let a_slice = &s[1..7]; // end_index is not inclusive, always immutable borrow, a_slice: &str
        compilation_error!(
            let a_slice = s[1..7]; // cannot move
        );
        println!("The slice of the string '{}' is '{}'", s, a_slice);

        let whole_string = &s[..];
        let from_start_to = &s[..7];
        let from_to_end = &s[4..];
    }
    {
        // "sliced" find_first_word
        fn find_first_word(s: &str) -> &str { // taking &str allow to handle with String, but not vise versa
            let bytes = s.as_bytes();
            for (i, &b) in bytes.iter().enumerate() {
                if b == b' ' {
                    return &s[..i];
                }
            }
            &s[..]
        }
        let mut s = String::from("Hello world");
        let first_word = find_first_word(&s); // immutable borrow
        println!("Before cleaning: word found is '{}'", first_word);
        s.clear(); // mutable borrow, that prevents from using 'first_word' later
        compilation_error!(
            println!("After cleaning: word found is '{}'", first_word); //  cannot borrow `s` as mutable because it is also borrowed as immutable
        );
    }
    {
        // array slices
        let a: [i32; 3] = [0, 2, 1];
        let s = &a[1..2]; // 's' is a slice to array of integers '&[i32]'
        println!("A slice to array is {:?}", s);
    }
}
