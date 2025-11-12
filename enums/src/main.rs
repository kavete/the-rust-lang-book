#![allow(unused)]

use std::fmt::format;
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

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    //etc
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
        }
    }
}

fn main() {
    let four = IpAddrKind::V4(String::from("127.0.0.1"));
    let six = IpAddrKind::V6(String::from("::1"));

    // Option<T> enum

    let some_number = Some(5);
    let some_char = Some('a');
    let absent_number: Option<i32> = None;

    // Match control flow
    //
    let five = Some(5);
    let six = plus_one(five);

    println!("{six:?}");

    // Concise control flow with if let and let else
    //
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }

    //Shorter way

    if let Some(max) = config_max {
        println!("The maximum is set to {max}");
    }

    let mut count = 0;

    let coin = Coin::Quarter(UsState::Alabama);

    //match coin {
    //  Coin::Quarter(state) => println!("State Quarter from {state:?}"),
    //  _ => (),
    // }

    if let Coin::Quarter(state) = coin {
        println!("State quarter from {state:?}!");
    } else {
        count += 1;
    }

    // Staying on the happy path with let else
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("The quarter is from {state:?}");
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn describe_state_quarter(coin: Coin) -> Option<String> {
    if let Coin::Quarter(state) = coin {
        if state.existed_in(1900) {
            Some(format!("{state:?} is pretty old"))
        } else {
            Some(format!("{state:?} is relatively new"))
        }
    } else {
        None
    }
}
