use add_one::add_one;

pub fn add_two(x: i32) -> i32 {
	add_one(x) + 1
}

fn main() {
        let num = 10;
        println!("Hello world, {} plus two is {}", num, add_two(num));
}


