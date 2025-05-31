# DropJack

This was written for fun using various AI tools and agents, including Junie from Jetbrains and Cursor.

This README is almost 100% AI generated. Take everything, including the code, with a heaping tablespoon of salt.

**DropJack** is a falling block puzzle game with a unique twist - instead of traditional Tetris blocks, you use playing cards! The objective is to create combinations of adjacent cards that sum exactly to **21** (like in Blackjack) to clear them from the board and score points.

## 🎮 Game Overview

DropJack combines the strategic card mechanics of Blackjack with the fast-paced gameplay of falling block puzzles. Cards fall from the top of the screen, and you must position them strategically to create paths of adjacent cards that sum to 21.

### Core Mechanics

- **Card Values**: Standard playing card values (Ace = 1 or 11, Face cards = 10, Numbers = face value)
- **Combination Rules**: Create paths of 2+ adjacent cards (up, down, left, right) that sum exactly to 21
- **Gravity**: When cards are cleared, remaining cards fall down to fill gaps
- **Progressive Difficulty**: Game speed increases over time
- **Scoring**: Earn points for each card cleared in combinations

### Difficulty Modes

- **Easy Mode**: Cards of any suit can be combined together
- **Hard Mode**: Only cards of the same suit can form valid combinations

## 🎯 How to Play

1. **Movement**: Use arrow keys to move the falling card left/right
2. **Drop**: Press down arrow for faster drop, or space for instant hard drop
3. **Objective**: Position cards to create adjacent paths that sum to 21
4. **Strategy**: Plan ahead - longer combinations score more points!
5. **Game Over**: When cards reach the top of the board

### Controls

- `←/→` - Move card left/right
- `↓` - Soft drop (faster fall)
- `Space` - Hard drop (instant placement)
- `Escape` - Pause/Menu

## 🎮 Input Support

DropJack supports both **keyboard and controller** input, with the game automatically detecting your preferred input method and displaying relevant on-screen instructions. Perfect for desktop play or handheld gaming on Steam Deck and other devices.

## 🔧 Technical Details

### Dependencies

- **Raylib** (5.5.1) - Graphics rendering and window management
- **rusqlite** (0.36.0) - SQLite database for high scores
- **rand** (0.9.1) - Random number generation for deck shuffling
- **chrono** (0.4.31) - Date/time handling for high score timestamps

### Key Algorithms

1. **Combination Detection**: Uses recursive pathfinding to detect all possible 21-sum paths
2. **Gravity System**: Implements realistic card falling with smooth animations
3. **Collision Detection**: Checks valid card placement and movement boundaries
4. **State Management**: Clean separation between game logic and rendering

### Architecture Highlights

- **Modular Design**: Clear separation of concerns across modules
- **Entity-Component Pattern**: Cards have both logical and visual position components
- **Event-Driven Updates**: Game state changes trigger appropriate visual effects
- **Database Abstraction**: Clean interface for high score persistence
- **Smooth Animations**: Visual positions interpolate smoothly to logical positions

## 🚀 Building and Running

### Prerequisites

- Rust (latest stable version)
- Cargo (comes with Rust)

### Build Instructions

```bash
# Clone the repository
git clone <repository-url>
cd dropjack

# Build the project
cargo build --release

# Run the game
cargo run --release
```

### Development

```bash
# Run in debug mode
cargo run

# Run tests
cargo test

# Check code with clippy
cargo clippy
```

## 🎨 Features

- **Smooth Animations**: Cards fall and move with fluid interpolation
- **Particle Effects**: Visual feedback for card destruction
- **High Score System**: Persistent leaderboard with difficulty tracking
- **Progressive Difficulty**: Game speed increases over time
- **Multiple Game Modes**: Easy and Hard difficulty with different rules
- **Clean UI**: Modern, readable interface with card graphics

## 🏆 Scoring System

- Base score: 21 points per cleared card
- Longer combinations preferred for strategic gameplay
- High scores saved with player initials and difficulty mode
- Separate leaderboards for Easy and Hard modes

## 🔮 Future Enhancements

The modular architecture makes it easy to add:

- Power-up cards with special abilities
- Additional difficulty modes
- Online multiplayer support
- Custom card themes
- Tournament mode
- Achievement system

---

Enjoy playing DropJack! 🃏✨
