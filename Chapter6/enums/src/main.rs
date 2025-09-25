#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    California,
    NewJersey,
    Texas,
    Washington,
}

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn plus_one(x : Option<i32>) -> Option<i32> {
   match x {
       None => None,
       Some(i) => Some(i+1),
   }
}

fn value_in_cents(coin : &Coin) {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from state: {state:?}!");
            25
        },
    };
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
