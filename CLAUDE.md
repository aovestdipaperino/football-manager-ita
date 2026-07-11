# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This repository preserves and runs a classic Commodore 64 BASIC football (soccer) management game written by Daniele Piccoli in the 1980s. Rather than rewriting the game, the project runs the **original BASIC listings** inside `c64basic`, a C64 BASIC V2 interpreter written in Rust that renders the 40x25 PETSCII screen into a modern terminal with Unicode and ANSI colors.

## Essential Commands

```bash
# Run the original Italian game (authentic C64 speed)
cargo run --release -p c64basic -- footballmanager.txt

# Run at full native speed, or a multiple of C64 speed
cargo run --release -p c64basic -- --speed max footballmanager.txt
cargo run --release -p c64basic -- --speed 4 footballmanager.txt

# Open a TCP port that injects received bytes as keypresses (for automation)
cargo run --release -p c64basic -- --keyport 6464 footballmanager.txt
# then: printf 'G' | nc 127.0.0.1 6464

# Parse without running / run N statements headless and dump the screen
cargo run --release -p c64basic -- --parse-only footballmanager.txt
cargo run --release -p c64basic -- --headless 5000 footballmanager.txt

cargo check
cargo test
```

## Game Variants (repo root)

- `footballmanager.txt` / `footballmanager.bas` / `footballmanager.prg`: the original Italian game, kept intact. `.txt` is petcat-format source (the interpreter's input); `.prg` is the tokenized C64 binary.
- `football-manager-ita-fixed.bas`: the Italian game plus fixes for bugs in the original listing (see `POSSIBLE-BUGS.md`).
- `football-manager-intl.bas`: internationalized edition. European superleague (64 clubs, 4 divisions A-D), English text, currency scaled x1000 to modern values, modern player names, 3 points per win. Includes the same bug fixes.

## Architecture: the `c64basic` crate

- `main.rs`: CLI parsing, crossterm event loop, execution pacing, TCP key injection. Execution is throttled by a token bucket earning `C64_STMTS_PER_SEC` (600) statements per second by default so the original delay loops and animations run at authentic speed; `--speed` scales this.
- `lang.rs`: lexer and parser for petcat-format BASIC. String literals may contain `{clr}`-style control escapes and `{$xx}` hex escapes which become PETSCII bytes.
- `interp.rs`: the interpreter. `run_slice(budget)` executes up to `budget` statements then returns so the host can render and poll input. Keyboard input arrives via `push_char` (PETSCII bytes).
- `screen.rs`: 40x25 grid of PETSCII bytes plus color state, mimicking C64 screen memory. The interpreter writes PETSCII; translation to Unicode happens only at render time.
- `petscii.rs`: PETSCII byte to Unicode glyph mapping, plus the C64 palette and petcat escape-name table. **The glyph tables were verified against the actual C64 character generator ROM** (`chargen-901225-01.bin`, shipped with VICE). When changing a mapping, check the 8x8 bitmap in the ROM first; do not trust online PETSCII charts. Bytes `$60-$7F` are hardware duplicates of `$C0-$DF` and are implemented as a delegation.

## Critical Algorithm Preservation

The interpreter must run the original listing unmodified. When investigating game behavior, consult the original BASIC in `footballmanager_documented.txt` and the mappings in `CONVERSION_NOTES.md`. Key formulas (line numbers from the listing):

- Defense rating (20180): `defense + midfield/2 + energy/2 + morale/2`
- Attack rating (20190): `attack + defense_rating/2 + defense/2 + midfield/2` (original had a bug: started from 0 instead of attack; fixed only in the `-fixed`/`-intl` variants)
- Goal (20290/20310): `RND(attack_rating) - RND(opponent_defense_rating) > 0`
- Opponent stats (3220): generated from league points, `RND*(points/matches*3)+10`, capped at 20
- Match tempo (20200): `HZ = both attacks - both defenses`, min 15
- Player market value (1230): `5000*(5-league) + 5000*style` (x1000 in intl)
- Loan (1540): `debt += amount*1.2; weekly_interest = debt/20`
- Original scoring: 2 points per win (3 in the intl edition)

`POSSIBLE-BUGS.md` documents 14 defects found in the original listing. The original files must keep them; the `-fixed` and `-intl` variants fix items 1-9.

## Verification Workflow

Ground truth is the original game running in VICE. A VICE build with an embedded MCP server (`~/.local/vice-mcp/bin/x64sc -mcpserver`, MCP at `http://127.0.0.1:6510/mcp`) can autostart `footballmanager.prg`, inject keys, and take screenshots for comparison against the Rust renderer. For scripted play of the Rust side, use `--speed max --keyport <port>` and drive the game with `nc`.

## Reference Documentation

- `README.md`: original C64 game documentation
- `CONVERSION_NOTES.md`: BASIC variable/algorithm notes
- `footballmanager_documented.txt`: annotated original BASIC listing
- `POSSIBLE-BUGS.md`: defects found in the original listing
- `NOTES.md`: PETSCII/graphics research notes
- `HOW-TO-PLAY.md`: gameplay guide
