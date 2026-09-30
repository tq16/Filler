# Filler Bot

A competitive game-playing bot built in **Rust** for the Filler strategy game.

The bot reads the current game state from the Filler engine, finds every valid placement for the given piece, evaluates the available moves, and chooses a position designed to move toward the opponent and compete for control of the board.

## How the Game Works

Filler is a two-player strategy game played on a grid.

Each turn, the game engine provides a piece that must be placed on the board.

A valid placement must:

- Overlap exactly one cell already owned by the player
- Never overlap an opponent's cell
- Stay completely inside the board

The objective is to expand across the board and prevent the opponent from gaining space.

## Bot Strategy

The bot evaluates all valid positions for every new piece.

Its strategy focuses on approaching the opponent and creating pressure on their available space.

For each turn, the bot:

1. Parses the current board and piece.
2. Finds every legal placement.
3. Builds a distance map from the opponent's cells.
4. Measures how close each possible move is to the opponent.
5. Counts direct neighboring contacts with opponent cells.
6. Selects the move with the shortest distance.
7. Uses enemy contact as a tie-breaker.

This encourages the bot to expand toward the opponent rather than simply filling empty areas randomly.

## Tech Stack

- **Rust**
- **Cargo**
- **Docker**
- Game engine communication through standard input/output

## Project Structure

```text
Filler/
└── docker_image/
    ├── maps/                  # Game maps
    ├── linux_robots/          # Linux opponent bots
    ├── m1_robots/             # Apple Silicon opponent bots
    ├── linux_game_engine
    ├── m1_game_engine
    ├── Dockerfile
    └── solution/
        ├── src/
        │   ├── main.rs        # Main game loop
        │   ├── models.rs      # Player and turn models
        │   ├── parser.rs      # Game input parsing
        │   ├── placement.rs   # Move validation
        │   ├── scoring.rs     # Distance and contact scoring
        │   ├── strategy.rs    # Move selection strategy
        │   ├── output.rs      # Coordinate output
        │   └── tests.rs       # Automated tests
        ├── Cargo.toml
        └── Cargo.lock
```

## Building with Docker

From the `docker_image` directory:

```bash
docker build -t filler .
```

Run the container and mount the solution directory:

```bash
docker run -v "$(pwd)/solution":/filler/solution -it filler
```

## Running a Match

Inside the container, a match can be started with the game engine.

Example:

```bash
./linux_game_engine \
  -f maps/map01 \
  -p1 solution/target/release/solution \
  -p2 linux_robots/wall_e
```

You can change:

- The map
- Player 1
- Player 2
- The opponent robot

Available maps include:

```text
map00
map01
map02
```

The project also includes several opponent bots such as:

```text
bender
h2_d2
terminator
wall_e
```

## Move Validation

Before evaluating a move, the bot checks that:

- The entire piece fits inside the board
- The piece does not overlap an enemy cell
- The piece overlaps exactly one of the player's existing cells

Only valid moves are passed to the scoring system.

## Distance Scoring

The bot creates a distance map from the opponent's occupied cells.

This allows it to determine how close each possible piece placement is to the opponent.

The move with the shortest distance is preferred.

If multiple moves have the same distance, the bot prefers the move with more direct contact around enemy cells.

## Input and Output

The bot communicates with the game engine using standard input and output.

It reads:

- Player information
- Board state
- Current piece

It returns coordinates in the format:

```text
x y
```

For example:

```text
7 2
```

## Testing

The project includes automated tests for:

- Board and piece parsing
- Valid placement detection
- Single-cell overlap rules
- Multiple-overlap rejection
- Enemy-overlap rejection
- Out-of-bounds rejection
- Coordinate output formatting

Run the tests from the solution directory:

```bash
cargo test
```

## What I Learned

This project gave me practical experience with:

- Algorithm design
- Grid-based game logic
- Competitive bot strategies
- Distance-based scoring
- Move validation
- Input parsing
- Rust modules
- Standard input/output communication
- Automated testing
- Docker-based development environments

## Author

**Taqi Sarhan**

Software Developer | Programming Student at Bahrain Polytechnic | Reboot01 Developer
