use basic_emulator::interpreter::Interpreter;
use basic_emulator::screen::Screen;
use std::fs;

fn main() {
    // Simple test to verify basic functionality works
    let simple_test = r#"
10 PRINT "STARTING TEST"
20 X=5
30 PRINT "X=";X
40 FOR I=1 TO 3
50 PRINT "I=";I
60 NEXT I
70 DIM A(5)
80 A(2)=99
90 PRINT "A(2)=";A(2)
100 GOSUB 200
110 PRINT "DONE"
120 END
200 PRINT "IN SUBROUTINE"
210 RETURN
"#;

    println!("=== Testing Basic Interpreter Functions ===\n");

    let screen = Screen::new();
    let mut interpreter = Interpreter::new(screen.clone());

    if let Err(e) = interpreter.load_program(simple_test) {
        eprintln!("Error loading program: {}", e);
        return;
    }

    let mut step_count = 0;
    loop {
        match interpreter.step() {
            Ok(true) => {
                step_count += 1;
                if step_count > 100 {
                    eprintln!("ERROR: Too many steps (possible infinite loop)");
                    return;
                }
            }
            Ok(false) => {
                println!("\n✓ Program completed successfully");
                println!("Total steps: {}", step_count);
                break;
            }
            Err(e) => {
                eprintln!("\n✗ Runtime error: {}", e);
                eprintln!("Step: {}", step_count);
                return;
            }
        }
    }

    // Now test with footballmanager.bas
    println!("\n=== Testing footballmanager.bas ===\n");

    let source = fs::read_to_string("/Users/enzo/Code/football-manager-ita/footballmanager.bas")
        .expect("Failed to read footballmanager.bas");

    let screen2 = Screen::new();
    let mut interpreter2 = Interpreter::new(screen2.clone());

    if let Err(e) = interpreter2.load_program(&source) {
        eprintln!("Error loading program: {}", e);
        return;
    }

    let mut step_count = 0;
    loop {
        if interpreter2.is_waiting_for_input() {
            println!("✓ Program reached INPUT statement");
            println!("Steps executed: {}", step_count);
            break;
        }

        match interpreter2.step() {
            Ok(true) => {
                step_count += 1;
                if step_count > 1000 {
                    eprintln!("ERROR: Too many steps");
                    return;
                }
            }
            Ok(false) => {
                println!("✓ Program ended");
                println!("Steps: {}", step_count);
                break;
            }
            Err(e) => {
                eprintln!("✗ Runtime error at step {}: {}", step_count, e);
                return;
            }
        }
    }

    println!("\n=== All Tests Passed ===");
}
