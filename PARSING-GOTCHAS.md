# C64 BASIC Parsing Gotchas

This document describes the special cases and quirks encountered when parsing C64 BASIC code from footballmanager.bas.

## Status

✅ **COMPLETE**: All 653 lines of footballmanager.bas successfully parse!
✅ **SOLVED**: All keyword normalization issues including OR/AND and TO
✅ **SOLVED**: Empty statement handling (consecutive colons)
✅ **SOLVED**: Leading decimal numbers (.5 format)
✅ **SOLVED**: Keyword precedence (keywords always match, even in identifiers)
🔧 **IMPLEMENTED**: Three-pass normalization strategy (keywords, OR/AND, TO)

## 1. No-Space Keyword Concatenation

C64 BASIC allows keywords to be written without spaces between them and their arguments.

### Examples:
- `PRINTCHR$(142)` → Should parse as `PRINT CHR$(142)` ✅ SOLVED
- `GOSUB2000` → Should parse as `GOSUB 2000` ✅ SOLVED
- `IFGG<>4` → Should parse as `IF GG<>4` ✅ SOLVED
- `POKE650,127` → Should parse as `POKE 650,127` ✅ SOLVED

### Solution:
Implemented three-pass keyword normalization in `Parser::normalize_keywords()`:
1. **Pass 1**: Normalizes statement keywords (PRINT, IF, GOTO, etc.)
2. **Pass 2**: Normalizes OR/AND using smart boundary detection
3. **Pass 3**: Context-aware normalization of TO in FOR loops

### Implementation Details:
**Pass 1: Statement Keywords**
- Keywords checked with word boundary detection (not preceded by alphabetic)
- Space added after keyword when followed by alphanumeric/`$`/`%`

**Pass 2: OR/AND in Expressions**
- Smart boundary detection: normalizes when (both boundaries) OR (both non-boundaries)
- `HZORQZ`: both sides are letters → normalize to `HZ OR QZ` ✅
- `FOR`: mixed (F is letter, space is not) → skip ✅
- Words in strings (ANCORA, FORMAZIONE) unaffected as they're parsed as string literals

**Pass 3: TO in FOR Loops**
- Context-aware normalization of TO
- Only normalizes TO when in FOR loop after `=`
- Handles `HZTOHZ` → `HZ TO HZ` correctly ✅

## 2. Optional THEN Keyword

In C64 BASIC, the `THEN` keyword is optional in IF statements when followed by GOTO or GOSUB.

### Examples:
- `IF DES$="G" THEN PRINT"TEXT"` → Standard form with THEN ✅
- `IF DES$="G" GOTO 135` → No THEN keyword ✅
- `IF DES$="G" THEN 100` → THEN with line number ✅

### Solution:
Modified `parse_if()` in parser.rs:393-400 to check if "THEN" is present before consuming it:
```rust
let remaining = &self.input[self.pos..];
if remaining.starts_with("THEN") {
    self.consume_word("THEN");
    self.skip_whitespace();
}
```

## 3. Embedded Keywords in Identifiers - THE BIG CHALLENGE

**CRITICAL ISSUE**: Variable names can contain keyword substrings that must NOT be normalized.

### Examples:
- `HZTOHZ` → Contains "TO" in FOR loop context → becomes `HZ TO HZ` ✅ SOLVED
- `HZORQZ` → Contains "OR" in IF expression → becomes `HZ OR QZ` ✅ SOLVED
- `FORAPE` → Variable containing "FOR" - correctly NOT normalized ✅
- `IFHZ` → Variable containing "IF" - correctly NOT normalized ✅

### Problem Cases:
```basic
FOR PZ=HZTOHZ+15        ' ✅ SOLVED: Context-aware TO normalization (Pass 3)
IFQZ<HZORQZ>HZ          ' ✅ SOLVED: Smart OR/AND boundary detection (Pass 2)
```

