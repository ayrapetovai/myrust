// use myrust::eat_at_restaurant; // ok

fn main() {
    {
        println!("Let's go to restaurant!");
        myrust::eat_at_restaurant();
    }
    {
        // TODO should work! It woks only in a separate package
        // use myrust::eat_at_restaurant;
        // eat_at_restaurant();
    }
    {
        // TODO should work! It woks only in a separate package
        // use myrust::eat_at_restaurant as ololo_func;
        // ololo_func();
    }
    {
        use std::collections::HashMap;
        let mut hm = HashMap::new();
        hm.insert("one", 1);
        println!("First access: The 'one' key's value is {}", hm["one"]);
        println!("Second access: The 'one' key's value is {}", hm["one"]);
    }
    {
        // nested paths
        use std::io::{self, Write};
        // Glob operator <- import all names
        use std::io::*;
    }
}

