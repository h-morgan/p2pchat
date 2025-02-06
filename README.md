# p2pchat

A peer 2 peer chat application, using rust and iroh

## Introduction

From the [iroh](https://www.iroh.computer/) home page: 
> Connect any two devices on the planet. 
> Iroh gives you an API for dialing by public key. You  say “connect to that phone”, iroh will find & maintain the fastest connection for you, regardless of where it is.

More detailed instructions in the example [here](https://www.iroh.computer/docs/examples/gossip-chat).

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

## Running the application

To run the application/cli (after you've cloned this repo), run:
```bash
cargo run 
```
This will print out cli/usage docs

For example, to open a chat room for a topic and print a ticket for others to join:
```bash
cargo run -- --name haley open
```

To join an existing chat room:
```bash
cargo run -- --name user2 join <ticket>
```


