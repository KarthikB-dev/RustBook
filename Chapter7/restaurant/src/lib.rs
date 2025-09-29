use std::fmt::Result;
use std::fmt;
use std::io::{self, Write};
use std::collections::*;

mod front_of_house;

fn function1() -> fmt::Result {
    return Ok(());
}

fn function2() -> io::Result<()> {
    return Ok(());
}

use crate::front_of_house::hosting::add_to_waitlist;

mod customer {
    pub fn eat_at_restaurant() {
        super::add_to_waitlist();
    }
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
