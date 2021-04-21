extern crate unicode_segmentation;

use std::collections::HashSet;
use std::iter::FromIterator;

/*
// TODO how to write this in rust the right way
// Java
// given collections of chars, with spaces, replace spaces with '%20'.
// 'len' pints to the last meaning char, all symbols right to it are garbage and can be used for better purpose.
// The length of garbage tail is always enough to store the collections with spaces replaces with '%20'
// str = "asd asd d    ", len = 9 <- four additional spaces should be enough.
int urlify(char[] str, int len) {
    if (str == null || len <= 0 || 0 < str.length) {
        return 0;
    }
    int back_index = str.length - 1;
    for (int i = len - 1; i >= 0; i--) {
        if (str[i] == ' ') {
            str[back_index--] = '0';
            str[back_index--] = '2';
            str[back_index--] = '%';
        } else {
            str[back_index--] = str[i];
        }
    }
    back_index++;
    for (int i = 0; i < str.length - back_index; i++) {
        str[i] = str[i + back_index];
    }
    return str.length - back_index;
}
 */

#[allow(dead_code)]
fn urlify(s: &mut [char], length: usize) -> usize {
    if length == 0 || s.len() < length {
        return 0;
    }
    let mut back_index = s.len() - 1;
    let mut offset = 0usize;
    for i in (0..length).rev() {
        if s[i] == ' ' {
            s[back_index] = '0';
            if back_index > 0 {
                back_index -= 1;
            } else {
                offset += 1;
            }
            s[back_index] = '2';
            if back_index > 0 {
                back_index -= 1;
            } else {
                offset += 1;
            }
            s[back_index] = '%';
            if back_index > 0 {
                back_index -= 1;
            } else {
                offset += 1;
            }
        } else {
            s[back_index] = s[i];
            if back_index > 0 {
                back_index -= 1;
            } else {
                offset += 1;
            }
        }
    }
    back_index += 1 - offset;
    for i in 0..(s.len() - back_index) {
        s[i] = s[i + back_index];
    }
    return s.len() - back_index;
}

fn main() {
    {
// Given a list of integers, use a vector and return the
//   mean (the average value),
//   median (when sorted, the value in the middle position),
//   mode (the value that occurs most often; a hash map will be helpful here) of the list.

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
        println!("median is {}", median(&numbers).unwrap_or(-1)); // 5

        fn mode(numbers: &Vec<i32>) -> Option<i32> {
            if numbers.len() > 0 {
                let mut counters = std::collections::HashMap::new();
                for n in numbers {
                    let counter = counters.entry(n).or_insert(0);
                    *counter += 1
                }
                let mut found_number = numbers.first().unwrap();
                let mut number_counter = counters[found_number];
                for (k, v) in counters {
                    if v > number_counter {
                        found_number = k;
                        number_counter = v;
                    }
                }
                Some(*found_number)
            } else {
                None
            }
        }
        println!("mode is {}", mode(&numbers).unwrap_or(-1)); // 2
        println!("mode is {}", mode(&vec![1, 3, 2, 1, 2, 1]).unwrap_or(-1)); // 1
    }
    {
// Convert strings to pig latin.
// The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.”
// Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”).
// Keep in mind the details about UTF-8 encoding!
        extern crate unicode_segmentation;
        use unicode_segmentation::UnicodeSegmentation;
        fn to_pig_latin(txt: &str) -> String {
            let vowels: HashSet<char> = HashSet::from_iter(vec![
                'e', 'u', 'i', 'o', 'a',
                'у','е','ы', 'а', 'о', 'я', 'ё', 'и',
                'あ','い','う','え','お'].into_iter());
            let mut result = String::new();
            for word in txt.split_whitespace() {
                result.push_str(
                    if vowels.contains(&word.chars().nth(0).unwrap_or_else(|| ' ')) {
                        String::from(word) + "hay"
                    } else {
                        let w = word.graphemes(true).skip(1).collect::<String>();
                        w + word.graphemes(true).take(1).collect::<String>().as_str() + "ay"
                    }.as_str()
                );
                result.push(' ');
            }
            result
        }
        let text = "first apple".to_string();
        println!("Text: '{}', pigged: '{}'", text, to_pig_latin(&text));
        let text = "съешь еще этих мягких французских булок".to_string();
        println!("Text: '{}', pigged: '{}'", text, to_pig_latin(&text));
        let text = "この　世界は　いいと　訝しまない".to_string(); // この　せかいは　いいと　いぶかしまない - vowel splitting does not work :(
        println!("Text: '{}', pigged: '{}'", text, to_pig_latin(&text));
    }
    {
// Using a hash map and vectors, create a text interface to allow a user to add employee names
// to a department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.”
// Then let the user retrieve a list of all people in a department or all people in the company
// y department, sorted alphabetically.
    }
}

#[cfg(test)]
mod tests {
    use urlify;

    fn create_test_string(s: &str) -> Vec<char> {
        let mut result: Vec<char> = Vec::with_capacity(s.len());
        let mut count = 0usize;
        for c in s.chars() {
            if c == ' ' {
                count += 1;
            }
            result.push(c);
        }
        let mut tail = vec![' '; count * 2];
        result.append(&mut tail);
        result
    }

    fn check_result(expected: &str, v: &Vec<char>, length: usize) {
        assert_eq!(expected.len(), length);
        let mut i = 0;
        for c in expected.chars() {
            assert_eq!(c, v[i]);
            i += 1;
        }
    }

    fn check(original: &str, expected: &str) {
        println!("Original '{}'", original);
        let mut result = create_test_string(original);
        println!("Target {:?}", result);
        let result_len = urlify(&mut result, original.len());
        println!("Result {:?}, length {}", result, result_len);
        check_result(expected, &result, result_len);
    }

    #[test]
    fn urlify_nochars_zero_len() {
        let mut v = vec![];
        let result_len = urlify(&mut v, 0);
        check_result("", &v, result_len);
    }

    #[test]
    fn urlify_nochars_some_len() {
        check("", "");
    }

    #[test]
    fn urlify_onechar_one_len() {
        check("a", "a");
    }

    #[test]
    fn urlify_one_space_one_len() {
        check(" ", "%20");
    }

    #[test]
    fn urlify_one_space_four_len() {
        check("ab c", "ab%20c");
    }

    #[test]
    fn urlify_three_space_four_len() {
        check(" abc c-1 ", "%20abc%20c-1%20");
    }
}