#![allow(unused)]
enum IpAddrKind {
    V4(String),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let four = IpAddrKind::V4(String::from("127.0.0.1"));
    let six = IpAddrKind::V6(String::from("::1"));

    // Option<T> enum

    let some_number = Some(5);
    let some_char = Some('a');
    let absent_number: Option<i32> = None;

    // Match control flow
}
