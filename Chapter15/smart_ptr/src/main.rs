use std::ops::Deref;

struct MyBox<T>(T);

fn hello(name: &str) {
    println!("Hello, {name}!");
}

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

fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(x, 5);
    assert_eq!(*y, 5);

    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    // Version without deref coercion.
    // It's painfully verbose...
    hello(&(*m)[..]);
}
