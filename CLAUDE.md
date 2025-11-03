# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust TUI (Terminal User Interface) conversion of a classic Commodore 64 BASIC football (soccer) management game originally written by Daniele Piccoli in the 1980s. The project faithfully recreates the original game mechanics, match simulation algorithms, and financial systems while providing a modern terminal interface using the `ratatui` library.

## Essential Commands

```bash
# Build and run in development mode
cargo run

# Build and run optimized release version
cargo run --release

# Build only
cargo build --release

# Check code without building
cargo check

# Run tests (when implemented)
cargo test
```

## Architecture Overview

### Module Structure

The codebase is organized into 6 core modules:

1. **`main.rs`**: Application entry point with TUI event loop
   - Handles terminal setup/teardown
   - Routes keyboard events to appropriate screens
   - Manages game state lifecycle

2. **`game_state.rs`**: Central game state management
   - `GameState` struct holds all game data (players, teams, finances, morale)
   - Implements core operations: loan management, weekly costs, league standings
   - Initializes all 64 teams across 4 Italian leagues and 24 famous players

3. **`player.rs`**: Player data structures
   - `Player` struct with `style` (1-5) and `power` (1-20) attributes
   - `PlayerStatus` enum: Free, Owned, Playing, Substitute, Sold
   - Player market value calculation based on style and league level

4. **`team.rs`**: Team data and statistics
   - `Team` struct with match records and league information
   - `TeamStats` for calculating defense/midfield/attack ratings
   - Hardcoded team names for all 4 Italian leagues (Serie A/B/C1/C2)

5. **`match_engine.rs`**: Match simulation using original C64 algorithms
   - `simulate_match()`: Core match simulation preserving original BASIC logic
   - `calculate_opponent_stats()`: AI team rating generation based on points
   - `simulate_other_matches()`: Simulates league matches for other teams
   - `update_morale()`: Post-match morale adjustments

6. **`market.rs`**: Transfer market and financial operations
   - Player buying/selling with price variation
   - Banking system with 20% interest on loans
   - Max debt limits based on league level

7. **`ui.rs`**: Terminal UI rendering with ratatui
   - `Screen` enum for navigation state
   - Renders all game screens: menus, squad view, standings, finances
   - Uses ratatui widgets for layout

### Critical Algorithm Preservation

The codebase preserves the original C64 BASIC algorithms exactly. When modifying match simulation or financial calculations, **always reference the original BASIC code** documented in `CONVERSION_NOTES.md` and `footballmanager_documented.txt`.

Key preserved formulas:
- **Player market value**: `5000 * (5 - league_level) + 5000 * player_style`
- **Loan with interest**: `debt += amount * 1.2; weekly_interest = debt / 20`
- **Team defense rating**: `(defense + midfield/2 + energy/2 + morale/2)`
- **Team attack rating**: `(attack + defense_rating/2 + defense/2 + midfield/2)`
- **Goal scoring**: `rng(attack_rating) - rng(opponent_defense_rating) > 0`
- **Morale on win**: `(20 - morale) / 2 + morale`
- **Morale on draw**: `morale / 2 + 1`

### Data Flow

1. **Game Initialization** (`GameState::new`):
   - Creates all 24 players with randomized stats
   - Initializes all 64 teams across 4 divisions
   - Sets starting finances (150,000) and morale (15)
   - Assigns 8 players to player's team

2. **Match Flow** (`play_match` in main.rs):
   - Select random opponent from same league
   - Calculate player team stats from owned players
   - Generate opponent stats based on points
   - Simulate match using original algorithms
   - Update team records, morale, finances
   - Simulate other league matches
   - Pay weekly costs
   - Update player power levels

3. **Financial Cycle** (`pay_weekly_costs` in game_state.rs):
   - Field rent: `player_count * (70 + (5-league) * 10)`
   - Misc costs: `500 * (5-league)`
   - Interest payment: `debt / 20`
   - If money < 0, take automatic loan with 20% interest

### Important Design Patterns

- **Borrow checker management**: Functions often collect data into temporary vectors before mutating `GameState` to avoid multiple mutable borrows
- **Original variable mapping**: BASIC array variables (A$, B$, D(), etc.) are mapped to Rust structs - see table in `CONVERSION_NOTES.md`
- **League indexing**: Leagues are 1-indexed (1=A, 2=B, 3=C1, 4=C2) matching the original
- **Team IDs**: Teams 0-15 are Serie A, 16-31 Serie B, 32-47 Serie C1, 48-63 Serie C2

## Development Workflow

### Adding New Features

When adding features:
1. Check `RUST_TUI_README.md` "Future Enhancements" section for planned features
2. If modifying game mechanics, consult `footballmanager_documented.txt` for original behavior
3. Use `CONVERSION_NOTES.md` to understand BASIC-to-Rust mappings
4. Test that original algorithm behavior is preserved

### Common Gotchas

- **Player IDs vs Array Indices**: Player IDs are 0-indexed but position calculations use `(id+1)/8`
- **Integer Division**: Rust's `/` for integers matches BASIC's `INT()` division
- **Random Number Generation**: Original uses `RND(1)*X` for range [0,X); Rust uses `gen_range(0..X)`
- **2-Point Win System**: Original game awards 2 points for a win (not 3), which is historically accurate

### Screen Navigation State

The `Screen` enum in ui.rs defines all possible UI states. When adding new screens:
1. Add variant to `Screen` enum
2. Implement rendering in `ui.rs::draw()`
3. Add keyboard handling in `main.rs::run_app()`
4. Add escape key handler to return to `Screen::GameMenu`

## Reference Documentation

- **`README.md`**: Original C64 game documentation
- **`RUST_TUI_README.md`**: Rust TUI implementation details and controls
- **`CONVERSION_NOTES.md`**: Line-by-line C64 BASIC to Rust algorithm mapping
- **`footballmanager_documented.txt`**: Complete original BASIC code analysis
- **`footballmanager.bas`**: Original C64 BASIC source code

## Testing Notes

Currently, the project has no automated tests. When implementing tests:
- Focus on algorithm preservation (match simulation, financial calculations)
- Test edge cases: bankruptcy, maximum debt, morale bounds
- Verify league standings sorting (points, then goal difference)
- Test player market value calculations at different league levels
