// the name and location of this directory are reserved for integration tests
// each file int '${PROJECT_ROOT}/tests' compiles like individual crate
// files in directory '${PROJECT_ROOT}/tests' are complied only by command '$ cargo test', not build

// integration tests works like separate crate and needs imports

extern crate myrust;
use myrust::compilation_error;

#[test]
fn compilation_error_macro_works_for_function_definitions() {
    compilation_error!(
        fn foo() {
            error // error
        }
    );
}

#[test]
fn compilation_error_macro_works_for_structure_definitions() {
    compilation_error!(
        struct Foo {
            x: y // error
        }
    );
}

#[test]
fn compilation_error_macro_works_for_variable_declarations() {
    compilation_error!(
        let x = y // error
    );
}