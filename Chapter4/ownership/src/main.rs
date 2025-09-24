fn main() {
    let s1 = String::from("Hello, world!");
    let s2 = s1;
    // Fails due to ownership transferring
    // println!("{s1}");
    let s3 = &s2;
    // Passes because a reference is used
    println!("{s3}");
    println!("{s2}");
}
