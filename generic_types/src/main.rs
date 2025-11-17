#![allow(unused)]
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for i in list {
        if i > largest {
            largest = i
        }
    }
    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];
    for i in list {
        if i > largest {
            largest = i
        }
    }
    largest
}

// Using generic types in functions

// fn largest<T>(list: &[T]) -> &T {
//     let mut largest = &list[0];
//     for item in list {
//         if item > largest {
//             largest = item
//         }
//     }
//     largest
// }

// Generics in Struct definitions
struct Point<T> {
    x: T,
    y: T,
}

struct Point2<T, U> {
    x: T,
    y: U,
}

// In Enum definitions
// Examples are the Option and Result enums

enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

// In method definitions

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
// Implement for specific type

impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// Different generic type parameters in Struct definitions and in method definitions
//

struct Point3<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point3<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point3<X2, Y2>) -> Point3<X1, Y2> {
        Point3 {
            x: self.x,
            y: other.y,
        }
    }
}
fn main() {
    let numbers = vec![43, 76, 87];
    let largest = largest_i32(&numbers);

    println!("{largest}");

    let characters = vec!['A', 'B', 'C', 'D', 'd', 'a', 'c'];

    let largest = largest_char(&characters);

    println!("{largest}");

    //let generic_largest = largest(&numbers);

    let integer = Point { x: 5, y: 7 };

    let float = Point { x: 5.1, y: 7.0 };

    println!("X-coordinates {} and {}", float.x(), integer.x());

    let both_integer = Point2 { x: 5, y: 7 };

    let both_float = Point2 { x: 5.0, y: 7.0 };

    let mixed = Point2 { x: 5, y: 7.0 };

    let mixed2 = Point2 { x: 5.0, y: 7 };
}
