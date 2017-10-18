Welcome to Rust workshop
------------------------

It consists into a list of "crates" (packaging name in Rust). Your goal is to make test passes using `cargo test`. You start with `01-basic`, and when you're done, go to `02-...` and continue this way until the end ! Each subdirectories may contain a `README` with few explanations/instructions, so read it !

Before proceeding, you need to install a working environment !

## Installing Rust

At the time of writing, the easiest way is to use [rustup](https://www.rustup.rs/).

_Note: under Windows, you will have to choose between two options. For `gnu`, you need nothing special ; rustup will automatically install GNU compiler tools. For `msvc`, you need to install `Visual C++ Build Tools 2015` ; [click here for more details](https://github.com/rust-lang-nursery/rustup.rs/#user-content-vs2015)._

To check your installation, just run `cargo run` from repository root directory. It should prints

```text
Congrulations !
You have compiled and run your first Rust program

Next, go to '01-basic' and make all tests passes with 'cargo test'
```

## Installing editor

### [Visual Studio Code](https://code.visualstudio.com/)

[Rust (rls)](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust) is currently the recommended extension. It is provided by Rust Developer Tools team (whih also provides [Rust Language Server](https://github.com/rust-lang-nursery/rls)).


There is also [Rust](https://marketplace.visualstudio.com/items?itemName=kalitaalexey.vscode-rust) but it's not actively developped. In this case, using RLS mode is highly recommanded.

### [IntelliJ IDEA](https://www.jetbrains.com/idea/)

_Note: you can also use other IntelliJ-based products. But IntelliJ IDEA has a community version._

[IntelliJ Rust](https://intellij-rust.github.io/) is the official plug-ins. Firstly, started as side projects from JetBrains employees. Since summer 2017, it is officially supported by JetBrains.

### Others

You can find more supported IDEs at [Are we (I)DE yet?](https://areweideyet.com/).
