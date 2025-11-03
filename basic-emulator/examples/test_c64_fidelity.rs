/// C64 Fidelity Test Suite
///
/// This tool runs test programs and compares output with expected C64 behavior.
/// Reference outputs should be captured from VICE emulator.

use basic_emulator::interpreter::Interpreter;
use basic_emulator::parser::Parser;
use basic_emulator::screen::Screen;
use std::fs;

struct TestCase {
    name: &'static str,
    program: &'static str,
    expected_output: &'static str,
    description: &'static str,
}

fn run_program(source: &str) -> Result<String, String> {
    let screen = Screen::new();
    let mut interpreter = Interpreter::new(screen.clone());

    interpreter.load_program(source)?;

    // Run up to 10000 steps or until program ends
    let mut steps = 0;
    loop {
        match interpreter.step() {
            Ok(true) => {
                steps += 1;
                if steps > 10000 {
                    return Err("Program did not complete within 10000 steps".to_string());
                }
            }
            Ok(false) => break, // Program ended
            Err(e) => return Err(format!("Runtime error: {}", e)),
        }
    }

    Ok(screen.get_content())
}

fn main() {
    let tests = vec![
        TestCase {
            name: "basic_arithmetic",
            description: "Test integer division and rounding",
            program: r#"
10 PRINT 5/2
20 PRINT INT(5/2)
30 PRINT 10/3
"#,
            expected_output: " 2.5\n 2\n 3.333333333\n",  // Trailing spaces trimmed at end of line
        },

        TestCase {
            name: "print_zones",
            description: "Test PRINT comma zones (10 char width)",
            program: r#"
10 PRINT "A","B","C"
20 PRINT "HELLO","WORLD"
"#,
            expected_output: "A         B         C\nHELLO     WORLD\n",
        },

        TestCase {
            name: "print_semicolon",
            description: "Test PRINT semicolon (no space)",
            program: r#"
10 PRINT "A";"B";"C"
20 PRINT "HELLO";"WORLD"
"#,
            expected_output: "ABC\nHELLOWORLD\n",
        },

        TestCase {
            name: "string_concatenation",
            description: "Test string concatenation with +",
            program: r#"
10 A$="HELLO"
20 B$="WORLD"
30 PRINT A$+B$
"#,
            expected_output: "HELLOWORLD\n",
        },

        TestCase {
            name: "array_bounds",
            description: "Test DIM A(10) creates 11 elements (0-10)",
            program: r#"
10 DIM A(10)
20 A(0)=100
30 A(10)=200
40 PRINT A(0),A(10)
"#,
            expected_output: " 100       200\n",
        },

        TestCase {
            name: "variable_initialization",
            description: "Test uninitialized variables are 0 or empty string",
            program: r#"
10 PRINT X
20 PRINT Y$
30 PRINT Z
"#,
            expected_output: " 0\n\n 0\n",
        },

        TestCase {
            name: "for_loop_step",
            description: "Test FOR loop with STEP",
            program: r#"
10 FOR I=1 TO 10 STEP 2
20 PRINT I;
30 NEXT I
"#,
            expected_output: " 1  3  5  7  9",  // C64 adds space before and after each number
        },

        TestCase {
            name: "for_loop_single_iteration",
            description: "Test FOR loop with start=end",
            program: r#"
10 FOR I=5 TO 5
20 PRINT I
30 NEXT I
"#,
            expected_output: " 5\n",
        },

        TestCase {
            name: "nested_for_loops",
            description: "Test nested FOR loops",
            program: r#"
10 FOR I=1 TO 3
20 FOR J=1 TO 2
30 PRINT I;J;
40 NEXT J
50 NEXT I
"#,
            expected_output: " 1  1  1  2  2  1  2  2  3  1  3  2",  // C64 adds space before and after each number
        },

        TestCase {
            name: "if_then_goto",
            description: "Test IF-THEN-GOTO",
            program: r#"
10 X=5
20 IF X=5 THEN GOTO 50
30 PRINT "NO"
40 GOTO 60
50 PRINT "YES"
60 PRINT "DONE"
"#,
            expected_output: "YES\nDONE\n",
        },

        TestCase {
            name: "gosub_return",
            description: "Test GOSUB and RETURN",
            program: r#"
10 GOSUB 100
20 PRINT "MAIN"
30 END
100 PRINT "SUB"
110 RETURN
"#,
            expected_output: "SUB\nMAIN\n",
        },

        TestCase {
            name: "data_read",
            description: "Test DATA and READ",
            program: r#"
10 DATA 10,20,30
20 READ A,B,C
30 PRINT A;B;C
"#,
            expected_output: " 10  20  30",  // C64 adds space before and after each number
        },

        TestCase {
            name: "string_functions",
            description: "Test CHR$, ASC, LEN",
            program: r#"
10 A$="HELLO"
20 PRINT LEN(A$)
30 PRINT ASC(A$)
40 PRINT CHR$(72)
"#,
            expected_output: " 5\n 72\nH\n",
        },

        TestCase {
            name: "mid_function",
            description: "Test MID$ function",
            program: r#"
10 A$="HELLO"
20 PRINT MID$(A$,2,3)
"#,
            expected_output: "ELL\n",
        },

        TestCase {
            name: "val_str_functions",
            description: "Test VAL and STR$",
            program: r#"
10 PRINT VAL("123")
20 PRINT STR$(456)
"#,
            expected_output: " 123\n 456\n",
        },

        TestCase {
            name: "comparison_operators",
            description: "Test comparison operators",
            program: r#"
10 PRINT 5>3
20 PRINT 5<3
30 PRINT 5=5
40 PRINT 5<>3
"#,
            expected_output: "-1\n 0\n-1\n-1\n",
        },

        TestCase {
            name: "logical_operators",
            description: "Test AND, OR, NOT",
            program: r#"
10 PRINT 1 AND 1
20 PRINT 1 AND 0
30 PRINT 1 OR 0
40 PRINT NOT 0
"#,
            expected_output: " 1\n 0\n 1\n-1\n",
        },
    ];

    println!("C64 BASIC Fidelity Test Suite");
    println!("==============================\n");

    let mut passed = 0;
    let mut failed = 0;
    let mut errors = 0;

    for test in tests {
        print!("Testing {}: {}... ", test.name, test.description);

        match run_program(test.program) {
            Ok(output) => {
                let output = output.trim();
                let expected = test.expected_output.trim();

                if output == expected {
                    println!("✓ PASS");
                    passed += 1;
                } else {
                    println!("✗ FAIL");
                    println!("  Expected: {:?}", expected);
                    println!("  Got:      {:?}", output);
                    failed += 1;
                }
            }
            Err(e) => {
                println!("✗ ERROR: {}", e);
                errors += 1;
            }
        }
    }

    println!("\n==============================");
    println!("Results: {} passed, {} failed, {} errors", passed, failed, errors);

    if failed > 0 || errors > 0 {
        std::process::exit(1);
    }
}
