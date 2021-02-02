use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil
}

use crate::List::{Cons, Nil};

fn main() {
    let a: Rc<List> = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("Rc count: {}", Rc::strong_count(&a));
    let _b: List = Cons(3, Rc::clone(&a));
    println!("Rc count: {}", Rc::strong_count((&a)));
    {
        let _c: List = Cons(4, Rc::clone(&a));
        println!("Rc count: {}", Rc::strong_count((&a)));
    }
    println!("Rc count: {}", Rc::strong_count((&a)));
}