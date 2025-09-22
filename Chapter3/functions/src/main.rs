fn main() {
    println!("Hello, world!");
    another_function(5);
    measurement_function(32, 'C');

    let x = {
        let y = 3;
        y + 1
    };
    let five = return_five();
    let six = incr(five);
    println!("Variable six: {six}");

    /*
    This is a multiline comment!
    */

    // This is a single line comment!
}

fn incr(x: u32) -> u32 {
    x + 1
}

fn return_five() -> u32 {
    5
}

fn measurement_function(temp: u32, units: char) {
    println!("The temperature is {temp} degrees {units}");
}

fn another_function(i: u32) {
    println!("Wow, another function!");
    println!("Input is: {i}");
}
