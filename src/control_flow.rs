// TODO match
fn main() {
    {
        let some_u8_value = Some(0u8);
        match some_u8_value {
            Some(3) => println!("three"),
            _ => (),
        }
        // consider equivalent
        // match one pattern and ignore others
        let x = Some(42);
        if let Some(42) = x {
            println!("42!");
        }
    }
    {
        fn returns_none_or_number(number: bool) -> Option<i32> {
            if number {
                Some(12)
            } else {
                None
            }
        }
        // "destructing" Option
        fn test_and_print_plus_one(op: Option<i32>) {
            if let Some(var) = op {
                println!("returned + 1 is {} ", var + 1);
            } else {
                println!("Empty value :(");
            }
        }
        test_and_print_plus_one(returns_none_or_number(true));
        test_and_print_plus_one(returns_none_or_number(false));
    }
}
