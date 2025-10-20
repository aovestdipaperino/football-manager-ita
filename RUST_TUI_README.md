# Football Manager TUI - Rust Edition

A modern Rust TUI (Terminal User Interface) conversion of the classic Commodore 64 BASIC Football Manager game by Daniele Piccoli.

## About

This project faithfully recreates the original C64 Football Manager game mechanics and algorithms in Rust, providing a terminal-based interface using the `ratatui` library. The core game logic, match simulation algorithms, and financial systems are directly adapted from the original BASIC source code.

## Features

### From the Original Game
- **4 Italian Football Leagues**: Serie A, B, C1, C2
- **64 Teams**: All original Italian teams across divisions
- **24 Famous Players**: Original roster from the 1980s (Maradona, Platini, Zico, etc.)
- **Player Management**: Buy, sell, and manage your squad
- **Match Simulation**: Uses the original C64 algorithms for realistic match outcomes
- **Financial System**: Banking, loans, weekly costs, and income
- **Team Morale**: Dynamic morale system affecting performance
- **Season Progression**: Promotion/relegation system across all leagues
- **League Standings**: Real-time league tables with goal difference

### Modern Enhancements
- **TUI Interface**: Clean, navigable terminal interface
- **Responsive Controls**: Keyboard-driven navigation
- **Real-time Updates**: Live game state updates
- **Cross-platform**: Runs on Linux, macOS, and Windows

## Installation

### Prerequisites
- Rust 1.70 or later
- A terminal emulator

### Build from Source

```bash
# Clone the repository
cd football-manager-ita

# Build the project
cargo build --release

# Run the game
cargo run --release
```

## How to Play

### Controls
- **Arrow Keys / Numbers**: Navigate menus and select options
- **Enter**: Confirm selection
- **ESC**: Go back to previous screen
- **Q**: Quit game
- **G**: Play match (from main menu)

### Main Menu Options

1. **Sell or List Players** - View your squad, check player values, and sell players
2. **Get a Loan** - Access the Sport Bank to take out loans (with 20% interest)
3. **View Standings** - See the current league table and your position
4. **View Team Status** - Check your finances, morale, and team condition
5. **Change Team Names** - Customize team names (not yet implemented)
6. **Change Player Names** - Customize player names (not yet implemented)
7. **Play Match** - Simulate a match against a random opponent
8. **Restart** - Start over with a new team

### Game Flow

1. **Start**: Begin with 150,000 in the bank and a basic squad
2. **Manage Squad**: Buy and sell players to build your team
3. **Play Matches**: Simulate matches and watch results
4. **Monitor Finances**: Pay weekly costs, manage debt
5. **Progress**: Win matches to gain points, avoid relegation, achieve promotion

### Match Simulation

The match engine uses the original C64 algorithms:
- **Team Stats**: Energy, Morale, Defense, Midfield, Attack
- **Player Attributes**: Style (1-5) and Power (1-20) affect team ratings
- **Goal Calculation**: Based on attacking vs defensive ratings
- **Morale Impact**: Wins boost morale, losses decrease it

### Financial System

- **Starting Money**: 150,000
- **Weekly Costs**:
  - Field rent: 70 per player + (5-league_level) * 10
  - Miscellaneous: 500 * (5-league_level)
  - Loan interest: debt / 20
- **Income Sources**:
  - Match attendance (varies by position in table)
  - Player sales
- **Loans**:
  - Max debt: 100,000 * (5-league_level)
  - Interest rate: 20%
  - Automatic weekly payments

## Original C64 Algorithms

This implementation preserves the key algorithms from the original BASIC game:

### Player Market Value
```rust
// Original: 5000*(5-N)+5000*A(PZ)
value = 5000 * (5 - league_level) + 5000 * player_style
```

### Team Stats Calculation
```rust
// Original: D(6)=INT(D(3)+D(4)/2+D(1)/2+D(2)/2)
defense_rating = (defense + midfield/2 + energy/2 + morale/2)

// Original: D(7)=INT(D(7)+(D(6))/2+(D(3))/2+(D(4))/2)
attack_rating = (attack + defense_rating/2 + defense/2 + midfield/2)
```

### Goal Scoring
```rust
// Original: IFINT(RND(1)*D(7)-(RND(1)*D(13)))>0
if rng(attack_rating) - rng(opponent_defense_rating) > 0 {
    goal_scored();
}
```

### Morale Update
```rust
// Win: K=INT(20-K)/2+K
morale = (20 - morale) / 2 + morale

// Draw: K=INT(K/2)+1
morale = morale / 2 + 1
```

## Project Structure

```
src/
├── main.rs           # Application entry point and event loop
├── game_state.rs     # Core game state management
├── player.rs         # Player data structures and logic
├── team.rs           # Team data structures and stats
├── match_engine.rs   # Match simulation engine (original algorithms)
├── market.rs         # Transfer market and financial system
└── ui.rs             # Terminal UI rendering
```

## Technical Details

- **Language**: Rust 2021 Edition
- **TUI Library**: ratatui 0.28
- **Terminal Backend**: crossterm 0.28
- **Random Number Generation**: rand 0.8
- **Serialization**: serde 1.0 (for future save/load features)

## Differences from Original

### Adaptations for Modern TUI:
- Input is menu-based rather than prompt-based
- Visual layout optimized for modern terminals
- Some original PETSCII graphics replaced with Unicode
- Team/player name editing not yet implemented
- Save/load system to be added

### Preserved from Original:
- All 64 teams and 24 players
- Complete match simulation algorithm
- Financial calculations
- Player power/style systems
- League progression mechanics
- Morale system

## Future Enhancements

- [ ] Save/load game feature
- [ ] Team and player name customization
- [ ] Enhanced match visualization
- [ ] Statistics tracking (historical data)
- [ ] Animation for goals/match events
- [ ] Sound effects (optional)
- [ ] Season review screens
- [ ] Trophy/achievement system

## Credits

- **Original Game**: Daniele Piccoli (C64 BASIC, 1980s)
- **Rust Conversion**: 2025
- **TUI Framework**: ratatui team
- **Original Game Preservation**: This project

## License

This is a modernization of a historical Commodore 64 program, preserved and adapted for educational purposes.

## Running the Game

```bash
# Quick start
cargo run --release

# Development mode (with debug info)
cargo run

# Build only
cargo build --release

# The binary will be in:
./target/release/football_manager_tui
```

---

**Note**: This is a faithful recreation of the 1980s gameplay experience with modern tooling. The original game's charm and mechanics have been preserved while providing a better user experience through the TUI interface.
