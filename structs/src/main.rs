#![allow(unused)]

fn main() {
    println!("Hello, world!");

    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    let mut user1 = User {
        email: String::from("etevak@mail.com"),
        username: String::from("etevak"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("another@example.com");

    fn build_user(email: String, username: String) -> User {
        User {
            active: true,
            username, // username: username,
            email,    // email: email,
            sign_in_count: 1,
        }
    }

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    let user3 = User {
        email: String::from("another2@mail.com"),
        ..user2
    };

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let Point(x, y, z) = origin;

    // Structs without any field

    struct AlwaysEqual;
    let subject = AlwaysEqual;
}
