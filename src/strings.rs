use std::ops::Index;
extern crate myrust;
use self::myrust::compilation_error;

#[allow(unused)]
fn main() {
    {
        let s = String::from("Текст на русском языке");
        println!("{}", s);
    }
    {
        let literal = "initial content";
        let s = literal.to_string(); // : String
        if &s == &literal {
            println!("literal and it's to_string result are equal");
        }
    }
    {
        let mut s = String::from("foo");
        s.push_str("bar");
    }
    {
        let s1 = String::from("Hello, ");
        let s2 = String::from("world!");
        // String::add(mut self, other: &str) -> String <----- takes ownership of s1 as self
        let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
        println!("{}", s3);
    }
    {
        // Index<T> is not implemented for String, cannot access to chars by []-syntax
        let s = String::from("some string");
        compilation_error!(
            let c = s[2]; // the type `String` cannot be indexed by `{integer}`
        );
        struct A { x: i32, y: i32 }
        impl Index<usize> for A {
            type Output = i32;

            fn index(&self, index: usize) -> &Self::Output {
                if index == 0 {
                    &self.x
                } else {
                    &self.y
                }
            }
        }
        let a = A{ x:1, y: 2 };
        println!("Index<usize> implementations: strust a[] {} {}", a[0], a[10]);
    }
    {
        // This word occupies 24 bytes.
        let hello = "Здравствуйте";
        println!("size of 'hello' is {}", hello.len()); // 24
    }
    {
        // no indexed access to string's elements, use 'for in' to handle individual char
        let hello = "Здравствуйте";
        let s = &hello[0..4];
        println!("The slice is {}", s) // Зд
    }
    {
        let hello = "Здравствуйте";
        print!("Printing UTF-8 chars one by one: ");
        for c in hello.chars() {
            print!("|{}", c); // |З|д|р|а|в|с|т|в|у|й|т|е|
        }
        println!("|");

        print!("Printing bytes one by one: ");
        for b in "नमस्ते".bytes() {
            print!("|{}", b); // |224|164|168|224|164|174|224|164|184|224|165|141|224|164|164|224|165|135|
        }
        println!("|");
    }
}
