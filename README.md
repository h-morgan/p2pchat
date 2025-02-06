# p2pchat

Writing a peer 2 peer chat application, in rust

## Pre-setup setup

Install rust

- Navigate to https://www.rust-lang.org/tools/install
- Copy command listed into terminal on your machine, follow prompts
- This install rust and cargo (rust's package manager)

Note: to uninstall rust and undo everything in the setup step above, run
```bash
rustup self uninstall
``` 

## Useful commands

To run the application:
```bash
cargo run
```

To build the application: 
```bash
cargo build
```

To add dependencies:
```bash
cargo add <packages>
```
Note: these get added to the [Cargo.toml](https://github.com/h-morgan/p2pchat/blob/main/Cargo.toml) file.
