// orphan rule:
// One restriction to note with trait implementations is that we can implement a trait on a type
// only if either the trait or the type is local to our crate.
// We can’t implement external traits on external types.

use std::fmt::{Display, Formatter, Debug};
extern crate myrust;
use myrust::compilation_error;
use std::ops::{Add, Deref};
use std::fmt;

trait Summary {
    fn summarize_author(&self) -> String; // Ok, abstract method, must be overridden
    fn summarize(&self) -> String { // Ok, default implementation, can be overridden
        format!("Read more... by author {}", self.summarize_author()) // can call abstract method
    }
}

#[allow(dead_code)]
struct NewsArticle {
    author: String,
    headline: String,
    content: String
}

#[allow(dead_code)]
struct Twit {
    username: String,
    content: String,
    replay: bool,
    retwit: bool
}

#[allow(dead_code)]
#[derive(Debug)]
struct Vk {
    login: String,
    message: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        self.author.clone()
    }

    fn summarize(&self) -> String {
        format!("Summary: {}: {}", self.author, self.content)
    }
}

impl Summary for Twit {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String {
        format!("Summary: {}: {}", self.username, self.content)
    }
}

impl Summary for Vk {
    fn summarize_author(&self) -> String {
        self.login.to_owned() // borrowed to ownable, usually by cloning, same as clone() for Strings
    }
    // default implementation for summarize "inherited"
}

impl Display for Vk {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self)
    }
}

fn notify(s: &impl Summary) { // accepts "subclasses" of trait 'Summary'
    println!("Braking news! {}", s.summarize())
}
// the above code '&impl Summary' is a syntax sugar for "trait bound" as below
#[allow(unused)]
fn notify_2<T: Summary>(s: &T) { // accepts "subclasses" of trait 'Summary'
    println!("Braking news! {}", s.summarize())
}

fn notify_3(s: &(impl Summary + Display)) { // Specifying Multiple Trait Bounds
    println!("Struct {}", s)
}

#[allow(unused)]
fn notify_4<T: Summary + Display>(s: &T) { // Specifying Multiple Trait Bounds
    println!("Struct {}", s)
}

// "impl Trait" syntax for returning "subclass"
fn create_summarizable() -> impl Summary {
    Twit {
        username: "aksj2ds".to_string(),
        content: "la la la la".to_string(),
        replay: false,
        retwit: false
    }
}

