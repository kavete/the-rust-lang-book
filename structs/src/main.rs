#![allow(unused)]

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

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

    // Calculating area of a rectangle
    //
    // Using normal variables
    let width = 30;
    let height = 50;

    println!("The area of the rectangle is {}", area(width, height));

    //Using tuples

    let rect = (30, 50);

    println!("Area from tuple {}", area2(rect));

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("The area of a the rectangle is {}", area3(&rect1));

    // Method syntax
    //
    println!("The area of the rectangle is {}", rect1.area());

    if rect1.width() {
        println!("The rectangle has non zero width which is {}", rect1.width);
    }

    let rect2 = Rectangle {
        width: 20,
        height: 15,
    };

    let rect3 = Rectangle {
        width: 40,
        height: 30,
    };

    println!("Can rect1 hold rect2 {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3 {}", rect1.can_hold(&rect3));

    // Associated functions
    //
    let square_rect = Rectangle::square(30);

    print!("The square rect is {square_rect:?}");
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
