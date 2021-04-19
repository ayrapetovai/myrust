// orphan rule:
// One restriction to note with trait implementations is that we can implement a trait on a type
// only if either the trait or the type is local to our crate.
// We can’t implement external traits on external types.

use std::fmt::{Display, Formatter, Debug};

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
            /* compilation error
            fn greatest(&self) -> &T {                     // duplicate definitions for `greatest`
                println!("I don't know...");
                &self.first
            }
            */
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
}