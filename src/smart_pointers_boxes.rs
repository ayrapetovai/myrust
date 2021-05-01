#![allow(unused)]
// Smart pointers own data they pint to.
// Smart pointers implement the Deref and Drop traits

// Box<T> for allocating values on the heap, pinter itself remains on the stack
// Rc<T>, a reference counting type that enables multiple ownership
// Ref<T> and RefMut<T>, accessed through RefCell<T>, a type that enforces the borrowing rules at runtime instead of compile time

extern crate myrust;
use myrust::Verbose;
use myrust::compilation_error;

// construct function list (cons list)
compilation_error!(
    enum List<T> {
        Nil,
        Cons(T, List<T>) // recursive type `List` has infinite size
    }
);

#[derive(Debug)]
enum List<T> {
    Nil,
    Cons(T, Box<List<T>>)
}

use crate::List::{Nil, Cons};
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    {
        let b = Box::new(42); // stores i32 value on the heap
        println!("Stored value is {}", b);

        let a = *b + 13; // explicit unboxing in needed
        println!("+= 13 is {}", a);
    }
    {
        // move value from stack to heap
        let x = 42;
        let b = Box::new(x);
        println!("try to use x {}", x + 1); // Ok, 'x' is copyable
    }
    {
        // move value from stack to heap
        let v = Verbose { id: 1 };
        let b = Box::new(v); // ownership is taken from 'v'
        compilation_error!(
            println!("try to use x {}", v.id); // borrow of moved value: `v`
        );
        println!("Verbose on heap {:?}", b);
        // here 'v' will be dropped
    }
    {
        let v = Box::new(vec![1, 2, 3]);
        // no explicit dereferencing is needed when call methods
        v.iter().for_each(|x| println!("the value in vector is {}", x));
    }
    {
        println!("After list declaration");
        let l = Cons(Verbose::new(2), Box::new(Cons(Verbose::new(3), Box::new(Nil))));
        println!("Before all list elements are dropped {:?}", l);
    }
    {
        struct MyBox<T>(T); // tuple structure
        impl<T> MyBox<T> {
            fn new(v: T) -> MyBox<T> {
                MyBox(v)
            }
        }
        impl<T> Deref for MyBox<T>{
            type Target = T;
            fn deref(&self) -> &Self::Target {
                let MyBox(x) = self; // or '&self.0'
                x
            }
        }
        let x = 5;
        let y = MyBox::new(x);

        assert_eq!(5, x);
        assert_eq!(5, *y);

        fn take_ref(m: &i32) {}

        take_ref(&y);  // same as ...
        take_ref(&*y); // this. In above dereferencing is maid implicitly, called "deref coercion", no runtime  penalty
    }
    {
        // Rust does deref coercion when it finds types and trait implementations in three cases:
        // 1. From &T to &U when T: Deref<Target=U>
        // 2. From &mut T to &mut U when T: DerefMut<Target=U>
        // 3. From &mut T to &U when T: Deref<Target=U>
        #[derive(Debug)]
        struct A { value: i32 }
        impl Deref for A {
            type Target = i32;
            fn deref(&self) -> &Self::Target {
                println!("A deref, value is {}", self.value);
                &self.value
            }
        }
        impl DerefMut for A {
            fn deref_mut(&mut self) -> &mut Self::Target {
                println!("A deref_mut, value is {}", self.value);
                &mut self.value
            }
        }
        let a = Box::new(A { value: 1 });
        let x: &i32 = &a;         // A deref, value is 1

        let mut a = Box::new(A { value: 1 });
        let mut x: &i32 = &mut a; // A deref, value is 1, rule "From &mut T to &U when T: Deref<Target=U>"
        x = &2; // 'x' is changed, but 'a.value' does not
        println!("x of value of A {{ value: 1 }} is {}", x);
        println!("A {{ value: 1 }} now is {:?}", a);

        let x: &mut i32 = &mut a; // A deref_mut, value is 1, rule "From &mut T to &mut U when T: DerefMut<Target=U>"
        compilation_error!(
            x = &3; // types differ in mutability, expected mutable reference `&mut i32`, found reference `&i32`
        );

        a.value = 4;
        let x: &i32 = &mut a;     // A deref, value is 4, rule "From &mut T to &U when T: Deref<Target=U>"
    }
    {
        // Box<T> allows immutable or mutable borrows checked at compile time;
        // Rc<T> allows only immutable borrows checked at compile time;
        // RefCell<T> allows immutable or mutable borrows checked at runtime.
        let r = RefCell::new(Verbose::new(4));
        compilation_error!(
            *r = 10; // type `RefCell<Verbose>` cannot be dereferenced
        );
        compilation_error!(
            r.get_mut().id = i32::MAX; // cannot borrow `r` as mutable, as it is not declared as mutable
        );
        let mut r = r;
        compilation_error!(
            r.id = 10; // no field `id` on type `RefCell<Verbose>`
        );
        r.get_mut().id = i32::MAX;
    }
}

trait Messenger {
    fn send(&self, msg: &str);
}
struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}
impl<'a, T: Messenger> LimitTracker<'a, T> {
    fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max
        }
    }

    fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger { sent_messages: RefCell::new(vec![]) }
        }
    }
    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            self.sent_messages.borrow_mut().push(msg.to_owned());
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);
        // The RefCell<T> keeps track of how many Ref<T> and RefMut<T> smart pointers are currently active.
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
