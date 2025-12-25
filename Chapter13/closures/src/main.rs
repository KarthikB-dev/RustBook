use std::{thread, time::Duration};
#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}
struct Inventory {
    shirts: Vec<ShirtColor>,
}
impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        // If the unwrapped value is Some, it's returned as is
        // The else branch calls the closure here
        user_preference.unwrap_or_else(|| self.most_stocked())
    }
    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);

    println!("The user with preference {user_pref1:?} gets {giveaway1:?}");

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);

    println!("The user with preference {user_pref2:?} gets {giveaway2:?}");

    let expensive_closure = |num: u32| -> u32 {
        println!("Calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    let two_but_slow = expensive_closure(2);
    println!("Calling the closure: {two_but_slow}");

    fn add_one_v1(x: u32) -> u32 {
        x + 1
    }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x: u32| x + 1;
    let add_one_v4 = |x: u32| x + 1;

    let example_closure = |x| x;

    // This first call of the closure 'locks in' the
    // closure's input type to strings
    let s = example_closure(String::from("hello"));
    // This second call is erroneous as a result
    // let n = example_closure(5);
    // It needs to be like this!
    let n = example_closure(5.to_string());

    let mut list = vec![1, 2, 3];
    println!("Before definining closures: {list:?}");

    let only_borrows = || println!("From closure: {list:?}");

    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");

    let mut borrows_mutably = || list.push(7);

    borrows_mutably();
    println!("After calling closure: {list:?}");

    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();

    let mut rectangle_list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    // let mut sort_operations = vec![];
    let value = String::from("closure called");

    // This works, but we are trying a convoluted
    // example for fun!
    // rectangle_list.sort_by_key(|r| r.width);
    let mut num_sort_operations = 0;
    rectangle_list.sort_by_key(|r| {
        // sort_operations.push(value);
        num_sort_operations += 1;
        r.width
    });
    println!("{rectangle_list:?}, sorted in {num_sort_operations} operations");
}
