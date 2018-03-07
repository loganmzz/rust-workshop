//! 10 Collections
//! --------------
//!
//! Welcome to tenth step of this Rust workshop.
//!
//! This step focuses on managing collection of data.
//!
//! ## Vector
//!
//! `std::vec::Vec` has already been introduced before and is generally initialized from `vec!` macro. It consists into a growable and ordered collection of items and it is backed by an array.
//!
//! ```rust
//! let v123 = vec![1, 2, 3];
//! let v11111 = vec![1; 5];
//! let pre_sized = Vec::<i32>::with_capacity(10);
//!
//! let mut vec = vec![1, 2, 3, 4, 5];
//! let first = vec[0];
//! vec.push(6);
//!
//! if let Some(last_in) = vec.pop() {
//!     /* ... */
//! }
//! ```
//!
//! ## Slice
//!
//! Previously, `&str` has been introduced as a borrowed String. In fact, it is a [_slice_](https://doc.rust-lang.org/stable/std/primitive.slice.html). And `String` is like a `Vec` of UTF-8 encoded character. Slices (`&[T]`) don't necessarly cover entire collection:
//!
//! ```rust
//! let vec = vec![1, 2, 3, 4, 5, 6];
//! let from_2_upto_4: &[usize] = &vec[1..4];
//! ```
//!
//! ## Range
//!
//! Range (`inclusive_start..exclusive_end`) is an iterator structure that can [be fully defined](https://doc.rust-lang.org/stable/std/ops/struct.Range.html), [have only lower bound](https://doc.rust-lang.org/stable/std/ops/struct.RangeFrom.html), [upper bound](https://doc.rust-lang.org/stable/std/ops/struct.RangeTo.html) or [no bound at all](https://doc.rust-lang.org/stable/std/ops/struct.RangeFull.html). So, it can be used for indexing but also for iterating.
//!
//! ```rust
//! let vec: Vec<_> = (1..7).collect();
//!
//! let drop8 = (0..8).chain(9..);
//! for i in drop8.take(16) {
//!     println!("{}", i);
//! }
//! ```
//!
//! ## Map & Set
//!
//! Like most languages, hash-based map is available through [HashMap](https://doc.rust-lang.org/std/collections/struct.HashMap.html).
//!
//! ```rust
//! #[derive(PartialEq, Eq, Hash)]
//! struct Key { name: String }
//! struct Item { key: Key, val: String }
//!
//! # impl Key { fn new(name: &str) -> Self { Key { name: String::from(name) } } }
//! # impl Item { fn new(key: &str, val: &str) -> Self { Item { key: Key::new(key), val: String::from(val) } } }
//!
//!
//! use std::collections::HashMap;
//!
//! let mut map = HashMap::new();
//! map.insert(Key::new("foo"), Item::new("foo", "bar"));
//!
//! let mut map: HashMap<_,_> = vec![  (Key::new("foo"), Item::new("foo", "bar")),
//!                  (Key::new("to") , Item::new("foo", "bar"))
//!               ].into_iter().collect();
//!
//! if let Some(borrowed_item) = map.get(&Key::new("foo")) {
//!     /* ... */
//! }
//!
//! let mut borrowed_item = map.entry(Key::new("hoge")).or_insert_with(|| Item::new("hoge", "piyo"));
//! ```
//!
//! If sorted element is required, then [BTreeMap](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html) must be used.
//!
//! ```rust
//! use std::collections::BTreeMap;
//!
//! let mut sorted_map = BTreeMap::new();
//! sorted_map.insert("rab", "oof");
//! sorted_map.insert("rba", "ofo");
//! sorted_map.insert("arb", "oof");
//!
//! use std::collections::Bound::{Included, Unbounded};
//! let range = (Included(&"r"), Unbounded);
//! for entry in sorted_map.range::<&str,_>(range) {
//!     /* ... */
//! }
//!
//! let greater_or_equal_to_rab = sorted_map.split_off(&"rab");
//! let lesser_than_rab = sorted_map;
//! ```
//!
//! If value doesn't matter and only key uniqueness does, then use [HashSet](https://doc.rust-lang.org/std/collections/struct.HashSet.html) and [BTreeSet](https://doc.rust-lang.org/std/collections/struct.BTreeSet.html). They are just wrapper over `*Map<K,()>`.

#[cfg(test)]
mod tests;
