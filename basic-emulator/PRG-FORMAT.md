# PRG File Format Support

The BASIC emulator now supports loading tokenized Commodore 64 PRG files directly using the `--prg` flag.

## What is a PRG File?

A PRG (Program) file is the standard binary format for Commodore 64 BASIC programs. Unlike plain text `.bas` files, PRG files contain:

1. **2-byte load address** (typically `$0801` / 2049)
2. **Tokenized BASIC program** where keywords are represented as single bytes (tokens $80-$CA)
3. **Linked list structure** where each line contains a pointer to the next line

## Usage

```bash
# Load and run a PRG file
cargo run --release -- --prg footballmanager.prg

# Build and use the binary directly
cargo build --release
./target/release/basic64 --prg /path/to/program.prg

# Detokenize a PRG file to plain text (without running)
cargo run --example test_prg_detokenizer --release -- program.prg
```

## How It Works

### 1. PRG File Structure

Each PRG file follows this memory layout:

```
Offset  Size  Description
------  ----  -----------
0       2     Load address (little-endian, e.g., $01 $08 for $0801)
2       *     Tokenized BASIC program (linked list of lines)
```

### 2. BASIC Line Format

Each line in the tokenized program follows this structure:

```
Offset  Size  Description
------  ----  -----------
0       2     Link pointer to next line (little-endian, $00 $00 = end)
2       2     Line number (little-endian)
4       *     Tokenized content (mix of tokens and ASCII)
*       1     Line terminator ($00)
```

### 3. Token Table

Commodore 64 BASIC V2 uses tokens $80-$CA (128-202) to represent reserved words:

| Token | Hex  | Keyword | Token | Hex  | Keyword |
|-------|------|---------|-------|------|---------|
| 128   | $80  | END     | 153   | $99  | PRINT   |
| 129   | $81  | FOR     | 164   | $A4  | TO      |
| 130   | $82  | NEXT    | 167   | $A7  | THEN    |
| 131   | $83  | DATA    | 170   | $AA  | +       |
| 139   | $8B  | IF      | 175   | $AF  | AND     |
| 141   | $8D  | GOSUB   | 176   | $B0  | OR      |
| 142   | $8E  | RETURN  | 178   | $B2  | =       |

See [MS-BASIC-TOKENIZATION.md](../MS-BASIC-TOKENIZATION.md) for the complete token table.

### 4. Detokenization Process

The detokenizer performs the following steps:

1. **Load PRG file** and skip the 2-byte load address
2. **Parse linked list** of BASIC lines
3. **Convert tokens** back to keywords using the token table
4. **Handle special cases**:
   - Quoted strings: Tokens inside `"..."` are treated as literal PETSCII bytes
   - REM statements: Everything after `REM` is treated as literal text
   - Smart spacing: Add spaces around keywords when needed for parser compatibility
5. **Output plain text** BASIC source code

### 5. Smart Spacing

The detokenizer adds intelligent spacing to make the output compatible with BASIC parsers:

- **Space before keyword** when preceded by alphanumeric character
  - Example: `HZOR` → `HZ OR`

- **Space after keyword** when followed by alphanumeric character
  - Example: `IFA$` → `IF A$`

Keywords that receive smart spacing:
`IF`, `FOR`, `THEN`, `TO`, `STEP`, `AND`, `OR`, `NOT`, `GOTO`, `GOSUB`, `ON`, `DIM`, `READ`, `INPUT`, `LET`, `PRINT`, `GET`, `NEXT`, `DATA`, `RETURN`, `END`

## Implementation Details

### Module: `src/prg_loader.rs`

#### Functions

**`load_prg_file(path: P) -> io::Result<Vec<u8>>`**
- Loads a PRG file and returns the raw bytes after the load address
- Prints the load address to stderr for debugging

**`detokenize_program(bytes: &[u8]) -> Result<String, String>`**
- Converts tokenized bytes to plain text BASIC
- Returns detokenized source code or error message

**`petscii_to_ascii(petscii: u8) -> char`**
- Converts PETSCII characters to ASCII
- Unknown characters become `?`

### Examples

**`examples/test_prg_detokenizer.rs`**
- Standalone tool to detokenize PRG files without running them
- Useful for inspecting PRG file contents
- Usage: `cargo run --example test_prg_detokenizer -- file.prg`

## Testing

Run the test suite:

```bash
# Unit tests
cargo test --lib

# Test detokenizer with real PRG file
cargo run --example test_prg_detokenizer --release -- footballmanager.prg

# Test parsing detokenized output
cargo run --example find_parse_error_detokenized --release -- detokenized.bas
```

## Limitations

1. **PETSCII Graphics**: Non-ASCII PETSCII characters (graphic characters) are converted to `?`
2. **Parser Compatibility**: Some valid C64 BASIC constructs may not parse correctly if the BASIC parser has limitations
3. **C64 BASIC V2 Only**: Currently supports only Commodore 64 BASIC V2 token set (not BASIC 3.5, 7.0, etc.)

## Example Output

Input PRG bytes:
```
01 08 09 08 01 00 89 20 36 00 ...
```

Detokenized output:
```
1 GOTO 6
5 XZ=PZ:GOTO 1690
6 POKE 650,127:PIQ$="? ..."
```

## References

- [MS-BASIC-TOKENIZATION.md](../MS-BASIC-TOKENIZATION.md) - Detailed tokenization documentation
- [Commodore 64 BASIC V2 ROM](https://github.com/microsoft/BASIC-M6502) - Original source code
- [RUNNING.md](../RUNNING.md) - User guide for running the emulator

## Troubleshooting

### "Unknown token $XX" error
- The PRG file may contain extended tokens not in C64 BASIC V2
- The file may be corrupted
- Try detokenizing with: `cargo run --example test_prg_detokenizer -- file.prg`

### Parser errors when loading PRG
- Some C64 BASIC constructs may not be supported by the parser
- Try detokenizing first to see the source: `cargo run --example test_prg_detokenizer -- file.prg > output.bas`
- Compare with original `.bas` file if available

### "Device not configured" error
- This is a terminal setup issue when running in non-interactive environments
- Use the detokenizer example instead: `cargo run --example test_prg_detokenizer`
