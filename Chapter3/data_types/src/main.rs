fn main() {
    let sum = 5;
    let difference = 95.5 - 4.3;
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3;
    let remainder = 43 % 5;
    let f: bool = false;
    let z: char = 'ℤ';
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is {y}");
    let x = tup;
    let first = x.0;
    let second = x.1;
    let third = x.2;
    // Array
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    // Three instances of one
    let b = [1; 3];
    let first = a[1];
}