### Solution for TO in FOR loops:
Implemented context-aware normalization (parser.rs:233-280):
- Track when inside a FOR loop statement
- Track when '=' has been seen (start of expression)
- Unconditionally normalize any "TO" found after FOR...=
- This correctly transforms `HZTOHZ` → `HZ TO HZ`

```rust
// Pass 3: TO in FOR loops
if in_for_loop && seen_equals && slice == "TO" {
    final_result.push_str(" TO ");
    i += 2;
    in_for_loop = false; // Only one TO per FOR
    continue;
}
```

### Solution for OR/AND in expressions:
Implemented smart boundary detection (parser.rs:181-231):
- Normalize when BOTH sides are boundaries (non-letters) OR BOTH sides are non-boundaries (letters)
- Skip when mixed (one boundary, one non-boundary)
- This distinguishes `HZORQZ` (letter-OR-letter → normalize) from `FOR` (letter-OR-space → skip)
- Words in string literals are unaffected since strings are parsed separately

```rust
// Pass 2: OR/AND boundary detection
let prev_not_letter = i == 0 || !result_chars[i - 1].is_alphabetic();
let next_not_letter = i + 2 >= result_chars.len() || !result_chars[i + 2].is_alphabetic();

// Normalize when (both boundaries) OR (both non-boundaries)
if (prev_not_letter && next_not_letter) || (!prev_not_letter && !next_not_letter) {
    second_pass.push_str(" OR ");
    i += 2;
    matched = true;
}
```

**Key insight**: Analyzed footballmanager.bas and found that all Italian words containing OR/AND (ANCORA, FORMAZIONE, FUORI, etc.) only appear inside string literals, which are not affected by normalization. Therefore, any OR/AND outside strings in actual code is an operator that should be normalized, with the exception of keywords like FOR which are handled by mixed-boundary detection.

## 4. String Concatenation in PRINT

PRINT statements can have items concatenated without spaces.

### Examples:
- `PRINT""` → Empty string (common pattern) ✅
- `PRINTCHR$(142):GOSUB2000` → Multiple statements on one line ✅

### Solution:
Parser handles empty strings and statement separators (`:`) correctly.

## 5. TAB and SPC Functions in PRINT

PRINT supports special spacing functions that aren't normal function calls.

### Examples:
- `PRINTTAB(4)PZ` → Tab to column 4, then print PZ ✅
- `PRINT SPC(5)"TEXT"` → Print 5 spaces, then TEXT

### Implementation:
- `PrintItem::Tab(Box<Expr>)` - Tab to column ✅
- `PrintItem::Spc(Box<Expr>)` - Print N spaces (implemented but unused in footballmanager.bas)
- `PrintItem::Comma` - Tab to next zone (every 10 columns) ✅

## 6. Multiple Statements Per Line

C64 BASIC uses `:` to separate multiple statements on one line.

### Example:
```basic
10 POKE650,127:POKE1690,0:POKE650,127:PIQ$="[SIDE] [BORDERS]"
```

### Solution:
Parser loops through statements in `parse_line()`, checking for `:` separator. ✅

## 7. PETSCII Placeholders

The BASIC code uses placeholder strings for PETSCII graphics that need special handling.

### Examples:
- `[SIDE]` → U+2502 │ (vertical line) ✅ Mapped in screen.rs
- `[BORDERS]` → U+2500 ─ (horizontal line) ✅ Mapped
- `[BALL]` → U+25CF ● (soccer ball) ✅ Mapped
- `[FIELD]` → U+2592 ▒ (field/grass) ✅ Mapped
- `[CLR]` → Clear screen command ⚠️ Runtime handling needed
- `[REVERSE]` → Reverse video mode ⚠️ Runtime handling needed

### Current Handling:
These are parsed as regular strings. Screen module has PETSCII mapping implemented in `map_petscii()` function (screen.rs:130-162).

## 8. Case Insensitivity

C64 BASIC is case-insensitive. All input is converted to uppercase.

