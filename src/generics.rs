use std::cmp::Ordering;

extern crate myrust;
use self::myrust::compilation_error;

// generalizing code with type-parameter
fn largest<T: PartialOrd>(a: &[T]) -> &T {
    let mut biggest_element = &a[0];
    for v in a {
        if biggest_element < v {
            biggest_element = v;
        }
    }
    biggest_element
}

#[allow(unused)]
fn main() {
    {
        let v = vec![1, 6, 2, 9, 4, 3];
        println!("biggest number in {:?} is {}", v, largest(&v));

        #[derive(Debug)]
        struct Vector {
            x: i32,
            y: i32,
        }
        impl PartialEq for Vector { // orphan rule - ok, Vector resides in this module
            fn eq(&self, other: &Self) -> bool {
                self.x == other.x && self.y == other.y
            }
        }
        impl PartialOrd for Vector {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Option::from((self.x * self.y).cmp(&(other.x * other.y)))
            }
        }

        let vectors = vec![Vector{ x: 1, y: 1 }, Vector{ x: 7, y: 2 }, Vector{ x: 2, y: 5 }];
        println!("vectors {:?} biggest is {:?}", vectors, largest(&vectors))
    }
    {
        // Generics arguments are a part of a type
        #[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
        struct Point<X, Y> {
            x: X,
            y: Y,
        }
        // in method definitions
        // impl <T> is needed for compiler to understand, that Point<T>'s 'T' is not a concrete type name, but a type-parameter name
        impl<T, Y> Point<T, Y> {
            fn x(&self) -> &T {
                &self.x
            }
        }

        let f_i_point = Point { x: 1.0, y: 3 };
        let mut i_f_point = Point { x: 42, y: 3.14 };
        compilation_error!(
            i_f_point = f_i_point; // mismatched types
        );
        i_f_point = Point { x: 1, y: 0.23 };

        println!("x part of point is {}", i_f_point.x());

        let mut points = vec![&Point {x: 4, y: 0}, &Point {x: 1, y: 1}, &Point {x: 0, y: 1}];
        points.sort();
        println!("Sorted points are {:?}", points);
        println!("Largest point is {:?}", largest(&points))
    }
}