#![doc(
    html_playground_url = "https://play.rust-lang.org/",
)]

//! 10 Closure (optional)
//! ---------------------
//!
//! Welcome to tenth step of this Rust workshop.
//!
//! This step focuses on closure.
//!
//! ## Syntax
//!
//! Rust’s closures are anonymous functions you can save in a variable or pass as arguments to other functions.
//! Unlike functions, closures can capture values from the scope in which they’re called.
//! They are present in many languages like javascript, Go, C++, ...
//!
//! In this short example we create a closure that take one (i32) parameter and add one to it.
//!
//! ```rust
//! let plus_one = |x| x + 1;
//! 
//! assert_eq!(2, plus_one(1));
//! ```
//! 
//! NOTE: Using closures instead of functions tends to be more costly because closures
//! capture some of the environment, and that has some overhead.
//! However, Rust has a smart compiler that can take advantage from the **borrow checker** to
//! produce a **zero cost** abstraction with closure that has enclosed variables.
//!
//! ## Ownership with closure
//!
//! As mentioned above, closures capture their current scope environment.
//! So closures capture the ownership of your parameters. To avoid memory error, you can no longer
//! use them in the parent scope.
//!
//! NOTE: You can try to run this code in _rust playground_ with the **run** button below
//!
//! ```compile_fail
//! let mut num = 4;
//!
//! let mut add_num = || num += 10;
//! //                ^^^^^^^^^^^^  The closure here capture "num" from the parent scope
//!
//! // The ownership is not longer held by your current scope.
//! num = 20;
//! ```
//!
//! So the compiler throws the error:
//!
//! ```bash
//! error[E0506]: cannot assign to `num` because it is borrowed
//!  --> src/main.rs:5:5
//!   |
//! 4 |     let mut add_num = || num += 10;
//!   |                       -- borrow of `num` occurs here
//! 5 |     num = 20;
//!   |     ^^^^^^^^ assignment to borrowed `num` occurs here
//! ```
//!
//! So the compiler allows us to avoid errors on memory management.
//! Imagine in another language, I will **free/liberate** the memory relative to "num" in the current scope.
//! I'll get a `segmentation fault` when I'll call closure later.
//!
//! ## Take a closure as arguments
//!
//! Closure can be passed as arguments to other functions or closures.
//!
//! ```rust
//! fn call_with_one<F>(your_closure: F) -> i32
//!     where F: Fn(i32) -> i32 {
//!     your_closure(1)
//! }
//!
//! let answer = call_with_one(|x| x + 2);
//! ```
//!
//! Get more info about closure in the [rust-book/closure](https://doc.rust-lang.org/book/second-edition/ch13-01-closures.html) section.

#[cfg(test)]
mod tests;
