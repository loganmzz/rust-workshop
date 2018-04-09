//! 05 Lifetime
//! -----------
//!
//! Welcome to fifth step of this Rust workshop.
//!
//! This step focuses on how Rust follows borrowed references.
//!
//! ## Syntax
//!
//! Lifetimes, as name suggest, is concept used by Rust to track life duration (scope) of borrows, and ensure borrowing rules (seen in previous chapter) are not violated.
//!
//! Lifetime shares their syntax with generics (see later) but prefixing variable by a quote (`'`). And can be applied to any declaration (function, struct, ...).
//!
//! ```rust
//! #[derive(Debug)]
//! struct Foo<'a> { bar: &'a str, } // Foo can't survive to bar
//!
//! fn display<'b>(foo: &Foo<'b>) {
//!     println!("{:?}", foo);
//! }
//!
//! let message = "foo";
//! let foo = Foo { bar: message };
//! display(&foo);
//! ```
//!
//! _Note: inference mechanism still applies. So it is rarely necessary to specify lifetime at usage._
//!
//! ## Elision
//!
//! Each time there is a reference, there is a lifetime. However, when obvious Rust compiler adds it automatically. Basically, if all parameters have a unique lifetime or there's a self reference parameter, then Rust applies this lifetime to all output references. Otherwise, compilation fails.
//!
//! ```rust,ignore
//! fn debug(s: &str);
//! fn debug<'a>(s: &'a str);
//!
//! fn split(s: &str) -> (&str, &str);
//! fn split<'a>(s: &'a str) -> (&'a str, &'a str);
//!
//! fn create<'a>(s: &'a str) -> Foo;
//! fn create<'a>(s: &'a str) -> Foo<'a>;
//!
//! fn merge(&self, &str) -> &str;
//! fn merge<'a, 'b>(&'a self, &'b str) -> &'a str;
//! ```

#[cfg(test)]
mod tests;