fn main() {
    {
        let a = NewsArticle {
            author: "Jhon Doe".to_string(),
            headline: "Smoking harm".to_string(),
            content: "blah blah blah".to_string()
        };

        let t = create_summarizable();

        let v = Vk {
            login: "ololosha".to_string(),
            message: "some text".to_string()
        };

        println!("{}", a.summarize());
        notify(&t);
        notify_3(&v);
    }
    {
        struct Pair<T> {
            first: T,
            second: T
        }

        impl<T: Display + PartialOrd> Pair<T> {
            fn greatest(&self) -> &T {
                if self.first < self.second {
                    &self.second
                } else {
                    &self.first
                }
            }
        }

        impl<T> Pair<T> {
            compilation_error!(
                fn greatest(&self) -> &T {                     // duplicate definitions for `greatest`
                    println!("I don't know...");
                    &self.first
                }
            );
            fn greatest_unknown(&self) -> String {
                "I don't know...".to_string()
            }
        }

        impl<T> Pair<T> {
            fn new(f: T, s: T) -> Pair<T> {
                Pair { first: f, second: s }
            }
        }
        let p = Pair::new("Hello", "World!");
        println!("Greatest is '{}'", p.greatest());

        let p = Pair::new(10, 2);
        println!("Greatest is {}", p.greatest());

        #[allow(dead_code)]
        struct A { x: i32 }
        let p = Pair {first: A{ x: 1}, second: A {x: 2}};
        println!("Greatest is '{}'", p.greatest_unknown());
    }
    {
        // blanket implementations:
        // implement a trait for any type that implements another trait.
        trait Runnable {
            fn run(&self) {}
        }
        trait Target {
            fn target(&self) -> String;
        }
        // for all types (T) that implement Runnable and Debug, implement trait Target
        impl<T: Runnable + Debug> Target for T {
            fn target(&self) -> String {
                format!("{:?} from target method", self)
            }
        }
        #[derive(Debug)]
        struct A { field: i32 }
        impl Runnable for A {} // also derive from Target due to blanket implementation

        // now target is available from A!
        let a = A { field: -1 };
        println!("struct has 'target' method: {}", a.target())
    }
    {
        // "associated types" are used for "generic instantiation" in definition, rather then in usage
        // generics and "associated types" difference: generics can be implemented several times, while "associated types" only once.
        trait TraitWithAccType {
            type B; // 'B' is a placeholder for a type, 'B' is an "associated type".
            fn foo() -> Self::B;
        }
        struct StructWithAccType {}
        impl TraitWithAccType for StructWithAccType {
            type B = i32; // implementation mus provide the concrete type for associated type.

            // OK
            // fn foo() -> Self::B {
            //     42
            // }
            fn foo() -> i32 { // OK, because 'B = i32' and 'fn foo() -> Self::B'
                42
            }
        }

        // error: internal compiler error: compiler/rustc_typeck/src/check/fn_ctxt/_impl.rs:526:17: no type for node HirId { owner: DefId(0:41 ~ traits[4135]::main), local_id: 364 }: type i32 (hir_id=HirId { owner: DefId(0:41 ~ traits[4135]::main), local_id: 364 }) in fcx 0x70000b479a30
        // let x: StructWithAccType1<B = i32> = StructWithAccType1{};

        compilation_error!(
        // error[E0191]: the value of the associated type `B` (from trait `TraitWithAccType`) must be specified
            let x: Box<dyn TraitWithAccType> = Box::new(StructWithAccType {});
        );
        compilation_error!(
            // error[E0038]: the trait `TraitWithAccType` cannot be made into an object
            let x: Box<dyn TraitWithAccType<B = i32>> = Box::new(StructWithAccType {});
        );
    }
    {
        // declare default "associated type"'s value type
        #[derive(Debug)]
        struct Point { x: i32, y: i32 }
        impl Add for Point {
            type Output = Point; // default type for 'Output'

            fn add(self, rhs: Self) -> Self::Output {
                Point {
                    x: self.x + rhs.x,
                    y: self.y + rhs.y
                }
            }
        }
        let p1 = Point { x: 2, y: 5};
        let p2 = Point { x: 8, y: 5};
        let p3 = p1 + p2;
        println!("p1 + p2 is {:?}", p3);
        assert_eq!((10, 10), (p3.x, p3.y));
    }
    {
        // Fully Qualified Syntax for Disambiguation: Calling Methods with the Same Name
        struct S {}
        trait WithFoo { fn foo(&self); }
        impl WithFoo for S {
            fn foo(&self) {
                println!("I am method from trait");
            }
        }
        impl S {
            fn foo(&self) {
                println!("I am method from S's implementation ");
            }
        }
        let s = S {};
        s.foo(); // I am method from S's implementation
        WithFoo::foo(&s); // I am method from trait

        let s: Box<dyn WithFoo> = Box::new( S {});
        s.foo(); // I am method from trait
    }
    {
        // qualification for associated functions
        trait Animal {
            fn baby_name() -> String;
        }

        struct Dog;

        impl Dog {
            fn baby_name() -> String {
                String::from("Spot")
            }
        }

        impl Animal for Dog {
            fn baby_name() -> String {
                String::from("puppy")
            }
        }
        println!("A baby dog is called a {}", Dog::baby_name()); // Spot
        compilation_error!(
            println!("A baby dog is called a {}", Animal::baby_name()); // type annotations needed
            // compiler cannot chose between two implementations of 'baby_name'
        );
        println!("A baby dog is called a {}", <Dog as Animal>::baby_name()); // puppy
    }
    {
        // Using Supertraits to Require One Trait’s Functionality Within Another Trait
        trait OutlinePrint: fmt::Display { // 'OutlinePrint' requires it's implementor to implement 'Display'
            fn outline_print(&self) {
                let output = self.to_string();
                let len = output.len();
                println!("{}", "*".repeat(len + 4));
                println!("*{}*", " ".repeat(len + 2));
                println!("* {} *", output);
                println!("*{}*", " ".repeat(len + 2));
                println!("{}", "*".repeat(len + 4));
            }
        }
        {
            #[allow(dead_code)]
            struct Point {
                x: i32,
                y: i32
            }
            compilation_error!(
                impl OutlinePrint for Point{} //  `main::Point` doesn't implement `std::fmt::Display`
            );
        }

        struct Point {
            x: i32,
            y: i32
        }
        impl Display for Point { // implementing of Display for Pint is required by OutlinePrint
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                write!(f, "{} {}", self.x, self.y)
            }
        }
        impl OutlinePrint for Point{}
        Point { x: 1, y: 2 }.outline_print();
    }
    {
        // Using the Newtype Pattern to Implement External Traits on External Types
        // no performance penalty
        struct Wrapper(Vec<String>); // vector is located in extern crate

        impl Deref for Wrapper {
            type Target = Vec<String>;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl Display for Wrapper {   // 'Display' also located in extern crate
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                write!(f, "[{}]", self.join(", ")) // no 'self.0.join...' due to implementing 'Deref'
            }
        }

        let w = Wrapper(vec![String::from("hello"), String::from("world")]);
        println!("w = {}, length is {}", w, w.len());
    }
}