fn less_than_five(number: u32) {
    if number < 5 {
        println!("The number is less than 5!");
    } else {
        println!("The number is greater than or equal to 5!");
    }
}

fn fahrenheit_to_celsius(degrees: u32) -> u32 {
    return (degrees - 32) * 5 / 9;
}

fn celsius_to_fahrenheit(degrees: u32) -> u32 {
    return (degrees + 32) * 9 / 5;
}

fn gen_fib(idx: u32) -> u32 {
    let mut two_before = 1;
    let mut prev = 1;
    if idx == 1 || idx == 2 {
        return prev;
    } else {
        let mut curr = 2;
        let mut tmp = 0;
        let mut curr_idx = 3;
        while curr_idx <= idx {
            curr = prev + two_before;
            two_before = prev;
            prev = curr;
            curr_idx += 1;
        }
        return curr;
    }
}

fn main() {
    let five = 5;
    less_than_five(five);
    let four = 4;
    less_than_five(four);

    // Rust can't evaluate u32 as a boolean!
    // if five {
    //     println!("The number is greater than or equal to 5!");
    // }

    if five < 5 {
        println!("Five is less than five hmmmm");
    } else if five > 5 {
        println!("Five is greater than five hmmm");
    } else {
        println!("Five is equal to five!");
    }

    // If statements can be rvalues!
    let year = 2004;
    let num_days_in_feb = if year % 4 == 0 { 29 } else { 28 };
    println!("Days in February, 2004: {num_days_in_feb}");

    // Loop loop
    let mut counter = 0;
    let iters = loop {
        if counter < 5 {
            println!("Counter is: {counter}");
            counter += 1;
        } else {
            break counter;
        }
    };
    println!("Iters: {iters}");

    // Loop labels:
    counter = 0;
    'outer_loop: loop {
        loop {
            counter += 1;
            if counter >= 50 {
                break 'outer_loop;
            }
            break;
        }
        counter += 10;
    }

    // While loops
    let nums = [1, 2, 3, 4];
    let mut idx = 0;
    let mut curr_elem = 0;
    while idx < nums.len() {
        curr_elem = nums[idx];
        println!("Nums: {curr_elem}");
        idx += 1;
    }

    // Fun with for loops
    for num in (1..3).rev() {
        println!("{num}!");
    }
    println!("Liftoff!");
    for num in nums {
        println!("Nums: {curr_elem}");
    }

    // Print the first 10 fibonacci numbers
    let mut fib_idx = 1;
    let mut curr = 0;
    while fib_idx <= 10 {
        curr = gen_fib(fib_idx);
        println!("Current fibonacci: {curr}");
        fib_idx += 1;
    }

    // Days of Christmas
    let mut idx = 4;
    let mut christmas_song = "".to_string();
    let song = loop {
        if idx == 0 {
            break christmas_song;
        } else {
            christmas_song.push_str(match idx {
                4 => "On the first day of Christmas my true love gave to me\n",
                3 => "Three French hens\n",
                2 => "Two turtle doves\n",
                1 => "And a partridge in a pear tree\n",
                _ => "",
            });
        }
        idx -= 1;
    };
    println!("Song: {song}");
}
