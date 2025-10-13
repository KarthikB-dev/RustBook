#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn add_two(num: u64) -> u64 {
    num + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 3);
        assert_eq!(result, 5);
    }

    // #[test]
    // fn fail_test() {
    //     panic!("It fail!");
    // }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 15,
            height: 10,
        };
        let smaller = Rectangle {
            width: 10,
            height: 5,
        };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cant_hold_larger() {
        let larger = Rectangle {
            width: 15,
            height: 10,
        };
        let smaller = Rectangle {
            width: 10,
            height: 5,
        };
        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_add_two() {
        let sum = add_two(2);
        assert_eq!(sum, 4);
    }
}
