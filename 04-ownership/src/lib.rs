//! 04 Ownership
//! ------------
//!
//! Welcome to fourth step of this Rust workshop.
//!
//! This step focuses on how Rust handles (de)allocation and what makes Rust very special: ownership.
//!
//! ## Move value
//!
//! All methods that have be defined until now _consume_ `self`. It means variable can't be reused after call.
//!
//! ```rust
//! struct MyType { /* ... */ }
//! impl MyType {
//!     fn method(self) { /* ... */ }
//! }
//!
//! let variable = MyType { /* ... */ };
//! variable.method(); // Ok
//!
//! // Compilation error
//! // variable.method();
//! ```
//!
//! By default, Rust _moves_ value. It means when a value is assigned to a variable (or a parameter, a field, ...), the variable owns the value until next move or end of block. When nothing owns a value, Rust just deallocates it. And all these things are checked at compile time with no runtime cost !
//!
//!
//! One solution could be to return `self` like in fluent API but a better one is following.
//!
//! ```rust
//! # struct MyType;
//! impl MyType {
//!     fn method(self) -> Self {
//!         self
//!     }
//! }
//!
//! let variable = MyType { /* ... */ };
//! let variable = variable.method();
//! variable.method();
//! ```
//!
//! ## Copy value
//!
//! If "by default" has been specified, it means Rust can do something different. The other case is _copying_ value. For example, primitive types (`i32`, `f64`, `bool`, `()` ...) can be reused after being passed as parameter.
//!
//! ```rust
//! fn add(left: u64, right: u64) -> u64 { left + right }
//!
//! let one = 1;
//! let two = add(one, one);
//! let three = add(two, one);
//! ```
//!
//! For _copy types_, each time value is assigned instead of moving ownership, Rust creates a new instance by making a memory copy. Custom types can be made _copy type_ by deriving `Copy`.
//!
//! ```rust
//! #[derive(Clone, Copy)]
//! struct MyType {}
//!
//! impl MyType {
//!     fn consume(self) {}
//! }
//!
//! let variable = MyType {};
//! variable.consume();
//! variable.consume();
//! ```
//!
//! ## Borrowing
//!
//! Rust also offers a solution to share a value without copying through borrowing. To borrow a value, owner must be prefix with `&` and requesting borrowing work the same way on type.
//!
//! ```rust
//! # struct MyType {}
//! fn borrowing_value(parameter: &MyType) {}
//!
//! let variable = MyType {};
//!
//! let first_borrowed = &variable; // Scoped to block
//! let second_one = &variable;
//!
//! borrowing_value(&variable); // Scoped to call
//!
//! borrowing_value(first_borrowed);
//! borrowing_value(second_one);
//! ```
//!
//! Borrow last until variable goes out of scope and is immutable even if original variable is mutable.
//!
//! _Note: `&str` can be created from by borrowing from a `String`._
//!
//! ## Mutability
//!
//! While any number of immutable borrow may exist at a time, one and only one can be mutable. It is like requesting a write/exclusive lock to the value. Mutable borrow can only be get from a mutable variable.
//!
//! ```rust
//! # struct MyType { field: i32 }
//! fn mutable_borrow(parameter: &mut MyType) {
//!     parameter.field = 0;
//! }
//!
//! let mut variable = MyType { field: 42 };
//!
//! mutable_borrow(&mut variable); // Scoped to call
//!
//! let borrowed = &mut variable; // Scoped to block
//! mutable_borrow(borrowed);
//!
//! // Compilation error
//! // let forbidden = &mut variable;
//! ```
//!
//! ## Dereferencing
//!
//! Sometimes having concrete type instead of a reference (`&`) is required but don't need ownership. Then uses dereference operator (`*`).
//!
//! ```rust
//! let owned = String::from("");
//!
//! {
//!     let reference = &owned;
//!     let compare = String::new();
//!     println!("is equal: {}", *reference == compare);
//! }
//!
//! let moved = owned;
//! ```
//!
//! Other nice thing is ability to change target through an `&mut`.
//!
//! ```rust
//! struct Foo { bar: String, }
//!
//! impl Foo {
//!     // Getter
//!     fn bar(&self) -> &String {
//!         &self.bar
//!     }
//!
//!     // Mutator
//!     fn bar_mut(&mut self) -> &mut String {
//!         &mut self.bar
//!     }
//! }
//!
//! let mut foo = Foo { bar: String::from("toto"), };
//! let foo_bar_mut = foo.bar_mut();
//! *foo_bar_mut = String::from("lala");
//! ```

#[cfg(test)]
mod tests;
