// //!Adds comments to item (in this case sec/lib.rs) containing the comments eg a crate, module etc
//! # Art
//!
//! A library for modeling artistic concepts.

// === Exporting a Convenient Public API with pub use
pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
    /// The primary colors according to the RYB color model.
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        // --snip--
        SecondaryColor::Green
    }
}



// cargo doc to generate documentation. cargo doc --open to open as well
// Use 3 /// for docs. They support mark down
// Most common sections are: Examples, Panics, Errors, Safety
// Cargo test runs the code in the comments
/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = cargo_cratesio::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}