# C64 BASIC Emulator

A complete Commodore 64 BASIC interpreter written in Rust, capable of running authentic 1980s BASIC programs.

## Features

- **Full C64 BASIC V2 Support**: All major keywords and functions
- **PRG File Support**: Load tokenized Commodore 64 PRG files directly with `--prg` flag
- **Accurate Parsing**: Handles no-space keyword concatenation like the original tokenizer
- **TUI Interface**: Beautiful terminal UI using ratatui
- **PETSCII Graphics**: Unicode mapping for C64 graphics characters
- **Interactive**: Full INPUT support with keyboard handling
- **Fast**: Optimized execution with configurable throttling

## Supported BASIC Features

### Statements
- PRINT (with TAB, SPC, zones, semicolons)
- INPUT (with prompts)
- LET (variable and array assignment)
- IF-THEN (with GOTO or statement blocks)
- GOTO, GOSUB, RETURN
- FOR-NEXT (with STEP)
- DIM (multi-dimensional arrays)
- DATA, READ
- POKE (screen colors)
- END, REM

### Functions
- **Math**: INT(), RND()
- **String**: CHR$(), ASC(), VAL(), STR$(), MID$(), LEN(), LEFT$(), RIGHT$()

### Operators
- Arithmetic: +, -, *, /, ^
- Comparison: =, <>, <, <=, >, >=
- Logical: AND, OR, NOT

## Building

```bash
cargo build --release
```

## Running

```bash
# Run a plain text BASIC program
cargo run --release -- program.bas

# Run a tokenized PRG file
cargo run --release -- --prg program.prg

# Or use the compiled binary
./target/release/basic64 program.bas
./target/release/basic64 --prg program.prg
```

## Controls

- Type normally when prompted for INPUT
- Press ENTER to submit input
- Press ESC to exit the program

## PRG File Format

The emulator supports loading tokenized Commodore 64 PRG files. See [PRG-FORMAT.md](PRG-FORMAT.md) for detailed documentation on:
- PRG file structure and tokenization
- How the detokenizer works
- Smart spacing algorithm
- Usage examples and troubleshooting

## Example Programs

### Hello World
```basic
10 PRINT "HELLO WORLD"
20 END
```

### FOR Loop
```basic
10 FOR I=1 TO 10
20 PRINT "I=";I
30 NEXT I
40 END
```

### Subroutines
```basic
10 GOSUB 100
20 PRINT "BACK IN MAIN"
30 END
100 PRINT "IN SUBROUTINE"
110 RETURN
```

## Testing

Run the included tests:

```bash
# Test parser
cargo run --example test_footballmanager

# Test interpreter
cargo run --example test_interpreter

# Test runtime execution
cargo run --example test_footballmanager_runtime
```

## Implementation Notes

### Keyword Normalization

The parser uses a three-pass normalization strategy to handle C64 BASIC's no-space syntax:

1. **Pass 1**: Statement keywords (PRINT, IF, FOR, etc.)
2. **Pass 2**: OR/AND operators with boundary detection
3. **Pass 3**: Context-aware TO in FOR loops

See [PARSING-GOTCHAS.md](../PARSING-GOTCHAS.md) for details.

### Execution Model

- Single-threaded step-by-step execution
- Configurable throttling (default: 100μs per step)
- Shared screen buffer using Arc<Mutex<>>
- Variable storage in HashMap
- Proper GOSUB and FOR loop stacks

## Project Structure

```
src/
├── main.rs         - TUI application entry point
├── lib.rs          - Public API exports
├── parser.rs       - Lexer and parser (three-pass normalization)
├── interpreter.rs  - Runtime execution engine
├── screen.rs       - TUI screen emulation
└── value.rs        - Value type system (Number/String)

examples/
├── test_*.rs       - Parsing test cases
└── test_*_runtime.rs - Runtime test cases
```

## Documentation

- [PARSING-GOTCHAS.md](../PARSING-GOTCHAS.md) - Detailed parsing challenges and solutions
- [IMPLEMENTATION-SUMMARY.md](../IMPLEMENTATION-SUMMARY.md) - Complete project overview
- [MS-BASIC-TOKENIZATION.md](../MS-BASIC-TOKENIZATION.md) - Original tokenizer reference

## License

See LICENSE file in the repository root.

## Acknowledgments

- Original C64 BASIC by Microsoft/Commodore
- footballmanager.bas - Authentic 1980s BASIC program used for testing
- ratatui - Excellent TUI library
- crossterm - Terminal manipulation
