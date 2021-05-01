use std::rc::{Rc, Weak};
use std::cell::RefCell;
extern crate myrust;
use myrust::Verbose;

// RefCell specifies what field we want to able to modify in an immutable object
#[derive(Debug)]
enum List<T> {
    Nil,
    Cons(T, RefCell<Rc<List<T>>>)
}

use crate::List::{Nil, Cons};

impl<T> List<T> {
    fn tail(&self) -> Option<&RefCell<Rc<List<T>>>> {
        match self {
            Nil => None,
            Cons(_, t) => Some(t)
        }
    }
}

#[derive(Debug)]
struct Node<T> {
    value: T,
    parent: RefCell<Weak<Node<T>>>,
    children: RefCell<Vec<Rc<Node<T>>>>
}

fn main() {
    {
        // memory leak
        let a = Rc::new(Cons(Verbose::new(5), RefCell::new(Rc::new(Nil))));
        println!("a initial rc count = {}", Rc::strong_count(&a));
        println!("tail of a is {:?}", a.tail());

        let b = Rc::new(Cons(Verbose::new(10), RefCell::new(a.clone())));
        println!("a rc count after b creation = {}", Rc::strong_count(&a));
        println!("b initial rc count = {}", Rc::strong_count(&b));
        println!("tail of b is {:?}", b.tail());

        if let Some(link) = a.tail() {
            *link.borrow_mut() = b.clone(); // cycled!
        }
        println!("b rc count after changing a = {}", Rc::strong_count(&b));
        println!("a rc count after changing a = {}", Rc::strong_count(&a));

        // now 'println!("{}", a.tail());' will overflow the stack
        println!("Verbose 5, 10 were not dropped, memory leak due to strong cross referencing");
    }
    {
        let leaf = Rc::new(Node {
            value: Verbose::new(11),
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![])
        });

        let branch = Rc::new(Node {
            value: Verbose::new(12),
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![leaf.clone()])
        });

        println!("Before crating weak reference, weak count of branch is {}", Rc::weak_count(&branch)); // 0
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
        println!("After crating weak reference, weak count of branch is {}", Rc::weak_count(&branch)); // 1

        println!("Branch {:?}", branch);
        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
        println!("Verbose 11 and 12 will be dropped, no memory leak, due to weak cross referencing");
    }
}
