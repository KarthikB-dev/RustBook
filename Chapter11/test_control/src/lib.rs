use std::{thread, time::Duration};

fn prints_and_returns_ten(value: i32) -> i32 {
    println!("I got the value {value}");
    value 
}

pub fn add_two(a: u64) -> u64 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_ten(10);
        assert_eq!(value, 10);
    } 

    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_ten(4);
        assert_eq!(value, 10);
    } 

    #[test]
    fn add_two_and_two() {
        let result = add_two(2);
        assert_eq!(result, 4);
    }

    #[test]
    fn add_four_and_two() {
        let result = add_two(4);
        assert_eq!(result, 6);
    }

    #[test]
    fn add_three_and_two() {
        let result = add_two(3);
        assert_eq!(result, 5);
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        thread::sleep(Duration::from_millis(4000000000));
    }
}


