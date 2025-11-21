pub mod aggregator {
    #![allow(unused)]
    use std::fmt::Display;

    pub trait Summary {
        fn summarize_author(&self) -> String;
        fn summarize(&self) -> String {
            // Default implementation
            format!("(Read more from {}... )", self.summarize_author())
        }
    }

    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    impl Summary for NewsArticle {
        // fn summarize(&self) -> String {
        //     format!("{}, by {} ({})", self.headline, self.author, self.location)
        // }

        fn summarize_author(&self) -> String {
            format!("@{}", self.author)
        }
    }

    pub struct SocialPost {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub repost: bool,
    }

    impl Summary for SocialPost {
        // fn summarize(&self) -> String {
        //     format!("({}: {})", self.username, self.content)
        // }

        fn summarize_author(&self) -> String {
            format!("@{}", self.username)
        }
    }

    // pub fn notify(item: &impl Summary) {
    //     println!("Breaking news {}", item.summarize());
    // }

    // Trait bound syntax

    pub fn notify<T: Summary>(item: &T) {
        println!("Breaking news {}", item.summarize());
    }

    // Return types that implement Traits
    //

    fn returns_summarizable() -> impl Summary {
        SocialPost {
            username: String::from("nairb"),
            content: String::from("I think therefore I am"),
            reply: false,
            repost: false,
        }
    }
    // Conditionally implement methods
    //
    struct Pair<T> {
        x: T,
        y: T,
    }

    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
    }

    impl<T: Display + PartialOrd> Pair<T> {
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("The largest member is x = {}", self.x);
            } else {
                println!("The largest member is y = {} ", self.y);
            }
        }
    }
}
