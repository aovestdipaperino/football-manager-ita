# C64 Fidelity Issues Found

Based on running the fidelity test suite, here are the issues we need to fix:

## Issues Found

### 1. **Trailing Spaces in Output** ✗
**Problem**: `get_content()` returns full 40-character width buffer with trailing spaces
**C64 Behavior**: Lines don't have trailing spaces in output
**Impact**: All output comparisons fail due to extra spaces
**Fix**: Modify `Screen::get_content()` to trim trailing spaces from each line
**File**: `src/screen.rs:149-156`

### 2. **Missing Space After Numbers in Semicolon PRINT** ✗
**Problem**: `PRINT 1;2;3` outputs `123`, should be `1 2 3` (with spaces)
**C64 Behavior**: Numbers are followed by a space in PRINT, even with semicolon
**Impact**: Tests like `for_loop_step`, `nested_for_loops`, `data_read` fail
**Fix**: Add space after numbers in `to_display_string()` or in PRINT handler
**File**: `src/value.rs:35-46` or `src/interpreter.rs:159-196`

### 3. **Leading Space on Positive Numbers** ✗
**Problem**: `PRINT 5` outputs `5`, should be ` 5` (with leading space)
**C64 Behavior**: Positive numbers have a leading space (for sign alignment)
**Impact**: Tests like `variable_initialization` fail
**Fix**: Add leading space to positive numbers in `to_display_string()`
**File**: `src/value.rs:35-46`

### 4. **Floating Point Precision Display** ✗
**Problem**: `10/3` outputs `3.3333333333333335`, C64 shows `3.33333333` (9 digits)
**C64 Behavior**: Float display limited to 9 significant digits
**Impact**: `basic_arithmetic` test fails
**Fix**: Limit float display to 9 significant digits
**File**: `src/value.rs:35-46`

## C64 PRINT Number Formatting Rules

From Commodore 64 Programmer's Reference Guide:

1. **Positive numbers**: Leading space `" 5"`
2. **Negative numbers**: Leading minus `"-5"`
3. **Trailing space**: Always added after a number (unless trailing semicolon at end of line)
4. **Precision**: Floating point shows max 9 significant digits
5. **Integer display**: Numbers with no fractional part shown without decimal point

## Implementation Plan

### Phase 1: Fix Screen Output (High Priority)
```rust
// src/screen.rs
pub fn get_content(&self) -> String {
    let buffer = self.buffer.lock().unwrap();
    buffer
        .iter()
        .map(|row| row.iter().collect::<String>().trim_end().to_string())  // ADD trim_end()
        .collect::<Vec<_>>()
        .join("\n")
}
```

### Phase 2: Fix Number Formatting (High Priority)
```rust
// src/value.rs
pub fn to_display_string(&self) -> String {
    match self {
        Value::Number(n) => {
            // C64-style number formatting
            let formatted = if n.fract() == 0.0 && n.abs() < 1e10 {
                format!("{}", *n as i64)
            } else {
                // Limit to 9 significant digits
                format!("{:.9}", n).trim_end_matches('0').trim_end_matches('.').to_string()
            };

            // Add leading space for positive numbers (C64 sign alignment)
            if *n >= 0.0 {
                format!(" {}", formatted)
            } else {
                formatted
            }
        }
        Value::String(s) => s.clone(),
    }
}
```

### Phase 3: Add Trailing Space After Numbers in PRINT
```rust
// src/interpreter.rs - exec_print()
PrintItem::Expr(expr) => {
    let value = self.eval_expr(expr)?;
    let text = value.to_display_string();
    self.screen.print(&text);

    // C64 adds space after numbers (but not strings)
    if matches!(value, Value::Number(_)) {
        self.screen.print(" ");
        column += text.len() + 1;
    } else {
        column += text.len();
    }
}
```

## Test Results Before Fixes

```
Results: 4 passed, 13 failed, 0 errors
```

### Tests Failing:
1. basic_arithmetic - Float precision
2. print_zones - Trailing spaces
3. print_semicolon - Trailing spaces, number spacing
4. variable_initialization - Trailing spaces, leading space
5. for_loop_step - Number spacing
6. nested_for_loops - Number spacing
7. if_then_goto - Trailing spaces
8. gosub_return - Trailing spaces
9. data_read - Number spacing
10. string_functions - Trailing spaces, leading space
11. val_str_functions - Trailing spaces, leading space
12. comparison_operators - Trailing spaces, leading space
13. logical_operators - Trailing spaces, leading space

### Tests Passing:
1. string_concatenation ✓
2. array_bounds ✓
3. for_loop_single_iteration ✓
4. mid_function ✓

## Expected Test Results After Fixes

All 17 tests should pass once we implement:
1. Trim trailing spaces from screen output
2. Add leading space to positive numbers
3. Add trailing space after numbers in PRINT
4. Limit float precision to 9 digits

## Additional C64 Quirks to Implement Later

1. **RND() algorithm**: Use exact C64 PRNG
2. **String space management**: Implement C64-style string descriptors
3. **TAB() behavior**: Exact zone positioning
4. **Error messages**: Match C64 error text
5. **Memory limits**: Simulate C64 memory constraints
6. **POKE/PEEK**: Accurate memory location behavior

## References

- Commodore 64 Programmer's Reference Guide, Chapter 2: BASIC Language
- C64 BASIC ROM disassembly: $E6BD (FOUT - Float Output)
- microsoft/BASIC-M6502 source code
