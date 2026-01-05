use std::thread;
use std::sync::{Arc, Mutex};

fn main() {
    // let m = Mutex::new(5);

    // {
    //     // unwrap makes the thread panic if another thread holding the lock
    //     // panics
    //     let mut num = m.lock().unwrap();
    //     *num = 6;
    // }

    // println!("m = {m:?}");

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        // All the threads try to increment the counter!
        // We need an atomic ref counter for thread safety
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
