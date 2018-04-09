#![doc(
html_playground_url = "https://play.rust-lang.org/",
)]

//! 01 Basic
//! --------
//!
//! Welcome to first step of this Rust workshop.
//!
//! This first step aims to teach you the very basis of Rust syntax (variables, primitives, functions, ...)
//!
//! So, before starting few things to know :
//!
//! ## Variables
//!
//! To create a variable, just use `let`:
//!
//! ```rust
//! let my_variable = 42;
//! ```
//!
//! You can also add a type hint with `: <type>` syntax but it is generally optional as compiler may infer it most time (from assignment but also usage):
//!
//! ```rust
//! let my_variable: i32;
//! ```
//!
//! You may assign value at variable declaration or later:
//!
//! ```rust
//! let not_initialized;
//! let initialized = 42;
//! not_initialized = initialized;
//! ```
//!
//! _Note: Rust uses [`snake_case`](https://en.wikipedia.org/wiki/Snake_case) convention for function and variable names. Compiler will warn you if you don't._
//!
//! ## Primitives
//!
//! ### Boolean
//!
//! Let's starts with boolean. They are represented by `bool` and can have two values: `true` or `false`.
//!
//! ### Numbers
//!
//! Let's continue with number. Integers may be signed (`i` prefix) or unsigned (`u` prefix) and their name defines their size (`8`, `16`, `32` or `64`). One exception is `size` prefix which depends on your system architecture. Decimal (`f` prefix) may only available with two size: simple precision (`32`) or double one (`64`).
//!
//! ```rust
//! let signed_32bits_integer: i32 = -400;
//! let unsigned_8bits_integer: u8 = 255;
//! let only_supported_on_64bits_architecture: isize = -9_000_000_000_000_000_000;
//! let decimal: f64 = -3.14;
//! ```
//!
//! Type hint can be replaced by suffixing literal (`-400i32`).
//!
//! _Note: there is no implicit number conversion. Explicitness is mandatory through `as` keyword._
//!
//! ```rust
//! let unsigned_8bits_integer = 255u8;
//! let unsigned_16bits_integer = 256u16;
//! let add: u16 = unsigned_8bits_integer as u16 + unsigned_16bits_integer;
//! ```
//!
//! ### Strings
//!
//! _Note: don't bother about notations and just accept it for the moment._
//!
//! Unlike other languages, strings are a bit more complex and refer to many types. A `"a string literal"` doesn't denote a `String` but a `&'static str` ! You can think at it as a memory reference to character array stores into executable binary. To create a `String` from a string literal just use the `String::from()` function.
//!
//! ```rust
//! let string_literal: &'static str = "a static message";
//! let string: String = String::from(string_literal);
//! ```
//!
//! String can also be built with `format!(pattern, param1, param2, ...)` macro using `{}` (`Display`) or `{:?}` (`Debug`) as placeholders. Pattern is checked at compile time !
//!
//! ```rust
//! let parameter: &'static str = "a parameter";
//! let message: String = format!("Display: {}\nDebug: {:?}", parameter, parameter);
//! ```
//!
//! In the same manner, `println!` and `eprintln!` are used to print string respectively on "stdout" and "stderr".
//!
//! Finally any type supporting `Display` has also a `to_string()` method.
//!
//! ```rust
//! let numToString: String = 42u64.to_string();
//! let strToString: String = "some characters".to_string();
//! ```
//!
//! ### Unit
//!
//! Let's terminate tour of primitive types and literals with unit type. It equals `void` type/keyword into some other languages. It means "nothing". It is the default function return type and so can be omitted. It is both represented as type and value by `()`.
//!
//! ```rust
//! let unit: () = ();
//! ```
//!
//! ## Functions
//!
//! Functions are declared with `fn function_name(param1: type1, param2: type2) -> ReturnType {}`. **ALL** types are mandatory as there is no type inference in function signature in order to ensure consistency ; except for return type which can be omitted if _nothing_ (`()`) is returned.
//!
//! Curly braces are mandatory and if function ends with an **expresssion** (not a statement which ends with a trailing `;`) it is used as return value. However you can use a `return` **statement** for early returns.
//!
//! _Note: in Rust all blocks are considered as an expression. And then can _return_ values by ending block with an expression.
//!
//! ```rust
//! fn add(left: i64, right: i64) -> i64 {
//!     left + right // No trailing ';'
//! }
//!
//! fn mul(left: i64, right: i64) -> i64 {
//!     return left * right;
//! }
//! ```
//!
//! ## Standard library
//!
//! Finally, API documentation is available at [https://doc.rust-lang.org/std/](https://doc.rust-lang.org/std/). Rust is shipped with other crates (Rust packaging) but only `std` matters as other are included by it.
//!
//! First thing to notice is the search bar ; it is a good entry when looking for something or even to quickly jump into it. In order to use it efficiently, an attention must be given to colors :
//!
//! * blue for module ([exemple](https://doc.rust-lang.org/stable/std/usize/))
//! * orange for struct ([exemple](https://doc.rust-lang.org/stable/std/string/struct.String.html))
//! * darker green for enum ([exemple](https://doc.rust-lang.org/stable/std/result/enum.Result.html))
//! * purple for trait ([exemple](https://doc.rust-lang.org/stable/std/marker/trait.Copy.html))
//! * brown for function and method ([exemple](https://doc.rust-lang.org/stable/std/result/enum.Result.html#method.is_ok))
//! * lighter green for macro ([exemple](https://doc.rust-lang.org/stable/std/macro.println.html))
//! * grayed blue for constant ([exemple](https://doc.rust-lang.org/stable/std/usize/constant.MAX.html))
//! * cyan for primitive ([exemple](https://doc.rust-lang.org/stable/std/primitive.usize.html))
//!
//! _Note: looking at link url will also give information._
//!
//! All crates automatically import [`std::prelude`](https://doc.rust-lang.org/std/prelude/) module (i.e. namespace).

#[cfg(test)]
mod tests;
