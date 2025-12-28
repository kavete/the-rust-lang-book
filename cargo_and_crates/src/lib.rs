// /// Adds one to the number given
// ///
// /// # Examples
// /// ```
// /// let arg = 5;
// /// let answer = my_crate::add_one(arg);
// ///
// /// assert_eq!(6, answer);
// ///```
//
// pub fn add_one(x: i32) -> i32 {
//     x + 1
// }

///! # Art
///!
///!A library for modelling artistic concepts
///
pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;
pub mod kinds {
    /// Primary colors according to the RYB mode
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }
    /// Secondary colors according to RYB model
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;
    /// Combines two primary colors in equal amounts to create a secondary color
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        SecondaryColor::Green
    }
}
