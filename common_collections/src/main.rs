#![allow(unused)]

pub mod utf_encoding;
fn main() {
    let v: Vec<i32> = Vec::new();

    let v2 = vec![4, 5, 8, 9, 23];

    // Updating a vector

    let mut numbers = Vec::new();

    numbers.push(5);
    numbers.push(6);
    numbers.push(7);

    // Reading elements of a vector

    let third = &v2[2];

    println!("Third element {third}");

    let third = v2.get(2);

    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element"),
    }

    // Iterating over values in a vector

    for i in &v2 {
        println!("{i}");
    }

    // Iterating over a mutable reference to make changes to the vector

    for i in &mut numbers {
        *i += 60;
    }

    // Using am enum to store multiple types

    enum SpreadSheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadSheetCell::Int(5),
        SpreadSheetCell::Float(7.5),
        SpreadSheetCell::Text(String::from("CellA2")),
    ];

    // Dropping a vector drops it's elements
    //
    // Storing UTF-encoded text with strings

    utf_encoding::strings::play_with_strings();
}
