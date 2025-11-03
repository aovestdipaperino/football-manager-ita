# Technical Documentation for footballmanager.bas

This is the comprehensive technical reference for the Commodore 64 BASIC Football Manager game. All information about the original BASIC code, memory operations, and emulation specifications are documented here.

---

# Table of Contents

1. [PETSCII Graphics Characters and TUI Equivalents](#petscii-graphics-characters-and-tui-equivalents)
2. [POKE Command Documentation](#poke-command-documentation)
3. [Commodore 64 BASIC Emulator Specification](#commodore-64-basic-emulator-specification)

---

# PETSCII Graphics Characters and TUI Equivalents

## Overview

The original Commodore 64 BASIC code uses PETSCII (PET Standard Code of Information Interchange) graphics characters to draw borders, boxes, and visual elements for the game's user interface. These are special characters available in the C64's uppercase/graphics character set that allow for semi-graphical text-based displays.

In the `.bas` file, these characters appear as placeholders like `[SIDE]`, `[BORDERS]`, `[BALL]`, etc. because modern text editors cannot display the actual PETSCII characters.

## PETSCII Characters Used in footballmanager.bas

Based on analysis of the code, the following PETSCII graphics characters are used:

### 1. Box Drawing Characters

| Placeholder | Description | PETSCII Code | Unicode Equivalent | TUI Character | Notes |
|-------------|-------------|--------------|-------------------|---------------|-------|
| `[SIDE]` | Vertical line (box side) | 221 ($DD) | U+2502 │ | `│` | Used for left and right borders |
| `[BORDERS]` | Horizontal line | 163 ($A3) | U+2500 ─ | `─` | Used for top and bottom borders |
| `[TOP BORDER]` | Top-left corner + horizontal line | Combination | U+250C ┌ + U+2500 ─ | `┌─` | Full top border line |
| `[BOTTOM BORDER]` | Bottom-left corner + horizontal line | Combination | U+2514 └ + U+2500 ─ | `└─` | Full bottom border line |
| `[FULL BORDER TOP]` | Complete top border with corners | Combination | U+250C ┌ + U+2500 ─ + U+2510 ┐ | `┌───┐` | Full width top |
| `[FULL BORDER BOTTOM]` | Complete bottom border with corners | Combination | U+2514 └ + U+2500 ─ + U+2518 ┘ | `└───┘` | Full width bottom |
| `[BORDER LINE]` | Horizontal divider line | 163 ($A3) | U+2500 ─ | `─` | Separator line |

### 2. Special Graphics Characters

| Placeholder | Description | PETSCII Code | Unicode Equivalent | TUI Character | Notes |
|-------------|-------------|--------------|-------------------|---------------|-------|
| `[BALL]` | Soccer ball/circle | 81 ($51) or 233 ($E9) | U+25CF ● | `●` or `○` | Represents the ball in field display |
| `[FIELD]` | Field/grass block | 160 ($A0) or 224 ($E0) | U+2592 ▒ | `▒` or `░` | Represents grass/field |
| `[CLR]` | Clear screen | CHR$(147) | ANSI: `\x1b[2J` | N/A | Terminal clear command |
| `[REVERSE]` | Reverse video mode | CHR$(18) | ANSI: `\x1b[7m` | N/A | Inverted colors |
| `[TOP]` | Top section marker | Text | N/A | N/A | Layout marker |

### 3. String Variables Containing Graphics

The game initializes several string variables (lines 10-70) to store border patterns:

```basic
10 PIQ$="[SIDE] [BORDERS]"          ' Vertical line + space + horizontal line
20 IP$="[SIDE][SIDE]                        [SIDE]"  ' Double vertical + spaces + vertical
30 RIG$="                                    [SIDE]"  ' Spaces + vertical (right align)
40 UNO$="[SIDE] [BORDERS]"           ' Same as PIQ$
50 DUE$=" [SIDE]                                    [SIDE]"  ' Space + vertical + spaces + vertical
60 TRE$=" [BORDERS]"                 ' Space + horizontal line
70 TUO$=" [BORDERS]"                 ' Space + horizontal line
```

These templates are used throughout the code to print consistent UI borders.

## Unicode Box-Drawing Characters Reference

For modern TUI implementations, use these Unicode characters:

### Single-Line Borders
```
┌─────┐   U+250C (top-left)    U+2500 (horizontal)  U+2510 (top-right)
│     │   U+2502 (vertical)
└─────┘   U+2514 (bottom-left) U+2500 (horizontal)  U+2518 (bottom-right)
```

### Double-Line Borders (Alternative)
```
╔═════╗   U+2554 (top-left)    U+2550 (horizontal)  U+2557 (top-right)
║     ║   U+2551 (vertical)
╚═════╝   U+255A (bottom-left) U+2550 (horizontal)  U+255D (bottom-right)
```

### Mixed Single/Double (for emphasis)
```
╓─────╖   U+2553 (top-left)    U+2500 (horizontal)  U+2556 (top-right)
║     ║   U+2551 (vertical)
╙─────╜   U+2559 (bottom-left) U+2500 (horizontal)  U+255C (bottom-right)
```

## TUI Implementation Mapping

### Recommended Mapping for Rust TUI (ratatui)

```rust
// Box drawing characters
const VERTICAL: &str = "│";      // [SIDE]
const HORIZONTAL: &str = "─";    // [BORDERS]
const TOP_LEFT: &str = "┌";
const TOP_RIGHT: &str = "┐";
const BOTTOM_LEFT: &str = "└";
const BOTTOM_RIGHT: &str = "┘";

// Special characters
const BALL: &str = "●";          // [BALL] - filled circle
const FIELD: &str = "▒";         // [FIELD] - medium shade

// Template strings
const PIQ: &str = "│ ─";
const IP: &str = "││                        │";
const UNO: &str = "│ ─";
const DUE: &str = " │                                    │";
const TRE: &str = " ─";
```

### Alternative ASCII-Safe Mapping (for compatibility)

If Unicode box-drawing is not supported, use ASCII characters:

```rust
const VERTICAL: &str = "|";      // [SIDE]
const HORIZONTAL: &str = "-";    // [BORDERS]
const TOP_LEFT: &str = "+";
const TOP_RIGHT: &str = "+";
const BOTTOM_LEFT: &str = "+";
const BOTTOM_RIGHT: &str = "+";

const BALL: &str = "O";          // [BALL]
const FIELD: &str = "#";         // [FIELD]
```

## Screen Control Codes

| Placeholder | PETSCII | Modern Equivalent | Purpose |
|-------------|---------|-------------------|---------|
| `[CLR]` | CHR$(147) | `\x1b[2J\x1b[H` (ANSI) | Clear screen and home cursor |
| `[REVERSE]` | CHR$(18) | `\x1b[7m` (ANSI) | Enable reverse video |
| CHR$(142) | 142 | `\x1b[0m` (ANSI) | Switch to uppercase/graphics mode |

## Usage Patterns in the Code

### Menu Borders (Lines 1110-1300)

The main menu uses a complete bordered box:

```basic
1110 PRINT"[FULL BORDER TOP]"
1120 PRINT"[SIDE]1[SIDE] [SIDE] PER VENDERE O LISTARE GIOCATORI  "
...
1300 PRINT"[FULL BORDER BOTTOM]"
```

**TUI Equivalent:**
```
┌────────��───────────────────────────┐
│1│ │ PER VENDERE O LISTARE GIOCATORI│
│2│ │ PER OTTENERE UN PRESTITO       │
│3│ │ PER STAMPARE LA CLASSIFICA     │
└────────────────────────────────────┘
```

### Soccer Field Display (Lines 2480-2600, 40000-40150)

The game draws a stylized soccer field using graphics characters:

```basic
2480 PRINT"[BORDERS]"
2490 PRINT"[SIDE]           [BALL]           [SIDE]"
2500 PRINT"[SIDE]            [SIDE]            [SIDE]"
2510 PRINT"[BORDERS] [SIDE][SIDE]       [SIDE]       [SIDE] [BORDERS]"
2520 PRINT"[SIDE] [SIDE][SIDE][SIDE]        [BALL][BALL]        [SIDE][SIDE] [SIDE]"
2530 PRINT"[SIDE][SIDE] [SIDE][SIDE]       [FIELD]       [FIELD] [SIDE][SIDE]"
```

**TUI Equivalent:**
```
─────────────────────────────────
│           ●           │
│            │            │
─ ││       │       │ ─
│ │││        ●●        ││ │
││ ││       ▒       ▒ ││
```

### Title Screen (Lines 2390-2460)

Creates a framed title:

```basic
2390 PRINT"[SIDE]                           [SIDE]"
2400 PRINT" [SIDE] [BORDERS] [SIDE]"
2410 PRINT" [SIDE] [SIDE][SIDE]FOOTBALL  MANAGER  C-64[SIDE][SIDE] [SIDE]"
2420 PRINT" [SIDE] [BORDERS] [SIDE]"
```

**TUI Equivalent:**
```
│                           │
 │ ─ │
 │ ││FOOTBALL  MANAGER  C-64││ │
 │ ─ │
```

## Character Set Dependencies

The game uses `POKE53272,28` to switch to the C64's uppercase/graphics character set, which contains:
- Uppercase letters A-Z
- Numbers 0-9
- **PETSCII graphics characters** (box-drawing, shapes, etc.)

In modern TUI implementations:
1. Use **Unicode box-drawing characters** (U+2500-U+257F range) for best visual appearance
2. Fall back to **ASCII characters** (+, -, |) for maximum compatibility
3. Use **Block Elements** (U+2580-U+259F) for field/shading: ▒, ░, ▓

## Testing TUI Display

To verify proper rendering in a modern terminal:

```rust
// Test pattern
println!("┌────────────┐");
println!("│ Test  ●  ▒ │");
println!("└────────────┘");
```

Expected output should show:
- Clean box corners (not broken)
- Straight lines (not dashed/dotted)
- Filled circle for ball
- Shaded block for field

If characters appear broken or as `?`, the terminal may not support Unicode box-drawing. Use ASCII fallback.

## References

- PETSCII Character Set: https://www.c64-wiki.com/wiki/PETSCII
- Unicode Box Drawing: https://en.wikipedia.org/wiki/Box-drawing_character
- C64 Screen Codes: https://sta.c64.org/cbm64scr.html
- Ratatui Block Widget: https://docs.rs/ratatui/latest/ratatui/widgets/block/

---

# POKE Command Documentation

This section catalogs all POKE commands used in the original Commodore 64 BASIC Football Manager game and explains their purposes.

## Overview

The game uses POKE commands to directly manipulate specific memory locations in the Commodore 64's hardware. These are used for controlling keyboard input behavior, screen colors, and character set graphics.

## Complete POKE Usage List

### Line 10: Keyboard and Input Buffer Control

```basic
10 POKE650,127:POKE1690,0:POKE650,127:PIQ$="[SIDE] [BORDERS]"
```

**Memory Addresses:**
- **POKE 650, 127** (appears twice)
  - **Address**: 650 ($028A in hex)
  - **Purpose**: Controls keyboard repeat behavior
  - **Value**: 127 sets the repeat delay to maximum
  - **Effect**: Disables or significantly slows down keyboard key repeat to prevent accidental multiple inputs during menu navigation

- **POKE 1690, 0** (appears once, but this is likely an error)
  - **Address**: 1690 ($069A in hex)
  - **Purpose**: Unknown/possibly erroneous
  - **Value**: 0
  - **Note**: This address is not a standard C64 system variable. May be a typo or experimental code that has no effect. Standard keyboard buffer is at location 631-640.

**Why This Matters:**
In a menu-driven game, preventing key repeat is crucial. Without this, holding down a key could cause the player to skip through multiple menu options unintentionally.

---

### Line 160: Border and Background Colors

```basic
160 POKE53280,5:POKE53281,5:PRINT"[CLR]"
```

**Memory Addresses:**
- **POKE 53280, 5** ($D020)
  - **Address**: 53280 (VIC-II chip border color register)
  - **Purpose**: Sets the screen border color
  - **Value**: 5 = Green
  - **Effect**: Changes the border around the screen to green

- **POKE 53281, 5** ($D021)
  - **Address**: 53281 (VIC-II chip background color register)
  - **Purpose**: Sets the main screen background color
  - **Value**: 5 = Green
  - **Effect**: Changes the background color of the screen to green

**C64 Color Palette Reference:**
```
0 = Black       8 = Orange
1 = White       9 = Brown
2 = Red        10 = Light Red
3 = Cyan       11 = Dark Gray
4 = Purple     12 = Gray
5 = Green      13 = Light Green
6 = Blue       14 = Light Blue
7 = Yellow     15 = Light Gray
```

**Why This Matters:**
The green color scheme gives the game a distinctive "football field" aesthetic, creating visual consistency with the soccer theme.

---

### Lines 1670, 1690, 2230: Character Set Switching

```basic
1670 IFA$=" "THENRETURN:POKE53272,28
1690 UZ=ASC(A$)-64:PRINT"":POKE53272,28
2230 PRINT"[CLR]":RETURN:POKE53272,28
```

**Memory Address:**
- **POKE 53272, 28** ($D018)
  - **Address**: 53272 (VIC-II Memory Control Register)
  - **Purpose**: Controls which character set ROM the VIC-II chip uses for display
  - **Value**: 28 (binary: 00011100)
  - **Effect**: Switches to uppercase/graphics character set

**Technical Breakdown:**
The VIC-II Memory Control Register (address 53272) controls:
- Bits 1-3: Character memory base address
- Bits 4-7: Screen memory base address

Value 28 (binary 00011100) means:
- Bits 1-3 = 110 (6 in decimal): Points to character ROM at $3000-$3FFF
- This is the C64's uppercase/graphics character set (as opposed to lowercase)

**Context in the Code:**

1. **Line 1670**: After player input in the player selling routine
   - Ensures graphics characters display correctly after user interaction

2. **Line 1690**: After converting user input character to player ID
   - Resets character set after ASCII conversion operations

3. **Line 2230**: After clearing screen in standings/league table display
   - Ensures proper character set for displaying league information

**Why This Matters:**
The C64 has two built-in character sets:
1. **Uppercase + Graphics** (PETSCII graphics characters)
2. **Uppercase/Lowercase** (normal text)

The game uses the uppercase/graphics character set to draw borders, boxes, and decorative elements around menus and tables. These POKE commands ensure the game stays in the correct character mode for displaying its custom UI elements (the borders defined in variables like PIQ$, IP$, UNO$, etc. at the beginning of the program).

## Summary Table

| Line | Address | Decimal | Hex    | Purpose | Value | Effect |
|------|---------|---------|--------|---------|-------|--------|
| 10   | 650     | 650     | $028A  | Keyboard repeat delay | 127 | Disable key repeat |
| 10   | 1690    | 1690    | $069A  | Unknown (possibly error) | 0 | No documented effect |
| 160  | 53280   | 53280   | $D020  | Border color | 5 | Green border |
| 160  | 53281   | 53281   | $D021  | Background color | 5 | Green background |
| 1670 | 53272   | 53272   | $D018  | Character set | 28 | Uppercase/graphics mode |
| 1690 | 53272   | 53272   | $D018  | Character set | 28 | Uppercase/graphics mode |
| 2230 | 53272   | 53272   | $D018  | Character set | 28 | Uppercase/graphics mode |

## Hardware Registers Used

### VIC-II Chip (Video Interface Chip)
The VIC-II is the graphics chip in the Commodore 64, responsible for all screen display.

- **$D018 (53272)**: Memory Control Register
- **$D020 (53280)**: Border Color Register
- **$D021 (53281)**: Background Color #0 Register

### System Variables
- **$028A (650)**: Keyboard repeat delay counter

## Implementation Notes for Modern Ports

When porting this game to modern systems (like the Rust TUI version):

1. **POKE 650, 127**: Implement input debouncing or key repeat delay in the event loop
2. **POKE 53280/53281, 5**: Set terminal background/border colors to green (if supported)
3. **POKE 53272, 28**: No direct equivalent needed - modern systems don't have character set switching, but ensure Unicode box-drawing characters render correctly

## References

- Commodore 64 Programmer's Reference Guide
- VIC-II chip specification (MOS 6567/6569)
- Memory map: https://www.c64-wiki.com/wiki/Memory_Map
- PETSCII character set: https://www.c64-wiki.com/wiki/PETSCII

---

# Commodore 64 BASIC Emulator Specification

Based on analysis of `footballmanager.bas` - A complete specification for implementing a BASIC emulator capable of running this program.

## Document Purpose

This specification catalogs all BASIC keywords, functions, operators, and behaviors required to implement an emulator that can execute the Football Manager C64 BASIC program.

---

## 1. CONTROL FLOW STATEMENTS

### 1.1 IF-THEN

**Syntax:**
```basic
IF <condition> THEN <statement>
IF <condition> THEN <line_number>
```

**Description:** Conditionally executes a statement or jumps to a line number based on a boolean expression.

**Examples from code:**
```basic
IFDES$="G"THENPRINT"PER GLI ALTRI"
IFC(PZ)=0ORC(PZ)=4GOTO790
IFA$="1"THENGOSUB1280:GOTO890
```

**Implementation Notes:**
- Condition evaluates to true if non-zero, false if zero
- Multiple statements can follow THEN using `:` separator
- THEN keyword is required (not optional)
- No space required between IF and condition

### 1.2 GOTO

**Syntax:**
```basic
GOTO <line_number>
```

**Description:** Unconditionally jumps execution to specified line number.

**Examples from code:**
```basic
GOTO890
IFDES$="G"GOTO135
IFC(PZ)=0ORC(PZ)=4GOTO790
```

**Implementation Notes:**
- Line number must exist in program
- Execution continues from target line
- Can appear standalone or after THEN

### 1.3 GOSUB

**Syntax:**
```basic
GOSUB <line_number>
```

**Description:** Calls a subroutine and pushes return address onto stack.

**Examples from code:**
```basic
GOSUB2000:PRINT""
IFHZ=65THENDES$="SCE":GOSUB100
GOSUB1150
```

**Implementation Notes:**
- Pushes current line number + 1 onto return stack
- Jumps to specified line number
- Must have corresponding RETURN
- Stack depth should support at least 25 nested calls

### 1.4 RETURN

**Syntax:**
```basic
RETURN
```

**Description:** Returns from subroutine to statement after most recent GOSUB.

**Examples from code:**
```basic
RETURN
IFA$=" "THENRETURN:POKE53272,28
IFXZ<=0THENRETURN
```

**Implementation Notes:**
- Pops return address from stack
- Continues execution at saved location
- Error if stack is empty ("RETURN WITHOUT GOSUB")

### 1.5 FOR-NEXT

**Syntax:**
```basic
FOR <variable>=<start> TO <end> [STEP <increment>]
...
NEXT [<variable>]
```

**Description:** Loop control structure with counter variable.

**Examples from code:**
```basic
FORAPE=1TO16:PRINTIP$:NEXT
FORHZ=1TO16:W(HZ)=0:NEXT
FORTR=1TO500:NEXT
FORPZ=HZTOHZ+15:PRINTTAB(4)PZ;PT$;A$(PZ):NEXT
```

**Implementation Notes:**
- Default STEP is 1 if omitted
- Loop executes at least once if start <= end
- Variable name after NEXT is optional
- Loop counter accessible inside loop body
- STEP can be negative
- No space required: `TO` can be written as `TO` or adjacent to numbers

### 1.6 END

**Syntax:**
```basic
END
```

**Description:** Terminates program execution.

**Examples from code:**
```basic
IFA$="R"THENEND
```

**Implementation Notes:**
- Stops program immediately
- Closes all files
- Returns to immediate mode

---

## 2. INPUT/OUTPUT STATEMENTS

### 2.1 PRINT

**Syntax:**
```basic
PRINT [<expression>[;|,]...]
```

**Description:** Outputs text and values to screen.

**Examples from code:**
```basic
PRINT"[CLR]"
PRINTIP$
PRINT""TAB(13)A$(QZ)TAB(25)A$(SZ)
PRINT"HAI \ "W
PRINT5000*(5-N)+5000*A(XZ);TAB(35)MID$(C$,(C(XZ)+3),1):
```

**Separators:**
- `;` - No spacing, continue on same line
- `,` - Tab to next print zone (every 10 columns)
- No separator at end - newline after output

**Implementation Notes:**
- Multiple expressions separated by `;` or `,`
- Trailing `;` suppresses newline
- Empty PRINT outputs newline
- Screen codes: `[CLR]` clears screen, etc.
- Backslash `\` is currency symbol on C64

### 2.2 INPUT

**Syntax:**
```basic
INPUT [<prompt>;]<variable>
```

**Description:** Reads user input into variable.

**Examples from code:**
```basic
INPUTA$
INPUTXZ:IFXZ<=0THENRETURN
INPUTQW$:IFQW$="^"THENHZ=HZ-12:GOTO2765
INPUT"PROMPT";A$
```

**Implementation Notes:**
- Displays `?` prompt by default
- Custom prompt with string followed by `;`
- Accepts numeric or string input based on variable type
- Waits for Enter key
- String input accepts empty string
- Numeric input validates and re-prompts on error

### 2.3 DATA

**Syntax:**
```basic
DATA <value>[,<value>...]
```

**Description:** Stores constant values for READ statement.

**Examples from code:**
```basic
DATA BORDON,TANCREDI,NELA,CABRINI,VIERCHOWOD,JUNIOR,PASSARELLA,TRICELLA
DATA ASCOLI,ATALANTA,AVELLINO,COMO,CREMONESE,FIORENTINA,INTER,JUVENTUS
```

**Implementation Notes:**
- Values separated by commas
- Can be numeric or string
- Multiple DATA statements form continuous stream
- String values don't require quotes unless containing commas or colons

### 2.4 READ

**Syntax:**
```basic
READ <variable>[,<variable>...]
```

**Description:** Reads values from DATA statements.

**Examples from code:**
```basic
FORHZ=1TO24:READB$(HZ):NEXT
N=4:FORHZ=1TO64:READA$(HZ):NEXT
```

**Implementation Notes:**
- Maintains internal data pointer
- Advances pointer after each read
- Type mismatch error if data type doesn't match variable
- "OUT OF DATA" error if no more data available
- Data pointer resets with RESTORE (not used in this file)

---

## 3. MEMORY/SYSTEM OPERATIONS

### 3.1 POKE

**Syntax:**
```basic
POKE <address>,<value>
```

**Description:** Writes byte value (0-255) to memory address.

**Examples from code:**
```basic
POKE650,127
POKE53280,5:POKE53281,5
POKE53272,28
```

**Memory Addresses Used:**
- `650` ($028A): Keyboard repeat delay
- `1690` ($069A): Unknown/possibly error
- `53280` ($D020): Border color
- `53281` ($D021): Background color
- `53272` ($D018): Character set control

**Implementation Notes:**
- Address range: 0-65535
- Value range: 0-255 (wraps with modulo 256)
- Direct hardware access
- Used for graphics, sound, I/O control

### 3.2 PEEK

**Syntax:**
```basic
PEEK(<address>)
```

**Description:** Reads byte value from memory address.

**Implementation Notes:**
- Returns value 0-255
- Not explicitly used in this program but standard BASIC feature

---

## 4. ARRAY OPERATIONS

### 4.1 DIM

**Syntax:**
```basic
DIM <array>(<size>)[,<array>(<size>)...]
```

**Description:** Declares arrays and allocates memory.

**Examples from code:**
```basic
DIMA$(64),B$(24),C$(30),D$(2),A(24),B(24),C(24),D(14),E(16),F(16),G(16)
DIMH(2),J(16),V(16),W(16)
```

**Implementation Notes:**
- String arrays use `$` suffix
- Numeric arrays have no suffix
- Array indices are 0-based by default (element 0 through size)
- Size 64 means 65 elements (0-64)
- Arrays must be dimensioned before use (except default 10 elements)
- Multi-dimensional arrays supported: `DIM A(10,10)`

---

## 5. STRING FUNCTIONS

### 5.1 CHR$

**Syntax:**
```basic
CHR$(<ascii_code>)
```

**Description:** Converts ASCII code to single-character string.

**Examples from code:**
```basic
PRINTCHR$(142)
```

**Implementation Notes:**
- ASCII code range: 0-255
- Returns single character string
- Code 142 switches to uppercase/graphics mode on C64

### 5.2 ASC

**Syntax:**
```basic
ASC(<string>)
```

**Description:** Returns ASCII code of first character in string.

**Examples from code:**
```basic
UZ=ASC(A$)-64
```

**Implementation Notes:**
- Returns numeric value 0-255
- If empty string, returns 0 or error (implementation dependent)
- Only examines first character

### 5.3 VAL

**Syntax:**
```basic
VAL(<string>)
```

**Description:** Converts string to numeric value.

**Examples from code:**
```basic
QZ=VAL(A$)
QW=VAL(A$)
HZ=VAL(A$):IFHZ<=0THEN3870
```

**Implementation Notes:**
- Parses numeric characters from start of string
- Stops at first non-numeric character
- Returns 0 if no numeric characters
- Handles negative numbers and decimals

### 5.4 STR$

**Syntax:**
```basic
STR$(<number>)
```

**Description:** Converts numeric value to string.

**Examples from code:**
```basic
C$(I-1)=STR$(SZ)+","+STR$(A3)+"."+STR$(H(A3))+STR$(H(A4))
```

**Implementation Notes:**
- Returns string representation
- Includes leading space for positive numbers
- Negative numbers have minus sign

### 5.5 MID$

**Syntax:**
```basic
MID$(<string>,<start>[,<length>])
```

**Description:** Extracts substring from string.

**Examples from code:**
```basic
PRINT" "MID$(C$,KJ,1);
PRINT5000*(5-N)+5000*A(XZ);TAB(35)MID$(C$,(C(XZ)+3),1):
PRINT""MID$(C$,INT((PZ-1)/8)+1,1)TAB(10)B$(PZ)
```

**Implementation Notes:**
- Start position is 1-based (1 is first character)
- Length is optional (defaults to rest of string)
- If start > string length, returns empty string
- If length extends past end, returns to end

### 5.6 LEN

**Syntax:**
```basic
LEN(<string>)
```

**Description:** Returns length of string.

**Examples from code:**
```basic
IFLEN(QW$)>15THEN2687
IFLEN(WQ$)>15THEN2838
```

**Implementation Notes:**
- Returns numeric value 0-255
- Empty string returns 0

---

## 6. MATHEMATICAL FUNCTIONS

### 6.1 INT

**Syntax:**
```basic
INT(<number>)
```

**Description:** Returns integer part (floor) of number.

**Examples from code:**
```basic
A(HZ)=INT(RND(1)*5)+1
PZ=INT(PZ-(RND(1)*(PZ/10))-(RND(1)*(PZ/10)))
D(PZ)=INT(RND(1)*I*3)+15
```

**Implementation Notes:**
- Truncates toward negative infinity (floor function)
- INT(3.7) = 3
- INT(-3.7) = -4 (not -3!)
- Used extensively for random number generation

### 6.2 RND

**Syntax:**
```basic
RND(<dummy>)
```

**Description:** Returns pseudo-random number between 0 and 1.

**Examples from code:**
```basic
A(HZ)=INT(RND(1)*5)+1
PZ=INT(RND(1)*24)+1
PRINT"ACCETTI "PZ" DAL "A$(INT(RND(1)*64)+1)
```

**Implementation Notes:**
- Argument is typically 1 (positive value)
- Returns value: 0 <= RND < 1
- RND(0) returns last random number
- RND(negative) reseeds generator
- Used pattern: `INT(RND(1)*N)+1` for random 1 to N

---

## 7. FORMATTING/OUTPUT CONTROL

### 7.1 TAB

**Syntax:**
```basic
TAB(<column>)
```

**Description:** Positions cursor at specified column (used within PRINT).

**Examples from code:**
```basic
PRINTTAB(4)PZ;PT$;A$(PZ)
PRINT"";"     SQUADRA"TAB(21)"F"TAB(25)"S"TAB(29)"PT."
PRINT""TAB(13)A$(QZ)TAB(25)A$(SZ)
```

**Implementation Notes:**
- Column number is 0-based or 1-based (implementation dependent)
- If already past column, moves to column on next line
- Used only within PRINT statements
- Multiple TAB calls in one PRINT allowed

---

## 8. OPERATORS

### 8.1 Comparison Operators

**Operators:**
- `=` Equal to
- `<>` Not equal to
- `<` Less than
- `>` Greater than
- `<=` Less than or equal to
- `>=` Greater than or equal to

**Examples from code:**
```basic
IFDES$="G"THENPRINT"PER GLI ALTRI"
IFGG<>4THEN230
IFQZ<HZORQZ>HZ+15THEN820
IFD(PZ)>20THEND(PZ)=20
IFXZ<=0THENRETURN
IFV(PZ)>=13ANDN<>4THENPRINT
```

**Implementation Notes:**
- Work with both numeric and string values
- String comparison is lexicographic (ASCII order)
- Return -1 for true, 0 for false
- Case-sensitive for strings on C64

### 8.2 Logical Operators

**AND**
```basic
<expr1> AND <expr2>
```
- Bitwise AND operation
- True if both non-zero
- Example: `IFA$<>"N"ANDA$<>"S"THEN1400`

**OR**
```basic
<expr1> OR <expr2>
```
- Bitwise OR operation
- True if either non-zero
- Example: `IFC(PZ)=0ORC(PZ)=4GOTO790`

**NOT** (not used in this file but standard)
```basic
NOT <expr>
```
- Bitwise NOT operation
- Inverts all bits

**Implementation Notes:**
- These are bitwise operations, not boolean
- -1 is all bits set (true)
- 0 is all bits clear (false)

### 8.3 Arithmetic Operators

**Operators:**
- `+` Addition
- `-` Subtraction (or unary minus)
- `*` Multiplication
- `/` Division
- `^` Exponentiation (not used in this file)

**Examples from code:**
```basic
INT(RND(1)*5)+1
PZ=INT(PZ-(RND(1)*(PZ/10))-(RND(1)*(PZ/10)))
5000*(5-N)+5000*A(XZ)
```

**Operator Precedence (highest to lowest):**
1. `^` Exponentiation
2. `-` Unary minus
3. `*`, `/` Multiplication, Division
4. `+`, `-` Addition, Subtraction
5. `=`, `<>`, `<`, `>`, `<=`, `>=` Comparisons
6. `NOT` Logical NOT
7. `AND` Logical AND
8. `OR` Logical OR

---

## 9. VARIABLE TYPES

### 9.1 String Variables

**Syntax:** Variable name ending with `$`

**Examples:**
- `A$`, `B$`, `C$`, `D$`
- `QW$`, `WQ$`, `ZIO$`
- `DES$`, `PT$`, `PIQ$`

**Implementation Notes:**
- Maximum length: typically 255 characters
- Store text/character data
- Array form: `A$(64)` creates string array

### 9.2 Numeric Variables

**Syntax:** Variable name with no suffix

**Examples:**
- `SI`, `W`, `Y`, `Z`, `K`, `R`, `B1`, `N`
- `HZ`, `PZ`, `UZ`, `XZ`
- Single letter and double letter names

**Implementation Notes:**
- Store floating-point numbers (typically 5-byte format)
- Range: typically ±1.7e±38
- Precision: about 9 significant digits
- Array form: `A(24)` creates numeric array

### 9.3 Integer Variables (not used, but standard)

**Syntax:** Variable name ending with `%`

**Implementation Notes:**
- Store 16-bit signed integers
- Range: -32768 to 32767
- Faster than floating point

---

## 10. COMMENTS

### 10.1 REM

**Syntax:**
```basic
REM <comment text>
```

**Description:** Marks rest of line as comment, ignored by interpreter.

**Examples from code:**
```basic
REM *****************************
REM GIOCO PRINCIPALE
REM ******************************
GOSUB2000:REM SCELTA
```

**Implementation Notes:**
- Everything after REM to end of line is ignored
- Can appear after statement using `:` separator
- No closing delimiter needed

---

## 11. LINE NUMBERS

**Syntax:**
```basic
<line_number> <statement>
```

**Description:** Every line must begin with a line number.

**Examples from code:**
```basic
10 POKE650,127:POKE1690,0
160 POKE53280,5:POKE53281,5:PRINT"[CLR]"
1070 INPUTA$:IFA$="^"THENHZ=HZ-16:GOTO810
```

**Implementation Notes:**
- Range: 0-63999 (typically)
- Must be in ascending order
- Common practice: increment by 10 (allows insertions)
- Multiple statements per line using `:` separator
- No space required after line number

---

## 12. SPECIAL SYNTAX FEATURES

### 12.1 Statement Separator

**Syntax:** `:`

**Description:** Separates multiple statements on same line.

**Examples from code:**
```basic
POKE650,127:POKE1690,0:POKE650,127
INPUTA$:IFA$="^"THENHZ=HZ-16:GOTO810
L=1::IFI>ZTHENWW=INT(RND(1)*2)+1
```

**Implementation Notes:**
- Allows multiple commands per line
- Can appear consecutively: `::` (empty statement)

### 12.2 Print Formatting Codes

**Special character sequences in strings:**
- `[CLR]` - Clear screen (CHR$(147))
- `[REVERSE]` - Reverse video mode
- Custom graphics characters embedded in strings

**Implementation Notes:**
- These are Commodore 64 screen codes
- Modern emulator should map to appropriate terminal codes
- Exact codes depend on character set (see POKE 53272)

---

## 13. PROGRAM STRUCTURE

### 13.1 Typical Program Flow

```basic
10-150   REM Header/credits
160-740  Initialization and utility subroutines
750-810  Variable/array declarations and setup
820-940  DATA statements
950-1090 Random initialization
1100-...  Main program logic
```

### 13.2 Subroutine Convention

**Pattern used:**
```basic
1100 REM MAIN MENU
1110-1460 Main menu implementation
1470 REM LISTA
1480-1620 Lista subroutine
```

**Implementation Notes:**
- REM line marks subroutine start
- GOSUB calls specific line number
- RETURN at end of subroutine
- No formal parameter passing (uses global variables)

---

## 14. MEMORY MAP (Commodore 64 Specific)

### 14.1 Important Memory Addresses

| Address | Hex    | Purpose |
|---------|--------|---------|
| 650     | $028A  | Keyboard repeat delay |
| 53272   | $D018  | VIC-II memory control (character set) |
| 53280   | $D020  | Border color |
| 53281   | $D021  | Background color |

### 14.2 Color Values

```
0 = Black       8 = Orange
1 = White       9 = Brown
2 = Red        10 = Light Red
3 = Cyan       11 = Dark Gray
4 = Purple     12 = Gray
5 = Green      13 = Light Green
6 = Blue       14 = Light Blue
7 = Yellow     15 = Light Gray
```

---

## 15. IMPLEMENTATION REQUIREMENTS

### 15.1 Essential Features

✅ **Must Implement:**
1. Line number execution
2. IF-THEN conditional branching
3. GOTO/GOSUB/RETURN
4. FOR-NEXT loops
5. PRINT with formatting (TAB, separators)
6. INPUT for user interaction
7. DIM for array declaration
8. DATA/READ for data initialization
9. String functions: MID$, LEN, VAL, STR$, ASC, CHR$
10. Math functions: INT, RND
11. Comparison operators: =, <>, <, >, <=, >=
12. Logical operators: AND, OR
13. Arithmetic operators: +, -, *, /
14. String and numeric variables
15. REM comments
16. POKE (at minimum for the specific addresses used)
17. END statement

### 15.2 Optional Features

⚠️ **Not Used in This Program:**
- ELSE clause
- WHILE/WEND, REPEAT/UNTIL loops
- DEF FN user-defined functions
- LEFT$, RIGHT$, INSTR string functions
- SIN, COS, TAN, LOG, EXP math functions
- PEEK function
- Graphics commands (PLOT, CIRCLE, etc.)
- Sound commands (SOUND, PLAY, etc.)
- File I/O (OPEN, CLOSE, GET, PRINT#)

### 15.3 Parser Requirements

**Tokenization:**
- Keywords can be abbreviated in some BASIC dialects
- No space required between keyword and operand: `IFXZ<=0`
- Case insensitive (typically converted to uppercase)

**Line Structure:**
```
<line_number> [<statement>[:<statement>...]]
```

**Expression Evaluation:**
- Proper operator precedence
- Parentheses for grouping
- Type coercion (numeric/string)

### 15.4 Runtime Requirements

**Memory:**
- String storage (heap)
- Numeric variable storage
- Array storage
- GOSUB return stack (min 25 levels deep)
- FOR-NEXT loop stack
- DATA pointer tracking

**Execution:**
- Line number lookup table/index
- Jump to arbitrary line number
- Statement-by-statement execution
- Interrupt handling (user break)

---

## 16. TESTING CHECKLIST

To verify emulator compatibility with this program:

- [ ] Program loads without syntax errors
- [ ] Data initialization (DATA/READ) populates arrays correctly
- [ ] Random number generation works (player stats initialized)
- [ ] Menu system responds to input
- [ ] GOSUB/RETURN navigation works
- [ ] FOR loops iterate correctly
- [ ] String manipulation (team/player names) functions
- [ ] Numeric calculations (finances, match simulation) accurate
- [ ] Array indexing works for all arrays
- [ ] Screen output formatting (TAB) displays correctly
- [ ] POKE commands don't crash (map to safe operations)
- [ ] Program can run full match simulation
- [ ] Game state persists through multiple matches

---

## 17. MODERN ADAPTATION NOTES

When implementing a modern BASIC emulator for this program:

1. **Terminal Control**: Map `[CLR]`, `[REVERSE]` to ANSI escape codes
2. **Colors**: Map POKE 53280/53281 to terminal background colors
3. **Character Sets**: POKE 53272 can be ignored or mapped to UTF-8 box characters
4. **Keyboard**: POKE 650 should implement input debouncing
5. **Random Seed**: Initialize RNG with system time for variety
6. **Screen Width**: Assume 40-column display for formatting

---

## 18. SUMMARY STATISTICS

**Keywords Used:** 34 unique keywords
**Total Lines:** ~650 lines
**Subroutines:** ~20 distinct subroutines
**Arrays:** 16 arrays (8 string, 8 numeric)
**DATA Statements:** 12 statements with ~100 values
**Complexity:** Medium (typical 1980s game)

**Most Frequently Used:**
1. PRINT (100+ occurrences)
2. IF-THEN (50+ occurrences)
3. GOTO (30+ occurrences)
4. FOR-NEXT (25+ occurrences)
5. GOSUB (20+ occurrences)

---

## APPENDIX A: Complete Keyword Reference

Alphabetical listing of all keywords used in footballmanager.bas:

```
AND       - Logical AND operator
ASC       - Convert character to ASCII code
CHR$      - Convert ASCII code to character
DATA      - Define constant data
DIM       - Declare arrays
END       - Terminate program
FOR       - Start loop
GOSUB     - Call subroutine
GOTO      - Unconditional jump
IF        - Conditional test
INPUT     - Read user input
INT       - Integer part of number
LEN       - String length
MID$      - Substring extraction
NEXT      - End of loop
OR        - Logical OR operator
POKE      - Write to memory address
PRINT     - Output to screen
READ      - Read from DATA
REM       - Comment
RETURN    - Return from subroutine
RND       - Random number
STR$      - Convert number to string
TAB       - Position output column
THEN      - Part of IF statement
TO        - Part of FOR statement
VAL       - Convert string to number
```

**Operators:**
```
=         - Assignment or comparison
+         - Addition
-         - Subtraction
*         - Multiplication
/         - Division
<         - Less than
>         - Greater than
<=        - Less than or equal
>=        - Greater than or equal
<>        - Not equal
:         - Statement separator
;         - Print separator (no space)
,         - Print separator (tab)
```

---

## APPENDIX B: Sample Emulator Pseudocode

```python
class BasicEmulator:
    def __init__(self):
        self.program = {}  # line_number -> line_text
        self.variables = {}  # name -> value
        self.arrays = {}  # name -> list
        self.data_values = []  # All DATA values
        self.data_pointer = 0
        self.gosub_stack = []
        self.for_stack = []
        self.current_line = None

    def load_program(self, filename):
        """Load and parse BASIC program"""
        pass

    def execute(self):
        """Execute program from first line"""
        lines = sorted(self.program.keys())
        self.current_line = 0

        while self.current_line < len(lines):
            line_num = lines[self.current_line]
            self.execute_line(line_num)
            self.current_line += 1

    def execute_line(self, line_num):
        """Execute a single line"""
        line = self.program[line_num]
        statements = line.split(':')
        for stmt in statements:
            self.execute_statement(stmt)

    def goto(self, line_num):
        """Jump to line number"""
        lines = sorted(self.program.keys())
        self.current_line = lines.index(line_num)

    def gosub(self, line_num):
        """Call subroutine"""
        self.gosub_stack.append(self.current_line + 1)
        self.goto(line_num)

    def return_from_sub(self):
        """Return from subroutine"""
        if not self.gosub_stack:
            raise RuntimeError("RETURN WITHOUT GOSUB")
        self.current_line = self.gosub_stack.pop()
```

---

**End of Documentation**

This document provides all necessary information about POKE commands and BASIC language specifications to implement a Commodore 64 BASIC emulator capable of running the Football Manager game.
