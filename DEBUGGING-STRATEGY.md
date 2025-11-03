# C64 BASIC Interpreter Fidelity Debugging Strategy

This document outlines our approach to ensure the Rust interpreter behaves identically to the original Commodore 64 BASIC V2 interpreter.

## Problem Statement

The interpreter may behave differently from a real C64 when running `footballmanager.bas` or `footballmanager.prg`. We need to:
1. Identify specific behavioral differences
2. Create reproducible test cases
3. Fix the interpreter to match C64 behavior
4. Prevent regressions

## Debugging Approach

### Phase 1: Capture Reference Behavior

**Objective**: Get ground truth from real C64 emulator

#### Tools Needed:
- VICE C64 emulator (most accurate)
- Script to capture program output
- Test programs that exercise specific features

#### Actions:
1. Run `footballmanager.prg` in VICE
2. Capture screen output at key points
3. Record user inputs and resulting outputs
4. Note any prompts, calculations, or display issues

#### Test Programs to Create:
```basic
REM Test 1: Basic arithmetic
10 PRINT 5/2
20 PRINT 5\2
30 PRINT INT(5/2)

REM Test 2: RND() behavior
10 FOR I=1 TO 5: PRINT RND(1): NEXT

REM Test 3: String handling
10 A$="HELLO"
20 PRINT A$;"WORLD"
30 PRINT A$,"WORLD"

REM Test 4: Array initialization
10 DIM A(10)
20 PRINT A(5)

REM Test 5: FOR loop edge cases
10 FOR I=1 TO 1 STEP 2
20 PRINT I
30 NEXT I
```

### Phase 2: Trace Execution

**Objective**: Compare step-by-step execution

#### Implementation:
Create a trace mode that logs:
- Line number executed
- Statement type
- Variable values after execution
- Screen output
- Input received

#### Tools to Build:
1. **Trace Logger**
   ```rust
   pub struct ExecutionTrace {
       line_number: u32,
       statement: String,
       variables_snapshot: HashMap<String, Value>,
       output: String,
   }
   ```

2. **Comparison Tool**
   - Run same program in VICE with trace output
   - Run in our interpreter with trace output
   - Diff the traces to find first divergence

### Phase 3: Known C64 Quirks to Check

#### Numeric Precision
- C64 uses Microsoft Binary Format (MBF) for floating point
- 40-bit numbers: 1 byte exponent + 4 bytes mantissa + 1 sign bit
- Our Rust uses IEEE 754 (f64)
- **Action**: Check if precision differences cause issues

#### RND() Function
- C64 uses specific PRNG algorithm
- Seed is stored at memory location $8B-$8F
- **Action**: Implement exact C64 RND() algorithm

#### String Memory Management
- C64 stores strings in string space, grows downward from $A000
- 3-byte string descriptors: length, address_lo, address_hi
- **Action**: Check string operations (concatenation, MID$, etc.)

#### Array Bounds
- C64 DIM A(10) creates array with indices 0-10 (11 elements)
- **Action**: Verify our array allocation matches

#### Variable Initialization
- C64 initializes numeric variables to 0
- C64 initializes string variables to ""
- **Action**: Confirm our interpreter does the same

#### PRINT Formatting
- Zone width: 10 characters
- Comma moves to next zone
- Semicolon suppresses space
- **Action**: Test PRINT TAB(), SPC(), zones

#### INPUT Behavior
- INPUT prompt shows "?" by default
- Extra prompt shows "??" on re-prompt
- **Action**: Check INPUT handling

#### Operator Precedence
- C64 precedence: ^, *, /, +, -, =, <>, <, >, <=, >=, NOT, AND, OR
- **Action**: Test complex expressions

### Phase 4: Build Test Harness

#### Components:

1. **Reference Output Generator**
   ```bash
   # Script to run program in VICE and capture output
   vice-capture.sh footballmanager.prg > reference.txt
   ```

2. **Interpreter Test Runner**
   ```rust
   cargo run --example generate_test_output -- footballmanager.bas > our_output.txt
   ```

3. **Diff Tool**
   ```bash
   diff -u reference.txt our_output.txt
   ```

