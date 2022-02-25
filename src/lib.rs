//! Fast designing menus for your Rust CLI programs.
//!
//! This crates provides many procedural macros to easily build menus.
//! It uses the [`ezmenulib`](https://docs.rs/ezmenulib) library crate in its expansion.
//!
//! # Example
//!
//! Here is an example of how to use the `derive(Menu)` macro
//! (it takes the same example as the documentation of the `ezmenu` library):
//! ```
//! use ezmenu::Menu;
//!
//! #[derive(Menu)]
//! #[menu(title = "Hello there!")]
//! struct MyMenu {
//!     #[menu(msg = "Give your name")]
//!     name: String,
//!     #[menu(msg = "Give a number")]
//!     number: i32,
//! }
//!
//! fn main() {
//!     let MyMenu { name, number } = MyMenu::from_menu();
//!     println!("values provided: name={}, number={}", name, number);
//! }
//! ```
//!
//! This sample code prints the standard menu like above:
//!
//! ```text
//! Hello there!
//! * Give your name: Ahmad
//! * Give a number: 1000
//! values provided: name=Ahmad, number=1000
//! ```
//!
//! ## Format it as you wish
//!
//! You can apply several formatting rules on a menu or on a field specifically.
//! You can edit:
//! * the chip: `* ` by default.
//! * the prefix: `: ` by default.
//! * insert a new line before prefix and user input: `false` by default.
//! * display default values or not: `true` by default.
//!
//! ### Example
//!
//! For a custom format on a field and a main formatting rule on a menu, you can build this with:
//! ```rust
//! #[derive(Menu)]
//! #[menu(chip = "==> ")]
//! struct License {
//!     author: String,
//!     #[menu(chip = "--> ")]
//!     project_name: String,
//!     date: u16,
//! }
//! ```
//!
//! The custom `==> ` chip will be applied on every field except those with custom formatting rules,
//! In this case, it will format the text like above:
//!
//! ```text
//! ==> Authors: ...
//! --> Project name: ...
//! ==> Date: ...
//! ```
//!
//! ## Skip fields with default values
//!
//! You can provide a default input value to a field with the `default` attribute:
//! ```rust
//! #[derive(Menu)]
//! struct License {
//!     #[menu(default = 2022)]
//!     date: u16,
//!     // ...
//! }
//! ```
//!
//! If the user provided an incorrect input, the program will not re-ask a value to the user,
//! but will directly return the default value instead.
//!
//! By default, the default value is visible. If you want to hide it, you can do so
//! with formatting rules:
//! ```rust
//! #[derive(Menu)]
//! #[menu(display_default = false)] // <-- applied on all the fields
//! struct License {
//!     #[menu(default = 2022, display_default = false)] // <-- applied on the field specifically
//!     date: u16,
//!     // ...
//! }
//! ```
//!
//! ## Use custom value types
//!
//! If the user has to provide a value which corresponds to your specific type,
//! you can add the `parsed` feature in your `Cargo.toml`:
//!
//! ```toml
//! [dependencies.ezmenu]
//! version = "0.2"
//! features = ["parsed"]
//! ```
//! Therefore, you only need to add the `ezmenu::parsed` attribute on that type.
//!
//! ### Example
//!
//! ```
//! #[ezmenu::parsed]
//! enum Type {
//!     MIT,
//!     BSD,
//!     GPL,
//! }
//!
//! #[derive(Menu)]
//! struct License {
//!     author: String,
//!     #[menu(default = "mit")] // <-- you can still use the default attribute
//!     ty: Type,
//! }
//! ```
//!
//! ## Provided custom value types
//!
//! The EZMenu library already provides
//! [custom value types](https://docs.rs/ezmenulib/latest/ezmenulib/#provided-custom-value-types)
//! to handle user input.
//!
//! ### Example
//!
//! ```
//! use ezmenulib::{MenuBool, MenuVec};
//!
//! #[derive(Menu)]
//! struct License {
//!     authors: MenuVec<String>,
//!     #[menu(msg = "Are you sure...????")]
//!     is_sure: MenuBool,
//!     // ...
//! }
//! ```
//!
//! ## Features
//!
//! This crate includes many features to allow you control your project building.
//!
//! For instance, the `"derive"` feature unlocks the `derive(Menu)` macro and is set by default,
//! and the `"parsed"` feature unlocks the `ezmenu::parsed` attribute macro.
//!
//! For example, if you only need to use the `parsed` feature to implement `FromStr` trait
//! on a type, you can do so by adding this to your `Cargo.toml` file:
//! ```toml
//! [dependencies.ezmenu]
//! default-features = false
//! features = ["parsed"]
//! version = "0.2"
//! ```
//!
//! Then you can add `#[ezmenu::parsed]` on that type and you'll be done :D
#![deny(missing_docs)]

/// The `derive(Menu)` procedural macro.
///
/// It allows to easily build CLI menus, by using the [`ezmenulib`](https://docs.rs/ezmenulib/) crate.
///
/// ## Example
///
/// ```
/// use ezmenu::lib::MenuVec;
/// use ezmenu::MenuVec;
///
/// #[derive(Menu)]
/// #[menu(
///     title = "mkLicense program",
///     chip = "--> Give the "
///     prefix = ">> ",
///     new_line = true,
/// )]
/// struct License {
///     #[menu(msg = "authors name separated by spaces")]
///     authors: MenuVec<String>,
///
///     #[menu(msg = "project name")]
///     proj_name: String,
///
///     #[menu(msg = "project year", default = 2022)]
///     year: u16,
/// }
/// ```
#[cfg(feature = "derive")]
pub use ezmenu_macros::Menu;

/// The attribute simply implements the `FromStr` trait on a type.
///
/// ## Example
///
/// ```
/// #[ezmenu::parsed]
/// enum Type {
///     MIT,
///     GPL,
///     BSD,
/// }
///
/// #[derive(Menu)]
/// struct License {
///     #[menu(msg("Give the type of the license"))]
///     ty: Type,
/// }
/// ```
#[cfg(feature = "parsed")]
pub use ezmenu_macros::parsed;

/// The re-exportation of the [`ezmenulib`](https://docs.rs/ezmenulib/)
/// crate for the macro expansion.
pub mod lib {
    pub use ezmenulib::*;
}
