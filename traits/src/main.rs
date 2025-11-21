use traits::aggregator::{SocialPost, Summary};
fn main() {
    let post = SocialPost {
        username: String::from("kavete"),
        content: String::from("I think therefore I am"),
        reply: false,
        repost: false,
    };

    println!("1 new post {}", post.summarize());

    // Traits as parameters
}
