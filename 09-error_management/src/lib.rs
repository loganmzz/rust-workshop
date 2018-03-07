//! 09 Error management
//! -------------------
//!
//! Welcome to ninth step of this Rust workshop.
//!
//! This step focuses on error handling.
//!
//! ## May success or fail
//!
//! [Result](https://doc.rust-lang.org/stable/std/result/enum.Result.html) enum is used in Rust to indicate that a function may returns a successful value or an error.
//!
//! ```rust
//! # enum DivisionError { Zero, NotInteger }
//! fn i64_divide(dividend: i64, divisor: i64) -> Result<i64, DivisionError> {
//!     if divisor == 0 {
//!         Err(DivisionError::Zero)
//!     } else {
//!         let quotient = dividend / divisor;
//!         if dividend != quotient * divisor {
//!             Err(DivisionError::NotInteger)
//!         } else {
//!             Ok(quotient)
//!         }
//!     }
//! }
//! ```
//!
//! Interesting things is that `Result` has very nice functional API:
//!
//! ```rust
//! fn with(val: Result<i32, &'static str>) -> Result<String, &'static str> {
//!     val.map(|ok| ok.to_string())
//! }
//! with(Ok(2)); // Ok("2")
//! with(Err("error")); // Err("error")
//! ```
//!
//! ```rust
//! fn with(val: Result<i32, &'static str>) -> Result<i32, String> {
//!     val.map_err(|err| err.to_uppercase())
//! }
//! with(Ok(2)); // Ok(2)
//! with(Err("error")); // Err("ERROR")
//! ```
//!
//! ```rust
//! let val1: Result<i32, &'static str> = Ok(2).and(Ok(4)); // Ok(4)
//! let val2: Result<i32, &'static str> = Ok(2).and(Err("error")); // Err("error")
//! ```
//!
//! ```rust
//! fn err(msg: &'static str) -> Result<i32, &'static str> { Err(msg) }
//! let val1: Result<i32, &'static str> = err("error").and(Ok(4)); // Err("error")
//! let val2: Result<i32, &'static str> = err("fatal").and(Err("useless")); // Err("fatal")
//! ```
//!
//! ## Forwarding error
//!
//! When chaining call, checking and returning back error may be quite annoying:
//!
//! ```rust
//! # struct Foo;
//! # struct Bar;
//! # struct IoError;
//! # struct FooBar { foo: Foo, bar: Bar }
//! fn parse_foo(input: &str) -> Result<Foo, IoError> {
//!     /* ... */
//! # Ok(Foo)
//! }
//! fn parse_bar(input: &str) -> Result<Bar, IoError> {
//!     /* ... */
//! # Ok(Bar)
//! }
//!
//! fn read(input: &str) -> Result<FooBar, IoError> {
//!     let foo = match parse_foo(input) {
//!         Ok(foo) => foo,
//!         Err(io_error) => return Err(io_error)
//!     };
//!
//!     let bar = match parse_bar(input) {
//!         Ok(bar) => bar,
//!         Err(io_error) => return Err(io_error)
//!     };
//!
//!     Ok(FooBar { foo, bar })
//! }
//! ```
//!
//! `?` operator can be used to simplify such pattern:
//!
//! ```rust
//! # struct FooBar { foo: i32, bar: i32 }
//! # struct IoError;
//! # fn parse_foo(input: &str) -> Result<i32, IoError> { Ok(0) }
//! # fn parse_bar(input: &str) -> Result<i32, IoError> { Ok(0) }
//! #
//! fn read(input: &str) -> Result<FooBar, IoError> {
//!     let foo = parse_foo(input)?;
//!
//!     let bar = parse_bar(input)?;
//!
//!     Ok(FooBar { foo, bar })
//! }
//! ```
//!
//! Finally, Error can also be converted if `From` trait is implement for function error type:
//!
//! ```rust
//! # use std::io;
//! # mod foo { pub struct Error; }
//! # struct Foo;
//! # struct Bar;
//! enum MyError {
//!     IoError(io::Error),
//!     FooError(foo::Error),
//! }
//!
//! impl From<io::Error> for MyError {
//!     fn from(e: io::Error) -> MyError {
//!         MyError::IoError(e)
//!     }
//! }
//!
//! impl From<foo::Error> for MyError {
//!     fn from(e: foo::Error) -> MyError {
//!         MyError::FooError(e)
//!     }
//! }
//!
//! fn does_io() -> Result<Foo, io::Error> {
//! # Ok(Foo)
//! }
//! fn does_foo() -> Result<(), foo::Error> {
//! # Ok(())
//! }
//! fn does() -> Result<Bar, MyError> {
//!     let foo = does_io()?;
//!     does_foo()?;
//!
//!     Ok(Bar)
//! }
//! ```
//!
//! ## Optional values
//!
//! `null` value is called "the billion dollar mistake". So, Rust has no `null` but a missing value can still be expressed with `std::option::Option` enum.
//!
//! ```rust
//! # enum FooError { NotFound }
//! # struct Bar;
//! # struct Foo { bar: Bar }
//! # fn search_bar() -> Option<Bar> { Some(Bar) }
//! fn make_optional(with_value: bool) -> Option<String> {
//!     if with_value {
//!         Some(String::from("value"))
//!     } else {
//!         None
//!     }
//! }
//!
//! Some(String::from("foo")).unwrap_or(String::from("bar")); // "bar"
//! None.unwrap_or_else(|| String::from("default")); // "default"
//!
//! // Return error if value is missing
//! fn may_fail() -> Result<Foo, FooError> {
//!     let bar = search_bar().ok_or(FooError::NotFound)?;
//!     Ok(Foo { bar })
//! }
//! ```
//!
//! ## Panic
//!
//! Rust has capability similar to `Exception` in other languages through `panic!` macro. It currently stops running thread and if it's `main`, program is stopped.
//!
//! ```should_panic
//! fn stop_thread(code: u64) {
//!     panic!("Abort thread (error code: {})", code);
//! }
//!
//! let optional: Option<i32> = None;
//! optional.expect("No value");
//! ```
//!
//! ```should_panic
//! let must_success: Result<i32, &'static str> = Err("but it fails !");
//! must_success.expect("it should have worked");
//! // panic: "it should have worked: but it fails !"
//! ```
//!
//! ```should_panic
//! let must_fail: Result<&'static str, &'static str> = Ok("but it was a surprising success !");
//! must_fail.expect_err("it must not work");
//! // panic: "it must not work: but it was a surprising success !"
//! ```
//!
//! `panic` is generally advised when invariants are broken, state is invalid, caller is not expected to programmatically handle error (caller bug), ...

#[cfg(test)]
mod tests;
