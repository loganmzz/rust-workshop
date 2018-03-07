//! 03 Types
//! ---------
//!
//! Welcome to third step of this Rust workshop.
//!
//! This step focuses on defining new types in Rust.
//!
//! ## Structure
//!
//! The basic structural type in Rust is called a `struct`. Similar to C-struct, it ony contains data and has three forms.
//!
//! The first one is called _unit struct_ and has no content (it generally serves as marker):
//!
//! ```rust
//! struct Empty;
//! ```
//!
//! The second one is called _tuple struct_. It is a comma-separated list of types, each field is accessible through its position.
//!
//! ```rust
//! struct Tuple(i32, String);
//!
//! let tuple = Tuple(-42, String::from("THE response"));
//!
//! let key: i32 = tuple.0;
//! ```
//!
//! And the final one, just called _struct_ with named fields:
//!
//! ```rust
//! struct CLike {
//!     key: i32,
//!     name: String,
//! }
//!
//! let clike = CLike { key: -42, name: String::from("THE response") };
//!
//! let key: i32 = clike.key;
//! ```
//!
//! In all cases, `struct` isn't a class-like construction and doesn't support inheritance !
//!
//! ## Enumeration
//!
//! Rust hasn't inheritance but in somes cases, finite set of subtypes is required. In such cases, Rust has `enum` where each entry is a `struct`-like. Entries can't be used directly as type and are only used for (de)construction.
//!
//! ```rust
//! enum Person {
//!     Anonymous,
//!     Natural { last_name: String, first_name: String },
//!     Company { name: String, owners: Vec<Person> },
//!     Association(String),
//! }
//!
//! let no_one: Person = Person::Anonymous;
//! let someone: Person = Person::Natural {
//!     last_name: String::from("DOE"),
//!     first_name: String::from("John"),
//! };
//! let famous_company: Person = Person::Company {
//!     name: String::from("$$$"),
//!     owners: vec![no_one, someone],
//! };
//! let another_organisation: Person = Person::Association(String::from("❤ organisation ❤"));
//! ```
//!
//! ## Implementation
//!
//! While Rust isn't really an object-oriented language, method-like call is still possible through `impl` block. It applies on a type, then functions are declared using `self` with no type as first parameter.
//!
//! ```rust
//! # struct CLike { key: i32, name: String }
//! impl CLike {
//!     fn get_key(self) -> i32 {
//!         self.key
//!     }
//! }
//!
//! let clike = CLike { key: 242860, name: String::from("...") };
//! let key = clike.get_key();
//! ```
//!
//! Static-like function are also posible and are called `associated function`. They are commonly used as constructor.
//!
//! ```rust
//! # struct CLike { key: i32, name: String }
//! impl CLike {
//!     fn new(key: i32, name: &str) -> CLike {
//!         CLike { key, name: String::from(name) }
//!     }
//! }
//!
//! let clike = CLike::new(242860, "...");
//! ```
//!
//! _Note: Here a new notation has been introduced. When initializing a "named" struct and a variable has same name as a field, `Type { field_name: field_name }` can be replaced by `Type { field_name }`._
//!
//! ## Derivation
//!
//! `Debug` support can be automatically added to types through _derivable trait_ feature.
//!
//! ```rust
//! #[derive(Debug)]
//! struct CLike { /* ... */ }
//!
//! println!("Debug: {:?}", CLike { /* ... */ });
//! ```
//!
//! In the same manner,
//!
//! * adds equality comparison with both `Eq` and `PartialEq`
//! * adds ordering comparison with both `Ord` and `PartialOrd`
//! * adds `default()` method with `Default`

#[cfg(test)]
mod tests;