4. **Automated Test Suite**
   ```rust
   #[test]
   fn test_c64_fidelity() {
       let programs = [
           "tests/arithmetic.bas",
           "tests/strings.bas",
           "tests/arrays.bas",
           "tests/loops.bas",
       ];

       for program in programs {
           let our_output = run_interpreter(program);
           let reference_output = load_reference(program);
           assert_eq!(our_output, reference_output);
       }
   }
   ```

### Phase 5: Specific Areas to Investigate

#### 1. Screen Codes vs PETSCII
- POKE 1024+X uses screen codes (0-255)
- PRINT uses PETSCII (0-255, different mapping)
- **Check**: Our POKE implementation

#### 2. Color Handling
- POKE 53280 (border color)
- POKE 53281 (background color)
- POKE 646 (text color)
- **Check**: Do we track these correctly?

#### 3. CHR$() Function
- CHR$(13) is RETURN
- CHR$(147) clears screen
- CHR$(142) switches to uppercase/graphics
- **Check**: Character set handling

#### 4. Memory Locations
- POKE 650 (key repeat)
- PEEK(53280) (border color)
- **Check**: Do we simulate these?

#### 5. GOTO/GOSUB Stack
- C64 has limited GOSUB stack (can overflow)
- **Check**: Do we match stack depth limits?

#### 6. String Concatenation
- C64 limits: string length max 255
- **Check**: String length limits

### Phase 6: Systematic Testing

#### Test Categories:

1. **Unit Tests**: Individual language features
2. **Integration Tests**: Small programs (10-20 lines)
3. **Fidelity Tests**: Real C64 programs
4. **Regression Tests**: Previously fixed bugs

#### Example Fidelity Test:
```rust
#[test]
fn test_rnd_sequence() {
    // This should produce the same sequence as C64
    let program = "
        10 FOR I=1 TO 5
        20 PRINT RND(1)
        30 NEXT I
    ";

    let output = run_program(program);
    let expected = vec![
        "0.185564041",
        "0.362945318",
        "0.547366858",
        // ... C64-accurate sequence
    ];

    assert_eq!(output, expected);
}
```

## Action Items

### Immediate:
1. [ ] Document specific differences observed in footballmanager
2. [ ] Create minimal reproduction cases
3. [ ] Set up VICE with script recording capability
4. [ ] Build trace output mode in interpreter

### Short-term:
1. [ ] Implement exact C64 RND() algorithm
2. [ ] Fix any PRINT formatting issues
3. [ ] Verify array bounds behavior
4. [ ] Test all operators and functions

### Long-term:
1. [ ] Build comprehensive test suite
2. [ ] Create C64 reference output database
3. [ ] Add continuous testing against VICE
4. [ ] Document all C64 quirks we support

## Tools We Need

### 1. VICE Automation Script
```bash
#!/bin/bash
# vice-test.sh - Run program in VICE and capture output
vice64 -autostart "$1" -screenshot output.png -sound no
```

### 2. Trace Comparison Tool
```rust
// compare_traces.rs
fn main() {
    let trace1 = load_trace("vice_trace.json");
    let trace2 = load_trace("our_trace.json");
    compare_and_report(trace1, trace2);
}
```

### 3. Test Case Generator
```rust
// Generate test cases for specific features
fn generate_arithmetic_tests() -> Vec<TestCase> { ... }
fn generate_string_tests() -> Vec<TestCase> { ... }
```

## Expected Issues

Based on common BASIC interpreter differences:

1. **Floating point precision**
   - Solution: Use same MBF format or accept minor differences

2. **RND() sequence**
   - Solution: Implement exact C64 PRNG

3. **PRINT spacing**
   - Solution: Match zone widths exactly

4. **String memory**
   - Solution: Implement string descriptor system

5. **POKE/PEEK**
   - Solution: Simulate key memory locations

## Success Criteria

1. ✅ footballmanager runs identically to C64
2. ✅ All test programs produce matching output
3. ✅ No visual differences in PRINT output
4. ✅ Same calculation results (within floating point tolerance)
5. ✅ INPUT/OUTPUT behavior matches
6. ✅ Error messages match (if applicable)

## Next Steps

**Start with**: What specific differences did you observe? This will help us prioritize which areas to investigate first.
