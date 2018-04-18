#![doc(
html_playground_url = "https://play.rust-lang.org/",
)]

//! 06 Trait
//! --------
//!
//! Welcome to sixth step of this Rust workshop.
//!
//! This step focuses on defining API abstraction.
//!
//! ## Concept & Syntax
//!
//! While Rust isn't an object-oriented language, it still has a number of similar features (e.g. methods). Interface-like is provided through concept of `trait` which describes an API contract. When using `trait`, it's said _implementing a trait for a target type_.
//!
//! ```rust
//! struct MyType {
//!     foo: u8,
//!     bar: String,
//! }
//!
//! trait MyTrait {
//!     fn value(&self) -> &u8;
//!
//!     fn with_default_impl(&self) -> &u8 {
//!         self.value()
//!     }
//!
//!     fn consume(self) -> Self; // Refer to target type
//! }
//!
//! impl MyTrait for MyType {
//!     fn value(&self) -> &u8 {
//!         &self.foo
//!     }
//!
//!     fn consume(self) -> Self {
//!         println!("consume: {:?}", self.bar);
//!         self
//!     }
//! }
//! ```
//!
//! ## Trait bounds
//!
//! Sometimes defining a trait requires target must implement other trait. This is called trait bounds. There's no inheritance as implementing trait apart is required.
//!
//! ```rust
//! use std::fmt::Debug;
//!
//! trait Identifiable {
//!     fn id(&self) -> String;
//! }
//!
//! trait Display : Debug + Identifiable {
//!     fn display(&self) {
//!         println!("[{:?}]  {:?}", self.id(), self);
//!     }
//! }
//!
//! #[derive(Debug)]
//! struct Displayer(String);
//! impl Identifiable for Displayer {
//!    fn id(&self) -> String {
//!      self.0.to_uppercase()
//!    }
//! }
//! impl Display for Displayer {}
//!
//! Displayer(String::from("foo_bar")).display();
//! ```
//!
//! ## Associated type
//!
//! A common construct in Rust is to associate one (or many) type(s) to an trait, so it can be specified when implementing for each target type.
//!
//! ```rust
//! trait WithAssociatedType {
//!     type ValueType;
//!     fn value(&self) -> &Self::ValueType;
//! }
//!
//! struct NumberContainer { value: i64, }
//! impl WithAssociatedType for NumberContainer {
//!     type ValueType = i64;
//!     fn value(&self) -> &i64 {
//!         &self.value
//!     }
//! }
//!
//! struct StringContainer { data: String, }
//! impl WithAssociatedType for StringContainer {
//!     type ValueType = String;
//!     fn value(&self) -> &Self::ValueType {
//!         &self.data
//!     }
//! }
//! ```
//!
//! ## Operator overloading
//!
//! It's common to use interface/trait concept to represents operators and Rust follows the same rule. For example, to support `+` operator:
//!
//! ```rust
//! # struct NumberContainer { value: i32 }
//! use std::ops::Add;
//!
//! impl Add for NumberContainer {
//!     type Output = NumberContainer;
//!     fn add(self, other: NumberContainer) -> NumberContainer {
//!         NumberContainer { value: self.value + other.value }
//!     }
//! }
//! ```
//!
//! All operator traits are stored under `std::ops` module.
//!
//! ## Derivable trait
//!
//! Previously, `#[derive(...)]` syntax has been introduced to add capability to new types. Items placed between parentheses are just trait that can be implemented manually. Such trait are called _derivable trait_.

#[cfg(test)]
mod tests;
