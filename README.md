# Bevy Game of Life (Modular ECS)

A simple implementation of **Conway's Game of Life** built with the [Bevy Engine](https://bevyengine.org/) (v0.15) in Rust.

## Features

- **Sparse Grid**: Uses a `HashSet` to track living cells, allowing for an infinite-feeling universe.
- **Interactive Drawing**: Add cells in real-time using your mouse.
- **Modern Bevy API**: Uses the latest component-based spawning and viewport conversion.
- **camera zooming/panning**: Zoom-in, zoom-out.
- **menu**: Navigation menu.

## Prerequisites

- [Rust and Cargo](https://rustup.rs/)
- System dependencies for Bevy (see [Bevy's setup guide](https://bevyengine.org/learn/book/getting-started/setup/))

## Controls

| Key / Action   | Function                      |
| :------------- | :---------------------------- |
| **Space**      | Play / Pause the simulation   |
| **Left Click** | Draw / Place living cells     |
| **R**          | Reset (Clear) the entire grid |
| **Esc**        | Quit application              |

## How to Run

1. Clone the repository:

```bash
git clone <repo-url>
cd bevy_game_of_life
```

2. Run in release mode for best performance:

```Bash
cargo run --release
```

## Rules of the Game

The simulation follows these four rules:

- Underpopulation: Any live cell with fewer than two live neighbors dies.
- Survival: Any live cell with two or three live neighbors lives on to the next generation.
- Overpopulation: Any live cell with more than three live neighbors dies.
- Reproduction: Any dead cell with exactly three live neighbors becomes a live cell.

---

### Next steps
