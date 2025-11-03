use basic_emulator::interpreter::Interpreter;
use basic_emulator::screen::Screen;

fn test_group(name: &str, program: &str) {
    println!("\n=== Testing: {} ===", name);
    let screen = Screen::new();
    let mut interpreter = Interpreter::new(screen.clone());

    match interpreter.load_program(program) {
        Ok(_) => {
            let mut steps = 0;
            loop {
                if steps > 1000 {
                    println!("✗ FAIL: Too many steps (infinite loop?)");
                    return;
                }

                if interpreter.is_waiting_for_input() {
                    println!("✓ PASS: Reached INPUT (expected)");
                    return;
                }

                match interpreter.step() {
                    Ok(true) => steps += 1,
                    Ok(false) => {
                        println!("✓ PASS: Completed in {} steps", steps);
                        return;
                    }
                    Err(e) => {
                        println!("✗ FAIL: Runtime error: {}", e);
                        return;
                    }
                }
            }
        }
        Err(e) => {
            println!("✗ FAIL: Parse error: {}", e);
        }
    }
}

fn main() {
    println!("C64 BASIC KEYWORD COMPATIBILITY TEST");
    println!("====================================");

    // Group 1: Basic I/O
    test_group("PRINT - Simple", r#"
10 PRINT "HELLO"
20 END
"#);

    test_group("PRINT - Multiple items", r#"
10 PRINT "A";"B";"C"
20 END
"#);

    test_group("PRINT - With comma zones", r#"
10 PRINT "A","B","C"
20 END
"#);

    test_group("PRINT - With semicolon suppression", r#"
10 PRINT "A";
20 PRINT "B"
30 END
"#);

    test_group("PRINT - TAB function", r#"
10 PRINT TAB(10)"HELLO"
20 END
"#);

    test_group("INPUT - Simple", r#"
10 INPUT A$
20 END
"#);

    test_group("INPUT - With prompt", r#"
10 INPUT "NAME";N$
20 END
"#);

    // Group 2: Variables and Assignment
    test_group("LET - Explicit", r#"
10 LET A=5
20 END
"#);

    test_group("LET - Implicit", r#"
10 A=5
20 B$="HELLO"
30 END
"#);

    test_group("Variables - String concat", r#"
10 A$="HELLO"
20 B$=" WORLD"
30 C$=A$+B$
40 END
"#);

    // Group 3: Control Flow
    test_group("GOTO - Simple", r#"
10 GOTO 30
20 A=99
30 END
"#);

    test_group("IF-THEN - Line number", r#"
10 A=5
20 IF A=5 THEN 40
30 B=99
40 END
"#);

    test_group("IF-THEN - Statement", r#"
10 A=5
20 IF A=5 THEN B=10
30 END
"#);

    test_group("IF without THEN", r#"
10 A=5
20 IF A=5 GOTO 40
30 B=99
40 END
"#);

    test_group("GOSUB-RETURN", r#"
10 GOSUB 100
20 END
100 A=5
110 RETURN
"#);

    // Group 4: Loops
    test_group("FOR-NEXT - Simple", r#"
10 FOR I=1 TO 5
20 A=A+1
30 NEXT I
40 END
"#);

    test_group("FOR-NEXT - With STEP", r#"
10 FOR I=10 TO 1 STEP -1
20 A=A+1
30 NEXT I
40 END
"#);

    test_group("FOR-NEXT - Nested", r#"
10 FOR I=1 TO 3
20 FOR J=1 TO 2
30 A=A+1
40 NEXT J
50 NEXT I
60 END
"#);

    // Group 5: Arrays
    test_group("DIM - 1D array", r#"
10 DIM A(10)
20 A(5)=99
30 END
"#);

    test_group("DIM - 2D array", r#"
10 DIM A(5,5)
20 A(2,3)=99
30 END
"#);

    test_group("DIM - String array", r#"
10 DIM A$(10)
20 A$(5)="HELLO"
30 END
"#);

    // Group 6: Data Statements
    test_group("DATA-READ", r#"
10 DATA 1,2,3,4,5
20 READ A,B,C
30 END
"#);

    test_group("DATA-READ - Strings", r#"
10 DATA HELLO,WORLD,TEST
20 READ A$,B$,C$
30 END
"#);

    // Group 7: Functions
    test_group("INT function", r#"
10 A=INT(5.7)
20 END
"#);

    test_group("RND function", r#"
10 A=RND(1)
20 END
"#);

    test_group("CHR$ function", r#"
10 A$=CHR$(65)
20 END
"#);

    test_group("ASC function", r#"
10 A=ASC("A")
20 END
"#);

    test_group("VAL function", r#"
10 A=VAL("123")
20 END
"#);

    test_group("STR$ function", r#"
10 A$=STR$(123)
20 END
"#);

    test_group("MID$ function", r#"
10 A$=MID$("HELLO",2,3)
20 END
"#);

    test_group("LEN function", r#"
10 A=LEN("HELLO")
20 END
"#);

    test_group("LEFT$ function", r#"
10 A$=LEFT$("HELLO",2)
20 END
"#);

    test_group("RIGHT$ function", r#"
10 A$=RIGHT$("HELLO",2)
20 END
"#);

    // Group 8: Operators
    test_group("Arithmetic operators", r#"
10 A=1+2
20 B=5-3
30 C=2*3
40 D=10/2
50 E=2^3
60 END
"#);

    test_group("Comparison operators", r#"
10 IF 5=5 THEN A=1
20 IF 5<>6 THEN B=1
30 IF 3<5 THEN C=1
40 IF 5>3 THEN D=1
50 IF 5<=5 THEN E=1
60 IF 5>=5 THEN F=1
70 END
"#);

    test_group("Logical operators - AND", r#"
10 IF 1 AND 1 THEN A=1
20 END
"#);

    test_group("Logical operators - OR", r#"
10 IF 0 OR 1 THEN A=1
20 END
"#);

    test_group("Logical operators - NOT", r#"
10 IF NOT 0 THEN A=1
20 END
"#);

    // Group 9: Memory Operations
    test_group("POKE", r#"
10 POKE 53280,0
20 END
"#);

    // Group 10: Miscellaneous
    test_group("REM - Comments", r#"
10 REM THIS IS A COMMENT
20 A=5
30 END
"#);

    test_group("END statement", r#"
10 A=5
20 END
30 B=99
"#);

    test_group("Empty statements", r#"
10 A=5::B=10
20 END
"#);

    println!("\n====================================");
    println!("TEST SUITE COMPLETE");
}
