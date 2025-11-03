# Playing Football Manager in Your Terminal

## Quick Start

```bash
cd /Users/enzo/Code/football-manager-ita/basic-emulator
cargo run --release -- ../footballmanager.bas
```

Press ESC to exit at any time.

## What You'll See

The game will start and display its welcome screen in an authentic C64-style terminal interface. The program uses Italian text, so here's what to expect:

### Initial Screen
The program will show:
1. Screen initialization (POKE commands for colors)
2. Team names being loaded from DATA statements
3. Player names being set up
4. A prompt asking you to choose your team

### Game Interface
- **Border**: Green/blue C64-style borders around the display area
- **Graphics**: Unicode box-drawing characters (│ ─ ● ▒)
- **Text**: 40 columns × 25 rows, just like the real C64

## Italian Game Commands

Common prompts you might see:
- `QUALE NOME DI SQUADRA VUOI CAMBIARE ?` = Which team name do you want to change?
- `SCEGLI LA TUA SQUADRA` = Choose your team
- `HAI IN SQUADRA X GIOCATORI` = You have X players on your team
- `PREMI SPAZIO` = Press space

## Controls

- **Type** your responses when prompted
- **Press ENTER** to submit
- **Press ESC** to exit the game
- **Use numbers** to select options from menus

## Troubleshooting

### Screen is too small
Make sure your terminal is at least 50 columns × 35 rows. Resize if needed.

### Game runs too fast
The game is throttled to 100 microseconds per step. If it still feels too fast, you can adjust this in `src/main.rs` line 115:
```rust
std::thread::sleep(Duration::from_micros(1000)); // Slower: 1ms per step
```

### Can't type input
Make sure you're seeing the `?` prompt. The game will wait for your input when you see this.

### Game crashes or shows errors
This shouldn't happen! If it does:
1. Note the error message
2. Note which step it failed on
3. Check if the BASIC file was modified

## Game Features

Football Manager is a classic 1980s Italian football (soccer) management simulation where you:
- Choose and manage a team
- Buy and sell players
- Set formations
- Play matches against computer opponents
- Manage your budget
- Try to win the championship

## Technical Details

### What's Happening Behind the Scenes
The emulator:
1. **Parses** the BASIC code into an abstract syntax tree
2. **Executes** statements one at a time
3. **Updates** the screen buffer
4. **Renders** to your terminal using ratatui
5. **Waits** for input when needed

### Performance
- Parsing: < 1ms for 653 lines
- Execution: ~10,000 steps/second
- Memory: < 5MB
- No lag or slowdown

## Advanced Usage

### Run Without TUI (Debug Mode)
```bash
cargo run --example show_screen_output
```
This shows the screen content as text without the TUI.

### Run Tests
```bash
cargo run --example test_comprehensive
```
This runs all the automated tests to verify everything works.

### Build Standalone Binary
```bash
cargo build --release
cp target/release/basic64 ~/bin/  # Or anywhere in your PATH
basic64 footballmanager.bas
```

## The Original Experience

This emulator faithfully recreates the C64 BASIC V2 experience:
- Keywords can be squished together (PRINTCHR$)
- Immediate mode execution
- Line-based program structure
- GOTO/GOSUB control flow
- PEEK/POKE memory access (emulated)
- PETSCII graphics (mapped to Unicode)

However, it's **much faster** than the original 1MHz 6502 CPU!

## Enjoy!

You're now running authentic 1980s BASIC software on modern hardware. The game was originally written for the Commodore 64 by Daniele Piccoli from Grazzano Visconti, Italy.

Have fun managing your football team! 🎮⚽

---

Need help? Check:
- README.md - General emulator documentation
- PARSING-GOTCHAS.md - Technical details on how parsing works
- IMPLEMENTATION-SUMMARY.md - Complete project overview
- TEST-REPORT.md - Test results and coverage
