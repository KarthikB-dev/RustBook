//! # Ch14_karthik
//!
//! `ch14_karthik` is the (rather poor) name of our crate!
//! it's used for basic arithmetic. It might help
//! your kids learn 1+1.

/// Adds one to the number given
///
/// # Examples
///
/// ```
/// let arg=5;
/// let answer = ch14::add_one(arg);
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
