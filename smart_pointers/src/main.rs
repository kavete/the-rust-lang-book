#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

pub mod reference_counting;

use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`", self.data)
    }
}

fn main() {
    // use List::{Cons, Nil};
    let b = Box::new(5);
    println!("b = {b}");

    let x = 5;
    let y = &x;
    let z = Box::new(5);
    let z2 = MyBox(5);

    // println!("{y}");

    assert_eq!(5, x);
    assert_eq!(5, *y);
    assert_eq!(5, *z);

    assert_eq!(5, *z2);

    let list = List::Cons(
        1,
        Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
    );

    println!("{:?}", list);

    let c = CustomSmartPointer {
        data: String::from("Declared First"),
    };

    let d = CustomSmartPointer {
        data: String::from("Declared Second"),
    };

    println!("CustomSmartPointers created");

    drop(d);
    println!("Custom smart pointer dropped before the end of main");

    reference_counting::rc_test();
}
