02 Control
----------

Welcome to second step of this Rust workshop.

This step focuses on structure controls in Rust.

## Conditional branching

As in many languages, conditions are expressed with `if` and `else` keywords (and chained with `else if`). Condition expression isn't put into parentheses but curly braces are mandatory.

```rust
if a_condition {
    do_something();
} else if one_variable == 42 {
    do_alternative();
} else {
    otherwise();
}
```

As describe previously, blocks are also expressions and so, the same apply for conditional.

```rust
fn fizzbuzz(num: u32) -> String {
    if num % 15 == 0 {
        String::from("FizzBuzz")
    } else if num % 5 == 0 {
        String::from("Buzz")
    } else if num % 3 == 0 {
        String::from("Fizz")
    } else {
        num.to_string()
    }
}
```

## Conditional loop

A condition can be checked in loop until it goes `false` using `while` keyword.

```rust
while value != 0 {
    value = next(value);
}
```

As usual, `break` can be used to exit loop and `continue` to jump to next iteration.

## Infinite loop

Infinite loop can be created with `loop` keyword. And `break` can be used to "return" a value.

```rust
let state = loop {
    let value = next();
    if value == 0 {
        break OK;
    } else if value < 0 {
        break ERR;
    }
}
```

## Iterative loop

`for` keyword is used to create a loop iterating over a collection, a range, ...

```rust
for element in vec![0, 1, 2, 3, 4] {
    // ...
}

for i in 0..5 {
    // ...
}
```
