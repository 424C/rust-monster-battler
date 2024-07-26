# Monster Battler

Monster Battler is a simple, roguelike text-based role-playing game written in Rust. Players create a character and engage in battles with randomly generated monsters, collecting treasure and improving their stats along the way.

## Features

- Character creation with randomized stats
- Turn-based combat system
- Dice rolling mechanics (D6 and D20)
- Treasure hunting between battles
- Simple game loop with option to continue or end the game

## Prerequisites

Before you begin, ensure you have met the following requirements:

- Rust programming language (latest stable version)
- Cargo (Rust's package manager, usually comes with Rust)

## Installation

1. Clone the repository:

   ```
   git clone https://github.com/yourusername/monster-battler.git
   cd monster-battler
   ```

2. Build the project:
   ```
   cargo build --release
   ```

## Usage

To start the game, run:

```
cargo run --release
```

Follow the on-screen prompts to:

1. Choose your character
2. Engage in battles
3. Find treasure
4. Decide whether to continue playing or end the game

## Game Mechanics

- Characters have four main stats: Health, Attack, Defense, and Gold
- Combat is turn-based, with initiative determined by a D20 roll
- Damage is calculated using D6 rolls
- Between battles, players have a chance to find treasure and improve their stats

## License

This project uses the [MIT](https://choosealicense.com/licenses/mit/) license.

## Acknowledgements

- Thanks to the Rust community for providing excellent documentation and resources.
- This project was inspired by classic text-based RPGs and tabletop games.
