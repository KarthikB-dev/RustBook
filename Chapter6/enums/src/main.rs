enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// Having a single message type enables genericity in functions, 
// i.e. just one could be used to used to handle *all* message updates
enum Message {
    Quit,
    Move {x : i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("Message called!");
    }
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    let five = Some(5);
    let absent_number : Option<i32> = None;
    dbg!(absent_number);
    // Optional integers don't support addition
    // let sum = five + absent_number;
}
