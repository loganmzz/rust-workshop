#![doc(
html_playground_url = "https://play.rust-lang.org/",
)]

//! 13 Memory
//! --------------
//!
//! Welcome to thirteenth step of this Rust workshop.
//!
//! This step focuses on defining memory guarantees.
//!
//!
//! ## Heap allocation
//!
//! Memory allocation is organized in two different ways: **heap** & **stack**.
//!
//! **Stack** is a per-thread allocated space. Each function generate a new stack frame then, when
//! completed, frame is thrown away with all allocated memory. That is fast and efficient way to
//! free memory. That's why it's default Rust behaviour.
//!
//! On the other side, **heap** has no lifetime bounds (except for process itself) and has no
//! particular organisation. These characteristics makes allocated memory to grow, to be shared
//! between threads and to survive stack frames.
//!
//! Primary way to declare a heap allocated variable is to use a `Box`:
//!
//! ```rust
//! let on_heap = Box::new(1);
//! ```
//!
//! `Box` acts exactly as a stack allocated data and still preserve ownership, borrowing and
//! lifetime: can be read-only shared, write-exclusive shared and freed when no more in use:
//!
//! ```rust
//! struct MyType { /* ... */ }
//! impl MyType {
//!     fn own(self) { /* ... */ }
//!     fn borrow(&self) { /* ... */ }
//!     fn mutate(&mut self) { /* ... */ }
//! }
//!
//! fn own() {
//!     let variable = Box::new(MyType { /* ... */ });
//!     variable.own(); // Ok
//!     // Compilation error
//!     // variable.method();
//! }
//!
//! fn borrow() {
//!     let variable = Box::new(MyType { /* ... */ });
//!     variable.borrow();
//!     variable.borrow();
//! }
//!
//! fn mutate() {
//!     let mut variable = Box::new(MyType { /* ... */ });
//!     let borrowed = &mut variable;
//!     borrowed.mutate();
//!     // Compilation error
//!     //variable.mutate();
//! }
//! ```
//!
//! `Box` are generally used to declare recursive types:
//!
//! ```rust
//! enum List<T> {
//!     Cons(T, Box<List<T>>),
//!     Nil,
//! }
//! ```
//!
//! ## Reference counter
//!
//! Sometimes you need to share read-only data between many "owner" but without clear one. In this
//! case `std::rc::Rc` helps you to share pieces of data in read-only. It's closed to a borrow, but
//! without pre-defined lifetime.
//!
//! ```rust
//! # struct Person { name: String }
//! # impl Person {
//! #     fn new(name: &str) -> Self {
//! #         println!("Hello {}", name);
//! #         Person { name: String::from(name) }
//! #     }
//! # }
//! # impl Drop for Person {
//! #     fn drop(&mut self) {
//! #         println!("Goodbye {}", self.name);
//! #     }
//! # }
//! #
//! use std::rc::Rc;
//! let mut list = {
//!     let rc = Rc::new(Person::new("Bob"));
//!     vec![rc.clone(), rc.clone(), rc.clone(), rc.clone()]
//! };
//!
//! list.remove(0);
//! list.remove(0);
//! list.remove(0);
//! list.remove(0); // Deallocated after this line
//! #
//! # println!("** END **");
//! ```
//!
//! It may happen two elements into a struct are (directly or not) counting reference mutually ; thus, memory can't be freed as counter can never rich zero due to circular references. For this purpose, `std::rc::Weak` must be used.
//!
//! ```rust
//! use std::rc::{Rc, Weak};
//! let value = Rc::new(1);
//!
//! let weak = Rc::downgrade(&value);
//! println!("weak: {:?}", weak.upgrade()); // Some(1)
//!
//! drop(value);
//! println!("weak: {:?}", weak.upgrade()); // None
//! ```
//!
//! # Interior mutability
//!
//! By default, Rust applies _inherited mutability_, meaning mutability is declared at usage and propagates to all pieces of the data.
//! Some constructs in Rust offers _interior mutability_, meaning data can mutate while not explicitly declared as `mut`:
//!
//! ```rust
//! use std::cell::Cell;
//! #[derive(Debug)]
//! struct Owner {
//!     owned: Cell<i32>,
//! }
//! impl Owner {
//!     fn new(value: i32) -> Self {
//!         Owner { owned: Cell::new(value) }
//!     }
//!     fn set(&self, newvalue: i32) {
//!         self.owned.set(newvalue)
//!     }
//! }
//!
//! let data = Owner::new(0);
//! data.set(1);
//! ```
//!
//! # Runtime-checked borrowing
//!
//! It may happens that borrowing rules can't be enforced at compile time. Then use `std::cell::RefCell` that will `panic!` when violating invariants. Following examples compiles but panic at runtime.
//!
//! ```should_panic
//! use std::cell::RefCell;
//! let cell = RefCell::new(0);
//!
//! let read0 = cell.borrow(); // Ok
//! let read1 = cell.borrow(); // Ok
//! cell.borrow_mut(); // Panic
//! ```
//!
//! ```should_panic
//! # let cell = std::cell::RefCell::new(0);
//! let write = cell.borrow_mut(); // Ok
//! cell.borrow(); // Panic
//! ```
//!
//! ```should_panic
//! # let cell = std::cell::RefCell::new(0);
//! let write = cell.borrow_mut(); // Ok
//! cell.borrow_mut(); // Panic
//! ```
//!
//! # Idiomatic Rust
//!
//! That's lot of stuff and idiomatic Rust requires to combine them all to provide guarantees you need.
//! For example, to share replaceable value:
//!
//! ```rust
//! # use std::rc::Rc;
//! # use std::cell::Cell;
//! #
//! let data: Rc<Cell<i32>> = Rc::new(Cell::new(0));
//!
//! let d0 = data.clone();
//! drop(data);
//!
//! let d1 = d0.clone();
//! d0.set(1);
//! # println!("data: {}", d1.get());
//! ```
//!
//! However, to extract value (`.get()`), type must be `Copy` and need to replace entire data.
//!
//! Thus, to share updatable value:
//!
//! ```rust
//! # use std::rc::Rc;
//! # use std::cell::RefCell;
//! #
//! #[derive(Debug)]
//! struct Foobar {
//!     foo: &'static str,
//!     bar: i32,
//! }
//!
//! let data = Rc::new(RefCell::new(Foobar { foo: "The question", bar: 42 }));
//!
//! let d0 = data.clone();
//! drop(data);
//!
//! let d1 = d0.clone();
//! d0.borrow_mut().foo = "The answer";
//! # println!("data: {:?}", d1.borrow());
//! ```
//!
//! Finally, you can rely on [_newtype_ pattern](https://github.com/rust-unofficial/patterns/blob/master/patterns/newtype.md) to provide cleaner API with no runtime overhead through zero-cost abstraction provided by Rust.
//!
//! ```rust
//! mod foobar {
//!
//!     struct InternalFoobar {
//!         foo: &'static str,
//!         bar: i32,
//!     }
//!
//!     use std::rc::Rc;
//!     use std::cell::RefCell;
//!     pub struct Foobar(Rc<RefCell<InternalFoobar>>,);
//!
//!     impl Foobar {
//!         pub fn new(foo: &'static str, bar: i32) -> Self {
//!             Foobar(Rc::new(RefCell::new(InternalFoobar { foo, bar })))
//!         }
//!
//!         pub fn set_foo(&self, foo: &'static str) {
//!             self.0.borrow_mut().foo = foo;
//!         }
//!     }
//! #
//! #     use std::fmt::{Debug, Formatter, Error};
//! #     impl Debug for Foobar {
//! #         fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
//! #             let this = self.0.borrow();
//! #             write!(f, "Foobar {{ foo: {:?}, bar: {:?} }}", this.foo, this.bar)
//! #         }
//! #     }
//! }
//!
//! let foobar = foobar::Foobar::new("The question", 42);
//! foobar.set_foo("The answer");
//! # println!("Result: {:?}", foobar);
//! ```


#[cfg(test)]
mod tests;
