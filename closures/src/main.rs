use std::{thread, time::Duration};

#[derive(Debug, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);

    println!(
        "The user with  preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;

    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    // Capturing and moving values
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let only_borrows = || println!("From closure: {list:?}");

    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");

    let mut borrows_mutably = || list.push(4);
    borrows_mutably();
    println!("After calling borrowing mutably: {list:?}");

    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();

    let mut rects = [
        Rectangle {
            width: 20,
            height: 30,
        },
        Rectangle {
            width: 10,
            height: 20,
        },
        Rectangle {
            width: 15,
            height: 45,
        },
    ];

    rects.sort_by_key(|r| r.width);

    println!("{rects:#?}");

    // let mut sort_operations = vec![];
    let mut num_sort_operations = 0;

    // let value = String::from("closure called");

    // rects.sort_by_key(|r| {
    //     sort_operations.push(value);
    //     r.width
    // });

    rects.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
}
