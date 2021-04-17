// crate is a library or binary (executable)
// there are lib.rs in this project, it is a separate crate, need to include it explicitly.
extern crate myrust; // this is for 'use myrust::eat_at_restaurant;'

fn main() {
    {
        println!("Let's go to restaurant!");
        myrust::eat_at_restaurant();
    }
    {
        use myrust::eat_at_restaurant;
        eat_at_restaurant();
    }
    {
        use myrust::eat_at_restaurant as ololo_func;
        ololo_func();
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

