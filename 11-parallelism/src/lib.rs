//! 11 Parallelism
//! --------------
//!
//! Welcome to eleventh step of this Rust workshop.
//!
//! This step focuses on basic multithreading API in Rust
//!
//! ## Thread
//!
//! To create a new thread, we call the `thread::spawn` function and pass it a piece of code.
//!
//! NOTE:
//! Calling join on the handle blocks the thread currently running
//! until the thread represented by the handle terminates.
//!
//! ```rust
//! use std::thread;
//!
//! fn simple_addition() -> i32 {
//!     1 + 1
//! }
//!
//! let other_thread = thread::spawn(simple_addition);
//! //                               ^^^^^^^^^^^^^^^  Here is a function pointer
//!
//! let res = other_thread.join().unwrap();
//! println!("res: {}", res);
//! ```
//!
//! NOTE: We can improve this code by using a **closure** instead of using a function pointer
//!
//! ```rust
//! use std::thread;
//!
//! let other_thread = thread::spawn(|| 1 + 1);
//! //                               ^^^^^^^^  Here is the closure
//!
//! let res = other_thread.join().unwrap();
//! println!("res: {}", res);
//! ```
//!
//! Get more info: [rust-doc/thread](https://doc.rust-lang.org/std/thread/struct.Thread.html).
//! 
//! 
//! ## Message passing to transfer data between threads
//! 
//! A approach to ensuring safe concurrency is message passing.
//! Rust’s standard library provides an implementation of channel.
//! The channel has two components: a transmitter and a receiver.
//! 
//! NOTE: A channel is closed if either the transmitter or receiver is dropped.
//! 
//! The `send` function takes **ownership** of its parameter, and when the value is moved, 
//! the receiver takes ownership of it. This stops us from accidentally editing
//! the value again after sending it; the ownership system checks that everything is okay at compile time.
//! 
//! ```rust
//! use std::thread;
//! use std::sync::mpsc::channel;
//! 
//! let (sender, receiver) = channel();
//! thread::spawn(move || {
//!     let mut data = 10;
//!     sender.send(data).unwrap();
//!     // You can't no longer modify "data" after calling `send`
//!     // because the ownership of data was transferred to "sender"
//! });
//! 
//! assert_eq!(receiver.recv().unwrap(), 10);
//! ```
//!
//! NOTE: **move** keyword
//!
//! We can force our closure to take ownership of its environment with the **move** keyword.
//!
//! Get more info: [rust-doc/channel](https://doc.rust-lang.org/std/sync/mpsc/index.html)

#[cfg(test)]
mod tests;
