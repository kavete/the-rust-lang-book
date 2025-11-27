// Lifetime annotations in Structs
use std::fmt::Display;

struct ImportantExcerpt<'a> {
    part: &'a str,
}


fn longest_with_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str 
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() { x } else { y }
}

fn main() {

    // Will not compile with the error // Value does not live long enough
    // let r;
    //
    // {
    //     let x = 5;
    //     r = &x;
    // }
    // println!("r = {}", r);
    //

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest is {result}");

    //
    let novel = String::from("One night in the smoky hut of Aketch...");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt{
        part: first_sentence,
    };

    // Static lifetime - It denotes that the affected reference can live for the entire life of the
    // program. All string literals have the 'static lifetime

    let s: &'static str = "Static lifetime";
    println!("s = {s}");

    // Generic types, Lifetimes and Trait Bounds in one function
}

fn longest<'a>(x: &'a str, y:&'a str) -> &'a str {
    if x.len() > y.len() {
        x 
    } else {
            y
    }
}
