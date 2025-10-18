use std::net::IpAddr;

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess must be greater than or equal to 1, got {value}");
        } else if value > 100 {
            panic!("Guess must be less than or equal to 100, got {value}");
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Guess must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
}

fn main() {
    let home: IpAddr = "127.0.0.1"
        .parse()
        .expect("Harcoded IP address should be valid");
}
