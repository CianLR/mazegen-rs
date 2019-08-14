# MazeGen

This project is built in rust and aims to generate mazes using a variety of methods. The currently implemented methods include:

 - Depth First Search
 - Kruskal's Algorithm
 - Wilson's Algorithm
 - Eller's Algorithm
 - Prim's Algorithm

## Building

To build run the following from the root directory:

```
cargo build
```

## Testing

Tests can be run with the following command:

```
cargo test
```

## Running

To generate a maze run:

```
cargo run
```

To specify a 15x15 maze generated with the DFS algorithm run:

```
cargo run -- -s 15 -a dfs
```

Output:

```
┏━━━━━━━━━━━━━━━┳━━━┳━━━━━┳━━━┓
┃ ╺━━━━━━━━━━━┓ ┃ ╻ ┣━╸ ╻ ┃ ╺━┫
┃ ╺━┳━━━━━┳━╸ ┃ ┃ ┃ ╹ ┏━┫ ┣━╸ ┃
┃ ┏━┛ ┏━╸ ┃ ┏━┫ ┃ ┣━━━┛ ┃ ┃ ╺━┫
┣━┛ ┏━┛ ╺━┛ ┃ ╹ ┃ ┃ ╺━━━┛ ┣━╸ ┃
┃ ╺━┻━━━━━┳━┛ ┏━┛ ┃ ╺━┳━━━┫ ╻ ┃
┣━━━━━━━┓ ┃ ╺━┛ ┏━┻━┓ ┃ ╻ ╹ ┃ ┃
┃ ╻ ╺━━━┫ ┗━━━━━┫ ╻ ╹ ┃ ┗━━━┫ ┃
┃ ┣━━━┓ ┗━━━━━━━┫ ┗━┳━┛ ╺━┓ ┗━┫
┃ ┃ ╻ ┣━╸ ┏━━━┓ ┗━┓ ┃ ┏━━━┻━╸ ┃
┃ ┃ ┃ ╹ ┏━┛ ╺━┛ ┏━┛ ┃ ┃ ┏━━━╸ ┃
┃ ┃ ┣━━━┻━┓ ╺━┳━┛ ┏━┻━┛ ┃ ┏━━━┫
┃ ┃ ┃ ┏━┓ ┗━━━┛ ┏━┛ ┏━━━┫ ┃ ╺━┫
┣━┛ ┃ ╹ ┣━━━┳━━━┫ ╻ ╹ ╻ ┃ ┣━╸ ┃
┃ ╺━┻━╸ ╹ ╻ ╹ ╻ ┗━┻━━━┛ ┃ ╹ ╻ ┃
┗━━━━━━━━━┻━━━┻━━━━━━━━━┻━━━┻━┛
```

