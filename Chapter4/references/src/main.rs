fn main() {
    println!("Hello, world!");
    let s = String::from("This is Karthik!");
    let _greeting = borrow(&s);
    // Can't run because of the move
    // println!("s: {s}");
    transfer_ownership(s);
    let mut greeting = String::from("It was nice meeting you!");
    mutate_reference(&mut greeting);
    let immutable_reference = &greeting;
    let mutable_reference = &mut greeting;
    // Disallowed because a mutable reference
    // was defined before - we don't know if 
    // that reference will modify the output
    // we're expecting
    // println!("{immutable_reference}");
    let dangling_reference_avoider = no_dangling_reference();
    println!("{dangling_reference_avoider}");
}

fn borrow(s : &String) -> &String {
    return s;
}

fn transfer_ownership(s : String) {
    println!("s: {s}");
}

fn mutate_reference(s: &mut String) {
    s.push_str(" Have a wonderful day!");
}

// Disallowed as the variable s will be cleared after the
// function runs
// fn dangling_reference() {
//     let s = String::from("I am a dangling reference!");
//     return &s
// }

fn no_dangling_reference() -> String {
    let s = String::from("I am *not* a dangling reference!");
    s
}