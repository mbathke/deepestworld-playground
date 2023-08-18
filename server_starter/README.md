# Deepest World Backend Starter in Rust

This backend creates websocket server that is called from the frontend application.

## Requirements

- rustc
- cargo 

## Install

```sh
cargo build
```

## Run

It creates a server at "0.0.0.0:7032".
At the current state the server only sends "Hello {count}" message in a loop.
This can then be replaced with game orders.

```sh
cargo run
```
