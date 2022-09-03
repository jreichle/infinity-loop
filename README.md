# Rusty Infinity Loop

## Intruduction
Infinity loop is a puzzle game built out of a grid of tiles, each with a particular set of connections pointing to orthogonal neighboring tiles. To solve the puzzle, the user must rotate individual tiles to match the connection of all neighboring tiles. The following two images help to demonstrate the visually intuitive ruleset.

## Preparations
To execute the game *Rusty Infinity Loop* you need to update rust to the latest version and install trunk and wasm, in order to use the yew-library.

```
$ rustup update
$ cargo install trunk
$ rustup target add wasm32-unknown-unknown 
```

## Execute the game

Type in the following command to run the server:
```
$ cargo run --bin web
```

If you want to start the program in a developer mode, use the following trunk command. For that, make sure to be in the ui-directory. The web-page will be opened in your standard browser immediatley.
```
$ trunk serve --open
```