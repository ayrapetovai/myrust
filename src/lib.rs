
macro_rules! compilation_error {
    ($e:stmt $(;)?) => {}
}

// src/main.rs and src/lib.rs are "crate roots", their content forms "module tree"
// there can be only one lib.rs in a package, it is a crate, it's name is the same as package name,
// it is 'extern' to other crates (executables) in this package.

// module tree
// crate
// └── front_of_house
//     ├── hosting
//     │   ├── add_to_waitlist
//     │   └── seat_at_table
//     └── serving
//         ├── take_order
//         ├── serve_order
//         └── take_payment

// In order to find and item we must know a path in a module tree
// Paths can be:
// 1. "absolut" (starts from the crate root, by word 'crate')
// 2. "relative" (from current module, using 'super' and 'self' or identifier).
// '::' path separator

// 'mod' defines a module, all members are private by default
// front_of_house is implicitly child of module crate
#[allow(unused)]
mod front_of_house {
    // hosting is a 'child' module of module front_of_house
    // hosting module is sibling to serving
    pub mod hosting { // 'pub' makes names available from outside of this scope
        pub fn add_to_waitlist() {}
        pub fn seat_at_table() {}
        fn prepare() { // private function, making module public does not affect on it's children
            // relative path, relative to this module
            add_to_waitlist();
            // relative path, relative to this module
            self::seat_at_table();
            // path, relative to this module
            self::super::look_at_the_personal();
        }
    }
    // serving is a 'child' module of module front_of_house
    // serving module is sibling to hosting
    mod serving {
        fn take_order() {
            // path, relative to this module
            super::look_at_the_personal()
        }
        fn serve_order() {}
        fn take_payment() {}
    }

    // privat for outsiders of front_of_house module, but accessible for children
    fn look_at_the_personal() {}
}

fn server_order() {}

mod back_of_house {
    #[derive(Debug)]
    pub struct Breakfast {
        pub toasts: String,
        seasonal_fruit: String // this field is private for siblings
    }

    // 'impl' cant be prefixed with 'pub'
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toasts: String::from(toast),
                seasonal_fruit: String::from("peaches")
            }
        }
    }

    // enum variants are public by default
    #[allow(dead_code)]
    pub enum Appetizer {
        Soup,
        Salad,
    }
    #[allow(unused)]
    fn fix_incorrect_order() {
        // this is not crate::front_of_house::serving::server_order
        super::server_order();
        cook_order();
    }

    fn cook_order() {}
}

#[allow(path_statements)]
pub fn eat_at_restaurant() {
    // absolut path, that is because of use of 'crate::'
    crate::front_of_house::hosting::add_to_waitlist();

    front_of_house::hosting::seat_at_table();

    compilation_error!(
        front_of_house::hosting::prepare(); // function `prepare` is private
    );

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toasts = String::from("Wheat");

    compilation_error!(
        meal.seasonal_fruit = String::from("blueberries"); // field `seasonal_fruit` of struct `Breakfast` is private
    );

    println!("I would like {:?}", meal);

    // enum's variants are public by default
    back_of_house::Appetizer::Salad; // warning: path statement with no effect
}

#[allow(unused)]
fn eat_at_restaurant_2() {
    // introduces names in to the scope
    {
        use front_of_house::hosting::*;
        add_to_waitlist();
        seat_at_table();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
