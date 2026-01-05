use std::thread;
use std::time::Duration;

fn main() {
    // The main thread can finish first and stop the spawned thread
    // prematurely!
    // thread::spawn(|| {
    //     for i in 1..10 {
    //         println!("hi number {i} from spawned thread!");
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });

    // for i in 1..5 {
    //     println!("hi number {i} from the main thread!");
    //     thread::sleep(Duration::from_millis(1));
    // }

    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    // Using join ensures that the 'handle' thread can finish
    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    // handle.join().unwrap();

    let v = vec![1, 2, 3];

    // We need the closure to take ownership of v to ensure that it
    // doesn't go out of scope => use the move keyword. This is because
    // Rust doesn't know how long the thread will run.
    let _handle = thread::spawn(move || {
        println!("Here's a vector: {v:?}");
    });

    // Mayhem!
    // This can't be done because the value was moved to the closure!
    // drop(v);

    _handle.join().unwrap();
}
