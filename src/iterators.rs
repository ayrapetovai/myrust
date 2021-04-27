extern crate myrust;
extern crate rand;

use myrust::compilation_error;
use myrust::Verbose;
use rand::Rng;

fn main() {
    // iterators are lazy
    {
        // immutable elements iterator
        let v = vec![1, 2, 3];
        let it = v.iter(); // iterator did not start to iterate yet
        // cycle calls it.next(), next rakes &mut self, however, 'it' is not mutable
        // works because for-loop takes ownership of iterator
        for x in it { // x is an immutable reference
            compilation_error!(
                *x = 1; // `x` is a `&` reference, so the data it refers to cannot be written
            );
            println!("value of iterator is {}", x);
        }
    }
    {
        // mutable elements iterator
        let mut v = vec![1, 2, 3];
        let it = v.iter_mut();
        for x in it { // x is a mutable reference
            *x = 1;
            println!("value of iterator is {}", x); // vector's element was changed
        }
    }
    {
        // ownership taking iterator: into_iter
        let verboses = vec![
            Verbose { id: 1 },
            Verbose { id: 2 },
            Verbose { id: 3 }
        ];
        let it = verboses.into_iter();
        for x in it { // x is a variable that got ownership
            println!("value of iterator is {:?}", x); // vector's element was changed
            // x goes out of scope here, and it owns an element... so element is dropped here
        }
        // `verboses` moved due to '.into_iter()' method call, this function takes ownership of the receiver `self`, which moves `verboses`
        compilation_error!(
            let v1 = verboses[1]; // borrow of moved value: `verboses`
        );
        println!("After iteration with taking ownership");
    }
    {
        // the Iterator trait has method 'fn next(&mut self) -> Option<Self::Item>'
        // when iterator is over it returns None.
        let v = vec![1, 2, 3];
        let mut it = v.iter();
        assert_eq!(Some(&1), it.next());
        assert_eq!(Some(&2), it.next());
        assert_eq!(Some(&3), it.next());
        assert_eq!(None, it.next());
        assert_eq!(None, it.next()); // no exceptions
        println!("iterator does not become invalid since it got to an end of the collection");
    }
    {
        // consuming adapters - methods that call 'next' on an iterator
        let v = vec![1, 2, 3];
        let it = v.iter();
        let sum_of_elements: i32 = it.sum();
        println!("Sum of elements {:?} is {}", v, sum_of_elements);
        assert_eq!(6, sum_of_elements);

        // iterator is no longer valid
        compilation_error!(
            for x in it {} // use of moved value: `it`, `it` moved due to 'it.sum()' method call
        );
    }
    {
        // produce other iterators
        let v = vec![1, 2, 3, 4];
        println!("Having vector {:?}", v);
        let plus_one_iter = v.iter().map(|x| x + 1); // 'map' produces an iterator
        for x in plus_one_iter {
            println!("Element with 'plus one iterator' is {}", x);
        }
        // let collected = v.iter().map(|x| x + 1).collect() as Vec<i32>;
        let collected: Vec<_> = v.iter().map(|x| x + 1).collect();
        println!("Collected {:?}", collected);
        assert_eq!(collected, vec![2, 3, 4, 5]);
    }
    {
        // rust can "unroll" (развернуть) a loop over iterator, when it knows how many iterations must take place
        let sum_of_random_ten_elements =
            [rand::thread_rng().gen_range(1..100); 10]
                .iter()
                .map(|x| x * x)
                .sum::<i32>();
        println!("sum_of_random_ten_elements is {}", sum_of_random_ten_elements);
    }
}

struct Counter {
    count: i32
}

impl Counter {
    #[allow(unused)]
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 9 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

#[test]
fn test_counter() {
    let mut c = Counter::new();
    assert_eq!(Some(1), c.next());
    assert_eq!(Some(2), c.next());
    assert_eq!(Some(3), c.next());
    assert_eq!(Some(4), c.next());
    assert_eq!(Some(5), c.next());
    assert_eq!(Some(6), c.next());
    assert_eq!(Some(7), c.next());
    assert_eq!(Some(8), c.next());
    assert_eq!(Some(9), c.next());
    assert_eq!(None, c.next());
}

#[test]
fn using_other_iterator_trait_methods() {
    let r = Counter::new()
        .zip(Counter::new())
        .skip(1)
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .reduce(|x, y| x + y);
    assert_eq!(Some(126), r);
}