//! # Art
//!
//! A library for modeling artistic concepts

pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::PrimaryColor;
    use crate::SecondaryColor;
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) {
        // nop
    }
}
