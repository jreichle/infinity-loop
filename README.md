# Rusty Infinity Loop

## Gameplay

[Infinity Loop][infinityloop] is a puzzle game built out of a grid of tiles, each with a particular set of connections pointing to orthogonal neighboring tiles. To solve the puzzle, the user must rotate individual tiles to match the connection of all neighboring tiles. The following two images help to demonstrate the visually intuitive ruleset.

**Example of a puzzle:**

![unsolved][unsolvedexample]

**And its solution:**

![solved][solvedexample]

> **_NOTE:_**  A given puzzle may have multiple valid solutions.

## Features

* WebAssembly Browser-UI
* Infinite Level Generator
* Level Solver
* Level Editor
* Requestable Hints
* Level Parsing and Storing

## Installation Guide

### Prerequisites

Clone this repository into a local directory.

```shell
git clone https://gitlab.lrz.de/rust-praktikum/infinity-loop-rust
```

This project runs on Rust version `1.6.3` or later.
Additionally the dependencies _trunk_ and _wasm_ are required, in order to use the _yew_ package

```shell
rustup update
cargo install trunk
rustup target add wasm32-unknown-unknown 
```

### Running the Application

Type in the following command to run the server:

```shell
cargo run --bin web
```

If you want to start the program in a developer mode, use the following trunk command while in the frontend directory. The web-page opens in your standard browser momentarily.

```shell
cd frontend/
trunk serve --open
```

[infinityloop]: <https://play.google.com/store/apps/details?id=com.balysv.loop&hl=de&gl=US>

[unsolvedexample]: <./docs/images/example-level.png>
[solvedexample]: <./docs/images/example-level-solution.png>
