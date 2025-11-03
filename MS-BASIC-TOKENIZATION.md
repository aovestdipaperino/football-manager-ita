# MS-BASIC Tokenization on 6502 (Commodore 64)

This document reverse-engineers the tokenization system used in Microsoft BASIC for the 6502 processor, specifically focusing on the Commodore 64 implementation (REALIO=3) as found in the [microsoft/BASIC-M6502](https://github.com/microsoft/BASIC-M6502) repository.

## Table of Contents

1. [Overview](#overview)
2. [Tokenized Program Structure](#tokenized-program-structure)
3. [Token Values](#token-values)
4. [The CRUNCH Process](#the-crunch-process)
5. [Reserved Word List Structure](#reserved-word-list-structure)
6. [Detokenization (LIST)](#detokenization-list)
7. [Special Cases](#special-cases)

---

## Overview

**Tokenization** is the process of converting BASIC source code text into a compressed, executable format. When you type a BASIC program line on the Commodore 64, the interpreter:

1. Accepts the line into an input buffer (`BUF`)
2. Calls the **CRUNCH** routine to convert reserved words to single-byte tokens
3. Stores the tokenized line in program memory starting at `TXTTAB`

This compression serves two purposes:
- **Space efficiency**: Multi-character keywords become single bytes (128-255)
- **Execution speed**: The interpreter can use token values as indices for table dispatch rather than string comparison

The key insight: **A reserved word token equals 128 plus its position in the reserved word list.**

---

## Tokenized Program Structure

### Memory Organization

Programs are stored starting at `[TXTTAB]` with this overall layout:

```
[TXTTAB]  ──→  Program Text (tokenized lines)
[VARTAB]  ──→  Simple Variables (6 bytes each)
[ARYTAB]  ──→  Array Variables
[STREND]  ──→  End of arrays
              ↓ String space grows downward ↓
[FRETOP]  ──→  Top of free string space
```

### Line Format

Each program line follows this structure:

```
┌──────────────┬──────────────┬──────────────┬──────────────────────┬──────┐
│ Link Pointer │  Line Number │  Tokenized   │   More tokens/text   │  $00 │
│   (2 bytes)  │  (2 bytes)   │   Content    │                      │      │
└──────────────┴──────────────┴──────────────┴──────────────────────┴──────┘
      ↓
   Points to next line's link field
```

**Field Details:**

- **Link Pointer** (2 bytes): Address of the next line's link field (little-endian)
  - If both bytes are $00, this marks the end of the program

- **Line Number** (2 bytes): Big-endian (high byte first)
  - Range: 0 to 64,000
  - Direct mode commands use $FF,$FF (65535)

- **Tokenized Content**: Mix of:
  - Single-byte tokens (values 128-255) for reserved words
  - ASCII text for variable names, literals, etc.
  - Spaces are preserved

- **Line Terminator**: $00 (null byte)

**Example:**

The line `10 PRINT "HELLO"` would be stored as:

```
[Link] [00] [0A] [9E] [20] ["HELLO"] [00]
  ↑     ↑    ↑    ↑    ↑       ↑      ↑
  │     │    │    │    │       │      └─ Line terminator
  │     │    │    │    │       └──────── String literal
  │     │    │    │    └──────────────── Space (preserved)
  │     │    │    └───────────────────── PRINT token ($9E = 158)
  │     │    └────────────────────────── Line number 10 (big-endian)
  │     └─────────────────────────────── Line number 10 (high byte)
  └───────────────────────────────────── Pointer to next line
```

---

## Token Values

Tokens are byte values from **128 to 255** (with bit 7 set). The Commodore 64 BASIC V2 uses the following token assignments:

### Statement Tokens (128-162)

| Token | Hex  | Keyword   | Token | Hex  | Keyword  |
|-------|------|-----------|-------|------|----------|
| 128   | $80  | END       | 145   | $91  | ON       |
| 129   | $81  | FOR       | 146   | $92  | NULL     |
| 130   | $82  | NEXT      | 147   | $93  | WAIT     |
| 131   | $83  | DATA      | 148   | $94  | LOAD     |
| 132   | $84  | INPUT#    | 149   | $95  | SAVE     |
| 133   | $85  | INPUT     | 150   | $96  | VERIFY   |
| 134   | $86  | DIM       | 151   | $97  | DEF      |
| 135   | $87  | READ      | 152   | $98  | POKE     |
| 136   | $88  | LET       | 153   | $99  | PRINT#   |
| 137   | $89  | GOTO      | 154   | $9A  | PRINT    |
| 138   | $8A  | RUN       | 155   | $9B  | CONT     |
| 139   | $8B  | IF        | 156   | $9C  | LIST     |
| 140   | $8C  | RESTORE   | 157   | $9D  | CLR      |
| 141   | $8D  | GOSUB     | 158   | $9E  | CMD      |
| 142   | $8E  | RETURN    | 159   | $9F  | SYS      |
| 143   | $8F  | REM       | 160   | $A0  | OPEN     |
| 144   | $90  | STOP      | 161   | $A1  | CLOSE    |
|       |      |           | 162   | $A2  | GET      |
|       |      |           | 163   | $A3  | NEW      |

### Function Tokens (164-202)

Functions start at token 164 (128 + ONEFUN offset):

| Token | Hex  | Keyword   | Token | Hex  | Keyword  |
|-------|------|-----------|-------|------|----------|
| 164   | $A4  | TAB(      | 183   | $B7  | LEFT$    |
| 165   | $A5  | TO        | 184   | $B8  | RIGHT$   |
| 166   | $A6  | FN        | 185   | $B9  | MID$     |
| 167   | $A7  | SPC(      | 186   | $BA  | GO       |
| 168   | $A8  | THEN      |       |      |          |
| 169   | $A9  | NOT       |       |      |          |
| 170   | $AA  | STEP      |       |      |          |
| 171   | $AB  | +         |       |      |          |
| 172   | $AC  | -         |       |      |          |
| 173   | $AD  | *         |       |      |          |
| 174   | $AE  | /         |       |      |          |
| 175   | $AF  | ^         |       |      |          |
| 176   | $B0  | AND       |       |      |          |
| 177   | $B1  | OR        |       |      |          |
| 178   | $B2  | >         |       |      |          |
| 179   | $B3  | =         |       |      |          |
| 180   | $B4  | <         |       |      |          |
| 181   | $B5  | SGN       |       |      |          |
| 182   | $B6  | INT       |       |      |          |

### Additional Function Tokens (181-202)

| Token | Hex  | Keyword   | Token | Hex  | Keyword  |
|-------|------|-----------|-------|------|----------|
| 181   | $B5  | SGN       | 194   | $C2  | PEEK     |
| 182   | $B6  | INT       | 195   | $C3  | LEN      |
| 183   | $B7  | ABS       | 196   | $C4  | STR$     |
| 184   | $B8  | USR       | 197   | $C5  | VAL      |
| 185   | $B9  | FRE       | 198   | $C6  | ASC      |
| 186   | $BA  | POS       | 199   | $C7  | CHR$     |
| 187   | $BB  | SQR       | 200   | $C8  | LEFT$    |
| 188   | $BC  | RND       | 201   | $C9  | RIGHT$   |
| 189   | $BD  | LOG       | 202   | $CA  | MID$     |
| 190   | $BE  | EXP       |       |      |          |
| 191   | $BF  | COS       |       |      |          |
| 192   | $C0  | SIN       |       |      |          |
| 193   | $C1  | TAN       |       |      |          |
| 194   | $C2  | ATN       |       |      |          |

### Identifying Tokens

To determine if a byte is a token:

```
IF (byte >= 128) AND (not inside quoted string) THEN
    byte is a token
ELSE
    byte is ASCII text
END IF
```

---

## The CRUNCH Process

The **CRUNCH** routine is called after a complete line is entered. It converts the ASCII text in the input buffer (`BUF`) into tokenized form.

### Algorithm Overview

```
1. Initialize source pointer to BUF
2. Set destination offset to 4 (skip link and line number)
3. Enable crunching mode
4. FOR each character in input buffer:
   a. Skip leading/trailing spaces (but preserve them)
   b. Check if inside quoted string (disable crunching)
   c. Check if inside DATA statement (disable crunching)
   d. Check for '?' → replace with PRINT token
   e. Try to match reserved words
   f. Copy character to output
5. Add null terminator
6. Link the program line
```

### Pseudo-code

```c
void CRUNCH() {
    TXTPTR = BUF;              // Source pointer
    dest_offset = 4;           // Skip link/line number in destination
    crunch_enabled = true;
    in_quote = false;
    in_data = false;

    while (*TXTPTR != 0) {
        char ch = *TXTPTR;

        // Handle quotes
        if (ch == '"') {
            in_quote = !in_quote;
            copy_char(ch);
            continue;
        }

        // Skip crunching inside quotes or DATA
        if (in_quote || in_data) {
            copy_char(ch);
            continue;
        }

        // Special case: '?' becomes PRINT
        if (ch == '?') {
            output_token(PRINT_TOKEN);  // 154
            TXTPTR++;
            continue;
        }

        // Try to match reserved word
        token = match_reserved_word(TXTPTR);
        if (token != 0) {
            output_token(token);
            TXTPTR += keyword_length;

            // Special: if we just tokenized DATA, disable crunching
            if (token == DATA_TOKEN) {
                in_data = true;
            }
            continue;
        }

        // No match - copy literal character
        copy_char(ch);
        TXTPTR++;
    }

    output_byte(0);  // Null terminator
}
```

### Reserved Word Matching

The `match_reserved_word()` function searches the `RESLST` table:

```c
int match_reserved_word(char *input) {
    int token = 128;  // First token value
    char *reslst_ptr = RESLST;

    while (true) {
        char *inp = input;
        char *res = reslst_ptr;

        // Compare characters
        while (true) {
            char res_ch = *res;
            char inp_ch = *inp;

            // Check if end of reserved word (high bit set)
            if (res_ch & 0x80) {
                // Last character of reserved word
                res_ch &= 0x7F;  // Clear high bit

                if (toupper(inp_ch) == res_ch) {
                    // Check that next input char is not alphanumeric
                    if (!isalnum(inp[1])) {
                        return token;  // Match found!
                    }
                }
                break;  // Not a match, try next word
            }

            if (toupper(inp_ch) != res_ch) {
                break;  // Mismatch
            }

            inp++;
            res++;
        }

        // Advance to next reserved word
        while (!(*reslst_ptr & 0x80)) {
            reslst_ptr++;
        }
        reslst_ptr++;  // Skip the high-bit terminator

        token++;

        if (token > MAX_TOKEN) {
            return 0;  // No match
        }
    }
}
```

### Key Implementation Notes

1. **Matching is case-insensitive**: `toupper()` is applied during comparison
2. **Whole-word matching**: After matching the last character, the algorithm checks that the next input character is not alphanumeric (prevents "PRINT" matching in "PRINTER")
3. **Sequential search**: Reserved words are searched in order from RESLST
4. **Longer words first**: The reserved word list must place longer keywords before shorter ones that are substrings (e.g., "INPUT#" before "INPUT")

---

## Reserved Word List Structure

The reserved word list (`RESLST`) is stored as a compact byte sequence where each word is terminated by a character with bit 7 set.

### Storage Format

Each reserved word is defined using the `DCI` (Define Crunched Item) macro:

```assembly
RESLST: DCI "END"      ; Stores: 'E', 'N', 'D'|$80
        DCI "FOR"      ; Stores: 'F', 'O', 'R'|$80
        DCI "NEXT"     ; Stores: 'N', 'E', 'X', 'T'|$80
        DCI "DATA"     ; Stores: 'D', 'A', 'T', 'A'|$80
        ; ... etc
```

### Encoding Rules

- **Characters**: Plain ASCII (uppercase)
- **Last character**: Has bit 7 set (OR with $80)
- **No delimiters**: Words are stored back-to-back
- **Token value**: Position in list + 128

### Example: How "FOR" is Stored

```
Memory:     'E'  'N'  'D'|$80  'F'  'O'  'R'|$80  'N'  'E'  'X'  'T'|$80
Hex:        $45  $4E  $C4     $46  $4F  $D2     $4E  $45  $58  $D4
Token:      128 (END)         129 (FOR)         130 (NEXT)
            ←─────────────────┴──────────────────┴────────────────→
                        Sequential storage
```

### Searching the List

To find token 130 (NEXT) during detokenization:

```c
char *find_reserved_word(int token) {
    char *ptr = RESLST;
    int current_token = 128;

    while (current_token < token) {
        // Skip to next word
        while (!(*ptr & 0x80)) {
            ptr++;
        }
        ptr++;  // Skip high-bit terminator
        current_token++;
    }

    return ptr;  // Points to first char of target word
}
```

### Ordering Constraints

Reserved words **must be ordered carefully** to prevent incorrect partial matches:

**Problem Example:**
```
If "FOR" appears before "FORMAT", then "FORMAT" would incorrectly match "FOR"
```

**Solution:**
- Place longer keywords before shorter substrings
- The original C64 BASIC list is carefully ordered:
  - "INPUT#" before "INPUT"
  - "PRINT#" before "PRINT"
  - Multi-character operators before single-char

---

## Detokenization (LIST)

When you type `LIST`, the interpreter must convert tokens back to readable text. This happens in the **PRIT3** routine.

### Algorithm

```c
void detokenize_line() {
    char *line_ptr = current_line + 4;  // Skip link and line number

    while (*line_ptr != 0) {
        char ch = *line_ptr;

        if (ch >= 128) {
            // This is a token - look it up
            print_reserved_word(ch);
        } else {
            // Regular ASCII character
            output_char(ch);
        }

        line_ptr++;
    }
}

void print_reserved_word(int token) {
    int index = token - 128;
    char *reslst_ptr = RESLST;

    // Skip to the target word
    for (int i = 0; i < index; i++) {
        while (!(*reslst_ptr & 0x80)) {
            reslst_ptr++;
        }
        reslst_ptr++;
    }

    // Print characters until high bit found
    while (true) {
        char ch = *reslst_ptr;

        if (ch & 0x80) {
            // Last character
            output_char(ch & 0x7F);  // Clear high bit and print
            break;
        }

        output_char(ch);
        reslst_ptr++;
    }

    output_char(' ');  // Space after keyword
}
```

### Assembly Implementation (from source)

```assembly
PRIT3:  INY
        LDA     RESLST,Y        ; Load character from reserved word list
        BMI     PRIT4           ; High bit set? End of word
        JSR     OUTDO           ; Print character
        BNE     PRIT3           ; Continue until end marker
PRIT4:  AND     #$7F            ; Clear high bit
        JSR     OUTDO           ; Print final character
        ; Continue with rest of line...
```

---

## Special Cases

### 1. The '?' Shortcut

The question mark `?` is a shorthand for `PRINT`. During tokenization:

```assembly
KLOOP1: CMPI "?"                ; Is character '?'?
        BNE  KLOOP2             ; No, continue
        LDAI PRINTK             ; Yes, load PRINT token (154)
        BNE  STUFFH             ; Store it
```

**Example:**
```basic
? "HELLO"     →  tokenizes to:  [154] [20] ["HELLO"] [00]
PRINT "HELLO" →  tokenizes to:  [154] [20] ["HELLO"] [00]
```

Both produce identical tokenized output.

### 2. DATA Statement Handling

After the `DATA` token is encountered, crunching is **disabled** for the remainder of the line:

```basic
10 DATA FOR, NEXT, PRINT
```

Tokenizes to:
```
[Link] [00] [0A] [131] [20] F O R , 20 N E X T , 20 P R I N T [00]
         ↑           ↑     ←──────────────────────────────────→
      Line 10      DATA            Not tokenized!
```

This prevents reserved words inside DATA from being converted to tokens.

**Implementation Flag:**
- `DORES`: Set to 0 after DATA token, preventing further crunching until line end

### 3. REM Statement Handling

Comments (REM statements) also disable tokenization:

```basic
10 REM THIS IS A COMMENT: PRINT SHOULD NOT TOKENIZE
```

Additionally, `REM` changes the line termination logic:
- Normal statements can be separated by `:` (colon)
- After `REM`, only the null byte terminates the line
- This allows colons to appear in comments without being interpreted as statement separators

### 4. Quoted String Handling

Strings enclosed in quotes are never tokenized:

```basic
10 PRINT "TYPE PRINT TO CONTINUE"
```

Tokenizes to:
```
[Link] [00] [0A] [154] [20] ["] T Y P E  P R I N T ... ["] [00]
                  ↑           ←─────────────────────────→
                PRINT              Not tokenized!
```

**Implementation:**
- `ENDCHR`: Stores the quote character (`"`)
- Crunching disabled while `ENDCHR != 0`
- Cleared when matching closing quote found

### 5. Spaces Are Preserved

Unlike some modern tokenizers, BASIC preserves spaces:

```basic
10 PRINT    "HELLO"
```

The multiple spaces between `PRINT` and the string are stored as-is in the tokenized form. This maintains formatting when the program is listed.

### 6. Multi-Character Operators

Some tokens represent multi-character sequences:

- `INPUT#` (token 132)
- `PRINT#` (token 153)
- `GO TO` can be written as `GOTO` (both → token 137)

The reserved word list must order these carefully:
```assembly
DCI "INPUT#"    ; Token 132 - must come before INPUT
DCI "INPUT"     ; Token 133
DCI "PRINT#"    ; Token 153 - must come before PRINT
DCI "PRINT"     ; Token 154
```

### 7. Variable Names Cannot Match Keywords

The whole-word matching prevents keywords from being tokenized when they're part of variable names:

```basic
10 PRINT1 = 5
```

Here, `PRINT1` is recognized as a variable name (not `PRINT` + `1`) because the character after `PRINT` is alphanumeric.

### 8. Line Number Range

Line numbers are stored as 16-bit values (0-65535), but BASIC enforces a range of **0 to 64,000**:

```assembly
LINGET: ; Read line number from input
        ; ... parsing code ...
        CMP #250        ; High byte > 250?
        BCS ERROR       ; Yes, throw error (> 64,000)
```

Special line number `65535` ($FFFF) indicates **direct mode** (immediate execution without line number).

---

## Summary

The MS-BASIC tokenization system on the 6502 is an elegant solution to the constraints of 1970s-80s microcomputers:

1. **Compression**: Multi-byte keywords → single bytes (saves precious RAM)
2. **Speed**: Token-based dispatch faster than string comparison
3. **Simplicity**: Linear search through ordered reserved word list
4. **Reversibility**: Detokenization via same table lookup

The system demonstrates several clever design choices:
- High-bit markers eliminate need for length bytes or delimiters
- Position-based token values enable O(1) dispatch tables
- Careful keyword ordering prevents ambiguous matches
- Special handling for quotes, DATA, and REM preserves literal text

This tokenization approach became the standard for 8-bit BASIC interpreters and influenced language design for decades.

---

## References

- [microsoft/BASIC-M6502](https://github.com/microsoft/BASIC-M6502) - Original Microsoft 6502 BASIC source code
- Configuration: `REALIO=3` (Commodore platform)
- Key routines: `CRUNCH`, `PRIT3`, `LNKPRG`
- Key data structures: `RESLST`, `BUF`, `TXTTAB`
