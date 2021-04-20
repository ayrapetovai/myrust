fn main() {

}

// '$ cargo test' builds a test binary, runs it in multiple threads, captures it's output and prints it only if test function failed

#[cfg(test)] // this tells to compile and run code only if '$ cargo test' was called, not '$ cargo build'.
mod tests {

    /*
Running target/debug/deps/tests-2ddcc18bee1289a1

running 2 tests
test tests::test_with_assert ... ok
test tests::test_panic ... ok
     */
    #[test]
    #[should_panic]
    fn test_panic() {
        panic!("Test panic handling");
    }

    #[test]
    fn test_with_assert() {
        let is_four_equals_to_sum_of_two_and_two = 4 == 2 * 2;
        assert!(is_four_equals_to_sum_of_two_and_two, "4 is not 2 * 2");
    }

    #[test]
    fn test_if_two_plus_two_is_four() -> Result<(), String> { // test suite can use Result
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    #[test]
    #[ignore]
    fn ignored_test() {
        println!("this test will be ignored wile run '$ cargo test', but can be run explicitly by '$ cargo test -- --ignored' <- run ignored tests only");
    }

}