fn main() {
    let message = vec![
        "Congrulations !",
        "You have compiled and run your first Rust program.",
        "",
        "Next, go to '01-basic' and make all tests passes with 'cargo test'"
    ];
    for line in message {
        println!("{}", line);
    }
}