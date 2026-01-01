#[derive(Debug)]
enum List {
    // A subpar implementation 
    // that makes it impossible to have an
    // in degree greater than one
    // Cons(i32, Box<List>),
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
	let x = 5;	
	// Causes an error: mutable reference to an immutable variable!
	// let y = &mut x;
	let value = Rc::new(RefCell::new(5));

	let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

	let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
	let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

	*value.borrow_mut() += 10;

	println!("a after = {a:?}");
	println!("b after = {b:?}");
	println!("c after = {c:?}");
}
