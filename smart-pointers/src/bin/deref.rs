use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}", name);
}

fn main() {
    let x: u32 = 5;
    let y: &u32 = &x;
    let box_y: Box<u32> = Box::new(x);
    let my_box_y: MyBox<u32> = MyBox::new(x);

    assert_eq!(x, *y);
    assert_eq!(x, *box_y);
    assert_eq!(x, *my_box_y);

    let m: MyBox<String> = MyBox::new(String::from("Express Gradient"));
    hello(&m);
}