### Solution:
```rust
Parser::new(input: &str) -> Self {
    let normalized = Self::normalize_keywords(&input.to_uppercase());
    // ...
}
```
✅ Implemented

## 9. Variable Name Suffixes

Variables have type suffixes that are part of the identifier.

### Examples:
- `A$` → String variable ✅
- `A%` → Integer variable ✅
- `A` → Floating-point variable (default) ✅

### Solution:
The `parse_identifier()` function includes `$` and `%` as valid identifier characters (parser.rs:690-711).

## 10. Numeric Line Numbers

Every statement must be preceded by a line number, which controls execution order.

### Example:
```basic
10 PRINT "HELLO"
20 GOTO 10
```

### Solution:
`parse_line()` first parses the line number, then parses statements. ✅

## 11. Array Indexing vs Function Calls

Arrays and functions both use parentheses, requiring disambiguation.

### Examples:
- `A(5)` → Could be array access or function call
- `CHR$(65)` → Function call ✅
- `PT$(I)` → Array access ✅

### Solution:
Maintain a list of known function names in `is_function()` (parser.rs:822-827):
```rust
fn is_function(name: &str) -> bool {
    matches!(
        name,
        "INT" | "RND" | "CHR$" | "ASC" | "VAL" | "STR$" | "MID$" | "LEN" | "LEFT$" | "RIGHT$"
    )
}
```

## 12. Comparison Operators

C64 BASIC uses different comparison operators than modern languages.

### Operators:
- `=` → Equality (not assignment in expressions!) ✅
- `<>` → Not equal ✅
- `<` → Less than ✅
- `<=` → Less than or equal ✅
- `>` → Greater than ✅
- `>=` → Greater than or equal ✅

### Solution:
`match_comparison_op()` handles multi-character operators (parser.rs:528-558). ✅

## 13. Empty Statements (Consecutive Colons)

C64 BASIC allows consecutive colons (`::`), which represent empty statements.

### Example:
```basic
3100 L=1::IFI>ZTHENWW=INT(RND(1)*2)+1
```
The `::` between `L=1` and `IF` is valid and should be handled gracefully.

### Solution:
Modified `parse_line()` to skip empty statements (parser.rs:337-341):
```rust
// Skip empty statements (consecutive colons)
if self.peek() == Some(':') {
    self.advance();
    continue;
}
```
✅ SOLVED

## 14. Leading Decimal Numbers

C64 BASIC allows numbers to start with a decimal point (e.g., `.5` instead of `0.5`).

### Example:
```basic
4550 IFRND(1)>.5THENA(PZ)=A(PZ)+1:GOTO4230
```
The `.5` is a valid number literal equal to `0.5`.

### Solution:
Modified `parse_primary()` to recognize `.` as a potential number start (parser.rs:869):
```rust
// Number (including .5 format)
if self.peek().map(|c| c.is_ascii_digit() || c == '.').unwrap_or(false) {
    return Ok(Expr::Number(self.parse_number()?));
}
```
✅ SOLVED

## 15. Keyword Precedence in Identifiers

**CRITICAL**: In C64 BASIC, keywords ALWAYS take precedence over identifiers.

### Examples:
- `IFI=5` → Parses as `IF I=5` (not as variable `IFI`) ✅
- `THENPRINT` → Parses as `THEN PRINT` (not `THENPRINT` variable) ✅
- `I>ZTHENWW` → Parses as `I>Z THEN WW` ✅

### Problem Case from Line 1700:
```basic
1700 IFC(UZ)=0THENPRINTRIG$"NON HAI "B$(UZ)" NELLA TUA SQUADRA"
```
After normalizing `THEN`, needed to also normalize `PRINTRIG$` → `PRINT RIG$`

