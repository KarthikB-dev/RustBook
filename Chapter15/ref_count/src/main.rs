use crate::List::{Cons, Nil};
use std::rc::Rc;

enum List {
    // A subpar implementation
    // that makes it impossible to have an
    // in degree greater than one
    // Cons(i32, Box<List>),
    Cons(i32, Rc<List>),
    Nil,
}

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("After creating a, its count is: {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!(
        "After creating b, a's ref count is: {}",
        Rc::strong_count(&a)
    );
    // This is disallowed because a has already been
    // moved to b (ownership has been transferred)
    // let c = Cons(4, Box::new(a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!(
            "After creating c, a's ref count is: {}",
            Rc::strong_count(&a)
        );
    }
    println!(
        "After c goes out of scope, a's ref count is: {}",
        Rc::strong_count(&a)
    );
    println!("Hello, world!");
}
