#![allow(unused)]
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello!")
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {value}");
        } else if value > 100 {
            panic!("Value should be less or equal to 100, got {value}");
        }
        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<(), String> {
        let result = add(2, 2);
        if result == 4 {
            Ok(())
        } else {
            Err(String::from("2 + 2 does not equal 4"))
        }
        // assert_eq!(result, 4);
    }

    #[test]
    #[ignore = "To check integrated tests"]
    fn another() {
        panic!("A failed test");
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };

        let smaller = Rectangle {
            width: 5,
            height: 4,
        };

        assert!(larger.can_hold(&smaller));
    }
    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };

        let smaller = Rectangle {
            width: 5,
            height: 4,
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        let result = add_two(5);
        assert_eq!(result, 7);
    }

    #[test]
    #[ignore]
    fn greeting_contains_name() {
        let result = greeting("Brian");
        assert!(
            result.contains("Brian"),
            "Greeting did not contain Brian, value was  `{result}`"
        );
    }

    #[test]
    #[should_panic(expected = "less or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
}
