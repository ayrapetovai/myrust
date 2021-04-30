#![allow(unused)]
extern crate myrust;
use myrust::Verbose;
use myrust::compilation_error;

fn main() {
    {
        let mut v = Verbose::new(0);
        v = Verbose::new(1); // v(0) is dropped here
        println!("Before gone out of scope");
    }
    {
        let mut v = Verbose::new(2);
        compilation_error!(
            v.drop(); // explicit use of destructor method, explicit destructor calls not allowed
        );
        // this 'drop' calls Drop::drop(&mut self)
        std::mem::drop(v); // takes ownership, this 'drop' "has no implementation"
        // 'std::mem:drop' may be useful when do RAII

        println!("v(2) is droppedm, before it has gone out of scope");
        v = Verbose::new(3);
        compilation_error!(
            let vv = v; // use of moved value: `v`, 'v' is invalid
        );
        println!("Going out of scope");
    }
}
