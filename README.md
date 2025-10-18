# Football Manager C-64

A complete Italian football (soccer) management simulation game written in Commodore 64 BASIC by Daniele Piccoli.

## About

This is a classic football management game for the Commodore 64 that allows players to manage an Italian football team through multiple seasons across different Italian league divisions (Serie A, B, C1, C2). The game features team management, player trading, financial systems, match simulations, and full season progression.

## Project Files

### `footballmanager.bas`
The main game source code written in Commodore 64 BASIC. This file contains:
- **Initialization and Setup** (Lines 10-810): Screen setup, variable initialization, team/player data
- **Main Menu System** (Lines 820-1000): Primary navigation and game options
- **Player Trading/Market** (Lines 1070-1460): Buy/sell players, transfer system
- **Banking/Financial System** (Lines 1470-1620): Team budget management, loans
- **League Tables/Statistics** (Lines 1630-1870): Rankings, standings, team statistics
- **Team Status Display** (Lines 1880-2900): View your squad, player stats
- **Match Simulation Engine** (Lines 3000-4280): Core gameplay - simulates football matches
- **Season Management** (Lines 4060-4300): Progress through seasons, promotions/relegations

This file can be loaded into a Commodore 64 emulator (like VICE) or transferred to real C64 hardware.

### `footballmanager.prg`
The compiled/tokenized program file ready to run on Commodore 64 hardware or emulators. This is the binary executable version of `footballmanager.bas` that can be directly loaded and run with:
```
LOAD "footballmanager.prg",8,1
RUN
```

### `footballmanager_documented.txt`
Comprehensive documentation and analysis of the game's code structure. This file includes:
- Complete program structure overview
- Variable definitions (string arrays, numeric arrays, global variables)
- Detailed explanation of game mechanics
- Code flow and logic documentation
- Line-by-line analysis of key routines

This is an invaluable reference for understanding how the game works internally, useful for:
- Code analysis and study
- Porting to other platforms
- Creating enhanced versions
- Learning Commodore 64 BASIC programming techniques

## Game Features

- Manage teams across 4 Italian divisions (Serie A, B, C1, C2)
- 64 teams total across all divisions
- 24 famous players available for trading
- Player attributes: style ratings (1-5), power/strength (1-20)
- Financial management system with banking options
- Match simulation with detailed statistics
- Season progression with promotion/relegation
- Team morale system
- Injury management
- League tables and standings

## How to Run

### Using an Emulator (Recommended)
1. Download a Commodore 64 emulator like [VICE](https://vice-emu.sourceforge.io/)
2. Load the `footballmanager.prg` file in the emulator
3. Type `RUN` and press Enter

### On Real Hardware
1. Transfer the `.prg` file to a C64-compatible storage device
2. Load and run following standard C64 procedures

## Technical Details

- **Platform**: Commodore 64
- **Language**: BASIC
- **Author**: Daniele Piccoli
- **Display**: Character-based graphics with PETSCII borders
- **Input**: Keyboard-based menu navigation

## Development

The game uses sophisticated techniques for a BASIC program including:
- Dynamic memory management with POKE commands
- Randomized match simulation algorithms
- Data persistence across game sessions
- Complex array structures for team/player management
- Menu-driven interface with custom graphics

## License

This is a historical Commodore 64 program preserved for educational and archival purposes.

---

*For detailed code analysis and implementation details, refer to `footballmanager_documented.txt`*
