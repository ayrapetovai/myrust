// 1. Given a list of integers, use a vector and return the
//   mean (the average value),
//   median (when sorted, the value in the middle position),
//   mode (the value that occurs most often; a hash map will be helpful here) of the list.

// Convert strings to pig latin.
// The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.”
// Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”).
// Keep in mind the details about UTF-8 encoding!

// Using a hash map and vectors, create a text interface to allow a user to add employee names
// to a department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.”
// Then let the user retrieve a list of all people in a department or all people in the company
// y department, sorted alphabetically.

fn main() {
    {
        let numbers = vec![1, 6, 2, 8, 5, 2, 9, 7, 4];
        println!("Given {:?}", numbers);

        fn mean(numbers: &Vec<i32>) -> i32 {
            numbers.iter().sum::<i32>() / numbers.len() as i32
        }
        println!("mean is {}", mean(&numbers)); // 4

        fn median(number: &Vec<i32>) -> Option<i32> {
            if number.len() > 0 {
                let mut sorted = number.clone();
                sorted.sort();
                Some(sorted[sorted.len() / 2])
            } else {
                None
            }
        }
        //         0  1  2  3  4  5  6  7  8
        // sorted [1, 2, 2, 4, 5, 6, 7, 8, 9]
        println!("median is {}", median(&numbers).unwrap()); // 5

        fn mode(numbers: &Vec<i32>) -> Option<i32> {
            if numbers.len() > 0 {
                let mut counters = std::collections::HashMap::new();
                for n in numbers {
                    let counter = counters.entry(n).or_insert(0);
                    *counter += 1
                }
                let mut found_key = *counters.keys().min().unwrap();
                let mut value = counters[found_key];
                for (k, v) in counters {
                    if v > value {
                        found_key = k;
                        value = v;
                    }
                }
                Some(*found_key)
            } else {
                None
            }
        }
        println!("mode is {}", mode(&numbers).unwrap()); // 2
    }

}