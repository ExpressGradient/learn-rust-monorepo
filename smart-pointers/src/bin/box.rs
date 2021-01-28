use crate::List::{Cons, Nil};

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32) // Max space needed to store the Message type.
}

enum List {
    Cons(i32, Box<List>),
    Nil
}

fn main() {
    let b: Box<i32> = Box::new(5);
    println!("b: {}", b);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}