# MazeGen

This project is built in rust and aims to generate mazes using a variety of methods. The currently implemented methods include:

 - Depth First Search
 - Kruskal's Algorithm
 - Wilson's Algorithm
 - Eller's Algorithm

## Building

To build run the following from the root directory:

```
cargo build
```

## Running

To generate a 15x15 maze build the project and then run:

```
cargo run dfs 15
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

