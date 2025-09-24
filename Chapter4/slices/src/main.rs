// Making the input type here &str means we 
// can call this function on both string slices
// and a string reference 
fn first_word(s : &str) -> &str {
    let bytes = s.as_bytes();
    let mut idx = 0;
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            idx = i;
            break;
        }
    }
    return &s[..idx];
}
fn immutable_word(s : &String) {
    println!("Word: {s}");
}
fn main() {
    let a = String::from("Hello there!");
    // The zero can be omitted 
    let b = &a[..5];
    println!("b: {b}");
    // Slice that extracts the entire string
    let a_copy = &a[..];
    println!("a_copy: {a_copy}");
    // Get the first word from "Hello there!"
    let first_word_a = first_word(&a);
    println!("{first_word_a}");
    let mut s = String::from("Hello world!");
    let word = first_word(&s);
    // Mutable references can't be declared after 
    // ownership has been transferred
    // s.clear();
    println!("{word}");
}