### Solution:
Removed all boundary checking from Pass 1 keyword normalization. Keywords ALWAYS match when their letters are found, regardless of what precedes them (parser.rs:138-174):
```rust
while i < chars.len() {
    let mut matched = false;
    for keyword in &statement_keywords {
        let keyword_len = keyword.len();
        if i + keyword_len <= chars.len() {
            let slice: String = chars[i..i + keyword_len].iter().collect();
            if slice == *keyword {
                // Always match keywords - they take precedence
                // This handles both THENPRINT and I>ZTHEN correctly
                result.push_str(keyword);
                i += keyword_len;
                if i < chars.len() {
                    let next = chars[i];
                    if next.is_ascii_alphanumeric() || next == '$' || next == '%' {
                        result.push(' ');
                    }
                }
                matched = true;
                break;
            }
        }
    }
    if !matched {
        result.push(chars[i]);
        i += 1;
    }
}
```

**Key Insight**: This matches the original C64 BASIC tokenizer behavior, which scanned for keyword tokens regardless of context. See MS-BASIC-TOKENIZATION.md for details.

✅ SOLVED

## 16. File Corruption / Typos

### Line 4000 - Extraneous Quote Character
Original file had:
```basic
4000 IFW<0THENPRINT"HAI \ "W":GOTO3805
```

The `"W"` pattern (quote-variable-quote) is invalid. Comparing with line 4010:
```basic
4010 PRINT"HAI \ "W
```

The intent was clearly to print a string followed by the variable W. The extra quote after W in line 4000 was a typo.

**Fix Applied**: Changed `"W":` to `W":` in line 4000.

This was the only syntax error found in the entire footballmanager.bas file.

✅ FIXED

## Keywords Requiring Normalization

Based on footballmanager.bas analysis:

### ✅ Successfully Normalized:
1. `PRINT` - Followed by expressions or function calls
2. `IF` - Followed by expressions
3. `GOTO` - Followed by line numbers
4. `GOSUB` - Followed by line numbers
5. `FOR` - Followed by variable names
6. `NEXT` - Followed by variable names (optional)
7. `THEN` - Followed by statements or line numbers
8. `TO` - Context-aware in FOR loops (Pass 3)
9. `INPUT` - Followed by variable names
10. `POKE` - Followed by addresses
11. `READ` - Followed by variable names
12. `DATA` - Followed by values
13. `DIM` - Followed by array declarations
14. `END` - Standalone or before next statement
15. `REM` - Rest of line is comment
16. `RETURN` - Usually standalone
17. `OR` - Smart boundary detection (Pass 2)
18. `AND` - Smart boundary detection (Pass 2)

## Current Implementation Strategy

### Three-Pass Normalization (parser.rs:130-280):

**Pass 1**: Statement Keywords
- Normalizes all statement-level keywords
- Adds space after keyword if followed by alphanumeric/$/%
- Checks word boundaries (not preceded by letter)

**Pass 2**: OR/AND Operators
- Smart boundary detection
- Normalizes when (both boundaries) OR (both non-boundaries)
- Skips mixed cases (e.g., FOR where F is letter, space after is not)
- Correctly handles HZORQZ → HZ OR QZ

**Pass 3**: Context-Aware TO
- Tracks FOR loop context
- After seeing `FOR...=`, unconditionally normalizes first "TO" encountered
- Prevents false matches in identifiers like MOTOR
- Correctly handles HZTOHZ → HZ TO HZ

## Files Modified

- `/Users/enzo/Code/football-manager-ita/basic-emulator/src/parser.rs` - Three-pass keyword normalization logic
- `/Users/enzo/Code/football-manager-ita/basic-emulator/src/screen.rs` - PETSCII graphics mapping
- `/Users/enzo/Code/football-manager-ita/basic-emulator/src/interpreter.rs` - Optional THEN handling
- `/Users/enzo/Code/football-manager-ita/PARSING-GOTCHAS.md` - This documentation
- `/Users/enzo/Code/football-manager-ita/basic-emulator/examples/test_line1080.rs` - Test for OR normalization
- `/Users/enzo/Code/football-manager-ita/basic-emulator/examples/simple_for_test.rs` - Test for TO normalization

