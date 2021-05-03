#![allow(unused)]
extern crate myrust;
use myrust::compilation_error;

trait Animal {
    fn make_sound(&self);
}

struct Zoo<T: Animal> {
    animals: Vec<Box<T>>
}
impl<T: Animal> Zoo<T> {
    fn make_sound(&self) {
        for animal in self.animals.iter() {
            animal.make_sound();
        }
    }
}

struct Wolf {}
impl Animal for Wolf {
    fn make_sound(&self) {
        println!("wolf says \"Come here little Read Hood\"");
    }
}

struct Horse {}
impl Animal for Horse {
    fn make_sound(&self) {
        println!("horse says \"<3 Salvatore Ganachi>\"");
    }
}

fn main() {
    // 'dyn' - for dynamic method dispatching
    // warning: trait objects without an explicit `dyn` are deprecated
    // 'Box' (a head allocation) is needed, because 'Horse' and 'Wolf' may occupy different memory sizes, cannot create them on stack
    let zoo: Vec<Box<dyn Animal>> = vec![Box::new(Horse{}), Box::new(Wolf{})];
    for beast in zoo {
        beast.make_sound();
    }
    // let zoo = Zoo {
    //     animals: vec![Box::new(Horse{}), Box::new(Wolf{})]
    // };
    // zoo.make_sound();

    //  A trait object points to both an instance of a type implementing our specified trait as well as a table used to look up trait methods on that type at runtime.
    // can only make object-safe traits into trait objects.
    // 1. The return type isn’t Self.
    // 2. There are no generic type parameters.
    compilation_error!(
        let x: Box<dyn Clone>; // the trait `Clone` cannot be made into an object
    );
}
