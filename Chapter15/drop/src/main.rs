struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`", self.data);
    }
}

fn main() {
    let first = CustomSmartPointer {
        data: String::from("Smart pointer 1"),
    };
    let second = CustomSmartPointer {
        data: String::from("Smart pointer 2"),
    };
    println!("Two smart pointers created!");
    // Explicit destructor calls are forbidden
    // first.drop(); won't work!
    drop(first);
}
