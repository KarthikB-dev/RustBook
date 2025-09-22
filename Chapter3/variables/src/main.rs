fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    let x = 6;
    println!("The value of x is: {x}");
    {
        let x = 21;
        println!("The value of x (inner scope) is: {x}");
    }
    println!("After that scope is left, the value of x is: {x}");
    
    // Disallowed due to type mutation:
    // let mut spaces = "   ";
    let spaces = "   ";
    spaces = spaces.len();
    println!("Spaces: {spaces}")
}
