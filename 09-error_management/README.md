09 Error management
-------------------

Welcome to ninth step of this Rust workshop.

This step focuses on error handling.

## May success or fail

[Result](https://doc.rust-lang.org/stable/std/result/enum.Result.html) enum is used in Rust to indicate that a function may returns a successful value or an error.

```rust
fn i64_divide(dividend: i64, divisor: i64) -> Result<i64, DivisionError> {
    if divisor == 0 {
        Err(DivisionError::Zero)
    } else {
        let quotient = dividend / divisor;
        if divided != quotient * divisor {
            Err(DivisionError::NotInteger)
        } else {
            Ok(quotient)
        }
    }
}
```

Interesting things is that `Result` has very nice functional API:

```rust
Ok(2).map(|ok| ok.to_string()) // Ok("2")
Err("error").map(|ok| ok.to_string()) // Err("error")

Ok(2).map_err(|err| err.to_string()) // Ok(2)
Err("error").map_err(|err| err.to_uppercase()) // Err("ERROR")


Ok(2).and(Ok(4)) // Ok(4)
Ok(2).and(Err("error")) // Err("error")

Err("error").and(Ok(4)) // Err("error")
Err("fatal").and(Err("useless")) // Err("fatal")
```

## Forwarding error

When chaining call, checking and returning back error may be quite annoying:

```rust
fn parse_foo(input: &str) -> Result<Foo, IoError> { /* ... */ }
fn parse_bar(input: &str) -> Result<Bar, IoError> { /* ... */ }

fn read(input: &str) -> Result<FooBar, IoError> {
    let foo = match parse_foo(input) {
        Ok(foo) => foo,
        Err(io_error) => return Err(io_error)
    };

    let bar = match parse_bar(input) {
        Ok(bar) => bar,
        Err(io_error) => return Err(io_error)
    };

    FooBar { foo, bar }
}
```

`?` operator can be used to simplify such pattern:

```rust
fn read(input: &str) -> Result<FooBar, IoError> {
    let foo = parse_foo(input)?;

    let bar = parse_bar(input)?;

    FooBar { foo, bar }
}
```

Finally, Error can also be converted if `From` trait is implement for function error type:

```rust
enum MyError {
    IoError(io::Error),
    FooError(foo:Error),
}

impl From<io::Error> for MyError {
    fn from(e: io::Error) -> MyError {
        MyError::IoError(e)
    }
}

impl From<foo::Error> for MyError {
    fn from(e: foo::Error) -> MyError {
        MyError::FooError(e)
    }
}

fn do_io() -> Result<Foo, io::Error> {}
fn do_foo() -> Result<(), foo::Error> {}
fn do() -> Result<Bar, MyError> {
    let foo = do_io()?;
    fo_foo()?

    Ok(Bar)
}
```

## Optional values

`null` value is called "the billion dollar mistake". So, Rust has no `null` but a missing value can still be expressed with `std::option::Option` enum.

```rust
fn make_optional(with_value: bool) -> Option<String> {
    if with_value {
        Some(String::from("value"))
    } else {
        None
    }
}

Some(String::from("foo")).unwrap_or(String::from("bar")) // "bar"
None.unwrap_or_else(|| String::from("default")) // "default"

// Return error if value is missing
fn may_fail() -> Result<Foo, FooError> {
    let bar = search_bar().ok_or_else(FooError::NotFound)?;
    Ok(Foo { bar })
}
```

## Panic

Rust has capability similar to `Exception` in other languages through `panic!` macro. It currently stops running thread and if it's `main`, program is stopped.

```rust
fn stop_thread(code: u64) {
    panic!("Abort thread (error code: {})", code);
}

let optional = None;
optional.expect("No value");

let must_success = Err("but it fails !");
must_success.expect("it should have worked"); // panic: "it should have worked: but it fails !"

let must_fail = Ok("but it was a surprising success !");
must_fail.expect_err("it must not work"); // panic: "it must not work: but it was a surprising success !"
```

`panic` is generally advised when invariants are broken, state is valid, caller is not expected to programmatically handle error (caller bug), ... 