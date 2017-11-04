08 Generics
-----------

Welcome to eighth step of this Rust workshop.

This step focuses on defining abstraction over types.

## Syntax

Generic syntax was already introduced on lifetime chapter. In facts, type variable are declared between angular brackets (`<>`) and a single uppercase letter is generally used.

```rust
struct Container<E> {
    item: E;
}

impl<E> Container<E> {
    fn item(&self) -> &E {
        &self.item
    }
}
```

## Bounds

Generics in Rust works similarly to C++ templates as code is compiled for each use. Thus, it permits very interesting things such as provided implementations based on which traits type variable implements. For example by using trait bounds:

```rust
impl<E: std::fmt::Debug> Container<E> {
    fn format(&self) -> String {
        format!("[{:?}]", self.item())
    }
}
```

Associated type may also be specified using their name and a "binding" style.

```rust
trait MyTrait {
    type AssociatedType;
}

impl<A, B: MyTrait<AssociatedType=A>>  MyType { /* ... */ }
```

## Target type

But things can be more interesting by making entire trait implementation generic. Meaning custom trait can be "automatically" implemented for any target type (or such matching some trait bounds).

```rust
#[derive(Debug)] //Container impl Debug if E also does
struct Container<E> { /* ... */ }
struct NonDebug;

trait Diagnostic {
    fn diagnose(&self) -> String;
}

impl<E: std::fmt::Debug> Diagnostic for E {
    fn diagnose(&self) -> String {
        format!("{:?}", self)
    }
}

// Ok:
//  - String is Debug
//  - so, Container also
//  - so, Container is Diagnostic
Container { item: String::from("info") }.diagnose();

// Compilation error
Container { item: NonDebug }.diagnose(); 
```

## Where

To make trait bounds clearer or to express more complex conditions, a `where` clause can be used.

```rust
// Clearer
impl<A: Trait1 + Trait2 + Trait3, B: Trait1 + Trait4> MyTrait<A> for B {}
impl<A, B> MyTrait<A> for B where
    A: Trait1 + Trait2 + Trait3,
    B: Trait1 + Trait4 {}

// Only available with where clause
impl<A, B> MyTrait<A> for B where
    Container<B>: Debug,
    Container<A>: Debug {}
```
## Multiple trait implementions

Finally another interesting thing with Rust generic implementation is capability of having many implementations of a single generic trait.

```rust
trait Convert<E> {
    fn convert(&self): E;
}

impl Convert<String> for MyType { /* ... */ }
impl Convert<Container<MyType>> for MyType { /* ... */ }
impl Convert<u64> for MyType { /* ... */ }

let explicit_string: String = my_type.convert();
let explicit_container: Container<_> = my_type.convert(); // Container type is inferred

let inferred_u64 = my_type.convert();
consuming_u64(inferred);
```
