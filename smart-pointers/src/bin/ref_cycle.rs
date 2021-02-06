use std::cell::RefCell;
use std::rc::{ Rc, Weak };
use self::List::{ Cons, Nil };

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None
        }
    }
}

#[derive(Debug)]
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
    parent: RefCell<Weak<Node>>
}

fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    println!("a initial rc count: {}", Rc::strong_count(&a));
    println!("a next item: {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
    println!("a rc count after creation of b: {}", Rc::strong_count(&a));
    println!("b initial rc count: {}", Rc::strong_count(&b));
    println!("b next item: {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a: {}", Rc::strong_count(&b));
    println!("a rc count after changing a: {}", Rc::strong_count(&a));

    // Causes stack overflow
    // println!("a next item: {:?}", a.tail());

    {
        let leaf: Rc<Node> = Rc::new(Node {
            value: 3,
            children: RefCell::new(vec![]),
            parent: RefCell::new(Weak::new())
        });
        println!("Leaf parent: {:?}", leaf.parent.borrow().upgrade());

        println!("Leaf strong rc count: {}", Rc::strong_count(&leaf));
        println!("Leaf weak rc count: {}", Rc::weak_count(&leaf));

        {
            let branch: Rc<Node> = Rc::new(Node {
                value: 5,
                children: RefCell::new(vec![Rc::clone(&leaf)]),
                parent: RefCell::new(Weak::new())
            });

            *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
            println!("Leaf parent: {:?}", leaf.parent.borrow().upgrade());

            println!("Leaf strong rc count: {}", Rc::strong_count(&leaf));
            println!("Leaf weak rc count: {}", Rc::weak_count(&leaf));

            println!("Branch strong rc count: {}", Rc::strong_count(&branch));
            println!("Branch weak rc count: {}", Rc::weak_count(&branch));
        }

        println!("Leaf parent: {:?}", leaf.parent.borrow().upgrade());

        println!("Leaf strong rc count: {}", Rc::strong_count(&leaf));
        println!("Leaf weak rc count: {}", Rc::weak_count(&leaf));
    }
}