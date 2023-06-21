//! # rustdoc-example
//!
//! A simple project demonstrating the use of rustdoc with the function
//! [`mult`].
#![warn(missing_docs)]
/// Returns the product of `a` and `b`.
pub fn mult(a: i32, b: i32) -> i32 {
    a * b
}
