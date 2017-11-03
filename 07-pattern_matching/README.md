07 Pattern matching
-------------------

Welcome to seventh step of this Rust workshop.

This step focuses on dispatching code based on value.

## Destructure

Same way shorthand syntax is used to bind a variable into a field, variables can be created from struct fields (including tuples):

```rust
struct FooBar { foo: u8, bar: u8, none: () }
let foobar = FooBar { foo: 2, bar: 13, none: () };

struct List(u8, u8, u8);
let list = List(2, 4, 6);

let FooBar { foo: oof, bar, .. } = foobar;
let List(i0, _, i2) = list;

println!("oof: {}   bar: {}  i0: {}  i2: {}", oof, bar, i0, i2);
```

_Note: `..` is used here to specify not all fields are bound and `_` as a placeholder._

## Enum

However, such construction can't be directly used with `enum`. Because they have many variants. Instead destructuring can be used as conditional clause in both `if` and `while` statements.

```rust
enum Result {
    Next(u8),
    Terminate,
}

let result = Result::Next(42);
if let Result::Next(next) = result {
    println!("next: {}", next);
} else {
    println!("terminate");
}
```

## Match

But there's more powerful control structure: `match`.

```rust
let value = 256;

let result = match value {
    0 => -1,
    2 | 4 | 6 | 8 => value + 5,
    10 ... 51 => value - 8,
    _ => value * 3
};
```

A `match` must be **exhaustive** so `_` is used as placeholder for any case.

## Guard

Extracted value can also be checked with an additionnal `if` clause.

```rust
let result = Result::Next(16);
match result {
    Result::Next(v) if v < 12 => println!("below threshold"),
    Result::Next(v) if v < 50 => println!("not so bad"),
    Result::Next(_)           => println!("too much"),
    _                         => ()
}
```
