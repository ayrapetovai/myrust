use std::path::StripPrefixError;

#[derive(Debug)]
struct Verbose {
    number: i32
}
impl Drop for Verbose {
    fn drop(&mut self) {
        println!("Verbose object {} is dropped", self.number);
    }
}

#[allow(unused)]
fn main() {
    // collections are stored at the heap
println!("********* vectors");
    {
        // holds values of the same type
        let mut v = Vec::new(); // an empty vector
        v.reserve(2);
        v.push(1);
        v.push(42);
        println!("Vector's content is {:?}", v); // [1, 42]
        {
            let second = &mut v[1];
            *second = 13;
            println!("Vector's 2nd element is {}", second); // 13
        }
        println!("Vector after mutation {:?}", v) // [1, 13]
    } // <- v goes out of scope and is freed here
    {
        // to create vector with elements macro is used
        let v = vec![1, 2, 3];
        println!("Non-empty-created vector is {:?}", v);
        let empty: Vec<i32> = vec!();
    }
    {
        // to create vector with elements macro is used
        let v = vec![42; 10]; // 10 elements of value 42
        println!("10-empty-created vector is {:?}", v);
        // comparing vector and array
        assert_eq!(v, [42; 10]);

        // accessing vector's non-existing element, checking "if-not-null"
        match v.get(12) {
            Some(_) => println!("No way..."),
            None => println!("Element with index 12 is not found")
        }
        if let Some(x) = v.get(1) {
            println!("The 1st element is {}", x);
        }
    }
    {
        let v = vec![Verbose{ number: 1 }, Verbose{ number: 2 }];
        // 'v' owns objects
        println!("Vector of Verbose objects is {:?}", v);
    } // 'v' was dropped => Verbose object 1 and Verbose object 2 was dropped
    {
        // iterating over a vector
        let mut v = vec![1, 2, 3];
        for i in &mut v {
            *i += 50;
        }
        println!("Each element get +50, vector is mutated to {:?}", v); // [51, 52, 53]
    }
    {
        // Using an Enum to Store Multiple Types
        pub enum SpreadSheetSell {
            Int(i32),
            Float(f32),
            Text(String),
        }
        impl Drop for SpreadSheetSell {
            fn drop(&mut self) {
                println!(
                    "Dropping SpreadSheetSell({})",
                    match self {
                        SpreadSheetSell::Int(i) => format!("{}", i),
                        SpreadSheetSell::Float(f) => format!("{}", f),
                        SpreadSheetSell::Text(t) => format!("{}", t)
                    }
                )
            }
        }
        let row = vec![
            SpreadSheetSell::Float(3.14),
            SpreadSheetSell::Int(42),
            SpreadSheetSell::Text(String::from("Hello")),
        ];
        for s in &row {
            match s {
                SpreadSheetSell::Float(f) => println!("Some float value {}", f),
                SpreadSheetSell::Text(t) => println!("Some text value {}", t),
                SpreadSheetSell::Int(i) => println!("Some integer value {}", i),
            }
        }
    }
}
