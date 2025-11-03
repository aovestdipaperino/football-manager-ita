use basic_emulator::interpreter::Interpreter;
use basic_emulator::screen::Screen;

fn main() {
    let source = r#"10 PRINT "HELLO WORLD"
20 FOR I=1 TO 5
30 PRINT "I=";I
40 NEXT I
50 PRINT "DONE"
60 END
"#;

    let mut screen = Screen::new();
    let mut interpreter = Interpreter::new(screen.clone());

    println!("Loading program...");
    if let Err(e) = interpreter.load_program(source) {
        eprintln!("Error loading program: {}", e);
        return;
    }

    println!("Running program...\n");
    let mut step_count = 0;
    let max_steps = 1000;

    loop {
        if step_count >= max_steps {
            eprintln!("\nProgram exceeded maximum steps ({})", max_steps);
            break;
        }

        match interpreter.step() {
            Ok(true) => {
                step_count += 1;
            }
            Ok(false) => {
                println!("\nProgram ended after {} steps", step_count);
                break;
            }
            Err(e) => {
                eprintln!("\nRuntime error at step {}: {}", step_count, e);
                break;
            }
        }
    }

    println!("\n--- Screen output ---");
    // We would need to add a method to get the screen content
    // For now, this just tests that the interpreter runs
}
