use object_oriented_design::Button;
use object_oriented_design::Draw;
use object_oriented_design::Post;

use object_oriented_design::Screen;

struct SelectBox {
    width: i32,
    height: i32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {}
}
fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 30,
                height: 50,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 40,
                height: 60,
                label: String::from("Press me!"),
            }),
        ],
    };

    screen.run();

    // Implementing Object oriented design pattern

    let mut post = Post::new();

    post.add_text("I ate ugali for lunch today");

    assert_eq!("", post.content());

    post.request_review();

    assert_eq!("", post.content());

    post.approve();

    assert_eq!("I ate ugali for lunch today", post.content())
}
