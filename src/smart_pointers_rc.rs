extern crate myrust;
use myrust::Verbose;

use crate::List::{ Nil, Cons };

// To enable multiple ownership, Rust has a type called Rc<T>, which is an abbreviation for "reference counting".
// Rc<T> gives only immutable access!
// for single threaded usage only
// allocates memory on heap
enum List {
    Nil,
    Cons(i32, Rc<List>)
}
impl List {
    fn iterate_over(&self, function: &dyn Fn(&i32) -> ()) {
        let mut i = self;
        loop {
            match i {
                Nil => break,
                Cons(value, tail) => {
                    function(value);
                    i = tail; // 'tail.as_ref()' implicitly
                }
            }
        }
    }
}

use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    {
        // Rc::clone does not do deep copy, just increments a reference counter
        let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
        println!("Strong counts of references to 'a' before cloning is {}", Rc::strong_count(&a));
        let b = Rc::new(Cons(3, a.clone())); // why 'Rc::clone(&a)' is suggested?
        println!("Strong counts of references to 'a' after first cloning is {}", Rc::strong_count(&a));
        let c = Rc::new(Cons(4, Rc::clone(&a))); // same implementation
        println!("Strong counts of references to 'a' after second cloning is {}", Rc::strong_count(&a));
        {
            let _d = Rc::new(Cons(6, a.clone()));
            println!("Strong counts of references to 'a' after cloning to local 'd' is {}", Rc::strong_count(&a));
        }
        println!("Strong counts of references to 'a' after destructing 'd' is {}", Rc::strong_count(&a));

        let printer = &|x: &i32| println!("List element's value is {}", x);
        println!("List a");
        a.iterate_over(printer);
        println!("List b");
        b.iterate_over(printer);
        println!("List c");
        c.iterate_over(printer);
    }
    {
        // Having Multiple Owners of Mutable Data by Combining Rc<T> and RefCell<T>
        let v = Rc::new(RefCell::new(Verbose::new(42)));
        let non_suspecting_ref = v.clone();
        println!("non_suspecting_ref is immutable and sees{:?}", non_suspecting_ref.borrow()); // 42
        v.borrow_mut().id = 13; // can mutate, despite of 'v' and 'non_suspecting_ref' are immutable
        println!("non_suspecting_ref now sees {:?}", non_suspecting_ref.borrow()); // 13
    }
}
